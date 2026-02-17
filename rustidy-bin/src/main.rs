//! Rust-tidy formatter

// Features
#![feature(
	never_type,
	decl_macro,
	macro_metavar_expr,
	macro_metavar_expr_concat,
	yeet_expr,
	pattern,
	unwrap_infallible,
	substr_range,
	try_trait_v2,
	try_blocks,
	super_let,
	if_let_guard,
	anonymous_lifetime_in_impl_trait
)]

// Modules
mod args;

// Imports
use {
	self::args::Args,
	app_error::{AppError, Context, bail},
	clap::Parser as _,
	rustidy_ast::{
		attr::{AttrInput, OuterAttrOrDocComment},
		expr::{ExpressionInner, without_block::ExpressionWithoutBlockInner},
		item::{ItemInner, Module, VisItemInner},
	},
	rustidy_ast_util::Identifier,
	rustidy_format::FormatOutput,
	rustidy_print::{Print, PrintFmt},
	rustidy_util::Config,
	std::{
		borrow::Cow,
		fs,
		io::{self, Write},
		path::{Path, PathBuf},
		process::ExitCode,
		time::Instant,
	},
	zutil_logger::Logger,
};

fn main() -> ExitCode {
	match self::run() {
		Ok(()) => ExitCode::SUCCESS,
		Err(err) => {
			tracing::error!("{}", err.pretty());
			ExitCode::FAILURE
		},
	}
}

fn run() -> Result<(), AppError> {
	// Initialize logging
	let logger = Logger::new();

	// Parse arguments
	let args = Args::parse();
	tracing::debug!(?args, "Arguments");

	// Set logger file from arguments
	logger.set_file(args.log_file.as_deref());

	let config_path = match args.config_file {
		Some(config_path) => Some(config_path),
		None => {
			let cur_dir = std::env::current_dir().context("Unable to get current directory")?;
			let mut cur_dir = cur_dir.as_path();
			loop {
				// TODO: Should we also allow `rustidy.toml`?
				let config_path = cur_dir.join(".rustidy.toml");
				if fs::exists(&config_path).context("Unable to check if directory exists")? {
					break Some(config_path);
				}

				match cur_dir.parent() {
					Some(parent) => cur_dir = parent,
					None => break None,
				}
			}
		},
	};
	let config = match config_path {
		Some(config_path) => {
			let config = fs::read_to_string(config_path).context("Unable to read configuration")?;
			toml::from_str(&config).context("Unable to parse configuration")?
		},
		None => Config::default(),
	};
	tracing::debug!(?config, "Configuration");

	match args.files.is_empty() {
		true => self::format_file(&config, None, None)?,
		false => {
			let mut files = args.files;
			while let Some(file_path) = files.pop() {
				let start = Instant::now();
				self::format_file(&config, Some(&mut files), Some(&file_path))?;
				let duration = start.elapsed();

				tracing::info!("{file_path:?}: {duration:.2?}");
			}
		},
	}


	Ok(())
}

fn format_file(
	config: &rustidy_util::Config,
	files: Option<&mut Vec<PathBuf>>,
	file_path: Option<&Path>,
) -> Result<(), AppError> {
	// Parse
	let input = match file_path {
		Some(file_path) => fs::read_to_string(file_path).context("Unable to read file")?,
		None => io::read_to_string(io::stdin()).context("Unable to read stdin")?,
	};
	let mut crate_ = rustidy::parse(&input, file_path.unwrap_or_else(|| Path::new("<stdin>")))?;

	// Queue modules for formatting.
	if let Some(file_path) = file_path &&
		let Some(files) = files
	{
		for item in &crate_.0.items.0 {
			// If it's not a module definition, skip it
			// TODO: Support modules inside of other modules (and other items).
			let item = item.0.get();
			let ItemInner::Vis(vis_item) = &item.inner else {
				continue;
			};
			let VisItemInner::Module(mod_) = &vis_item.inner else {
				continue;
			};
			if mod_.inner.is_def() {
				continue;
			}

			// Then get it's path
			let mod_path = self::mod_path(file_path, &input, &item.attrs, mod_)?;
			files.push(mod_path);
		}
	}

	// Format
	let _: FormatOutput = rustidy::format(&input, config, &mut crate_);

	// Then output it to file
	let mut print_fmt = PrintFmt::new(&input);
	crate_.print(&mut print_fmt);
	match file_path {
		Some(file_path) => fs::write(file_path, print_fmt.output()).context("Unable to write file")?,
		None => io::stdout()
			.write_all(print_fmt.output().as_bytes())
			.context("Unable to write to stdout")?,
	}


	Ok(())
}

/// Returns a module's path
fn mod_path(
	file_path: &Path,
	input: &str,
	attrs: impl IntoIterator<Item = &OuterAttrOrDocComment>,
	mod_: &Module,
) -> Result<PathBuf, AppError> {
	let path = match self::find_path_attr(input, attrs)? {
		// If it had a `#[path = ...]` attribute, use that
		Some(name) => file_path.with_file_name("").join(&*name),

		// Otherwise, use it's identifier
		None => {
			let name = match &mod_.ident {
				Identifier::Raw(ident) => {
					super let ident = ident.1.str(input);
					let ident = ident.strip_prefix("r#").expect("Raw identified didn't start with `r#`");
					Cow::Borrowed(ident)
				},
				Identifier::NonKw(ident) => ident.0.1.str(input),
			};

			// Try `<name>/mod.rs` first
			let mod_rs_path = file_path
				.parent()
				.expect("File had no parent")
				.join(&*name)
				.join("mod.rs");
			match mod_rs_path.try_exists().context("Unable to check if file exists")? {
				true => mod_rs_path,
				// If it fails, try the new module system
				false => {
					// TODO: Is this the proper way to check for the root?
					let is_root = ["lib.rs", "main.rs", "mod.rs"]
						.iter()
						.any(|file| file_path.ends_with(file));
					let mod_file_name = format!("{name}.rs");
					match is_root {
						true => file_path.with_file_name(mod_file_name),
						false => file_path.with_extension("").join(mod_file_name),
					}
				},
			}
		},
	};

	Ok(path)
}

/// Finds a `#[path = ...]` attribute
// TODO: Support `cfg_attr(..., path = ...)` and others?
fn find_path_attr<'input>(
	input: &'input str,
	attrs: impl IntoIterator<Item = &OuterAttrOrDocComment>,
) -> Result<Option<Cow<'input, str>>, AppError> {
	for attr in attrs {
		let Some(attr) = attr.try_as_attr_ref() else { continue };
		if !(attr.open.value.path.is_str(input, "path")) {
			continue;
		}
		let expr = match &attr.open.value.input {
			Some(AttrInput::EqExpr(eq_expr)) => &eq_expr.expr,
			_ => bail!("Malformed `#[path = ...]` attribute"),
		};
		let expr = expr.0.get();
		let literal = match &*expr {
			ExpressionInner::WithoutBlock(expr)
				if let ExpressionWithoutBlockInner::Literal(literal) = &expr.0.inner =>
				literal,
			_ => bail!("Expected a literal expression in `#[path = ...]` attribute"),
		};
		let name = match literal {
			// Note: The rust compiler doesn't support c-strings or byte-strings here, only regular and raw strings,
			//       so we also don't.
			rustidy_ast_literal::LiteralExpression::String(s) => s.contents(input),
			// TODO: Allow raw strings here
			rustidy_ast_literal::LiteralExpression::RawString(_) =>
				todo!("Raw strings in `#[path = ...]` attributes aren't currently supported"),
			_ => bail!("Expected a string literal in `#[path = ...]` attribute"),
		};

		return Ok(Some(name));
	}

	Ok(None)
}
