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
	app_error::{AppError, Context, bail, ensure},
	clap::Parser as _,
	rustidy_ast::{attr::OuterAttrOrDocComment, item::{ItemInner, Module, VisItemInner}},
	rustidy_ast_util::Identifier,
	rustidy_format::FormatOutput,
	rustidy_print::Print,
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
			let cur_dir = std::env::current_dir()
				.context("Unable to get current directory")?;
			let mut cur_dir = cur_dir.as_path();
			loop {
				// TODO: Should we also allow `rustidy.toml`?
				let config_path = cur_dir.join(".rustidy.toml");
				if fs::exists(&config_path)
					.context("Unable to check if directory exists")? {
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
			let config = fs::read_to_string(config_path)
				.context("Unable to read configuration")?;
			toml::from_str(&config)
				.context("Unable to parse configuration")?
		},
		None => Config::default(),
	};
	tracing::debug!(?config, "Configuration");

	match args.files.is_empty() {
		true => self::format_file(&config, None, None, args.check)?,
		false => {
			let mut files = args.files;
			while let Some(file_path) = files.pop() {
				let start = Instant::now();
				self::format_file(
					&config,
					Some(&mut files),
					Some(&file_path),
					args.check
				)
					.with_context(|| format!("While formatting {file_path:?}"))?;
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
	check: bool
) -> Result<(), AppError> {
	// Parse
	let input = match file_path {
		Some(file_path) => fs::read_to_string(file_path)
			.context("Unable to read file")?,
		None => io::read_to_string(io::stdin())
			.context("Unable to read stdin")?,
	};
	let mut crate_ = rustidy::parse(
		&input,
		file_path
			.unwrap_or_else(|| Path::new("<stdin>"))
	)?;

	// Queue modules for formatting.
	if let Some(file_path) = file_path && let Some(files) = files {
		for item in &crate_.items.0 {
			// If it's not a module definition, skip it
			// TODO: Support modules inside of other modules (and other items).
			let ItemInner::Vis(vis_item) = &item.0.inner else {
				continue;
			};
			let VisItemInner::Module(mod_) = &vis_item.inner else {
				continue;
			};
			if mod_.inner.is_def() {
				continue;
			}

			// Then get it's path
			let mod_path = self::mod_path(file_path, &item.0.attrs, mod_)?;
			files.push(mod_path);
		}
	}

	// Format
	let _: FormatOutput = rustidy::format(&input, config, &mut crate_);

	let output = crate_.print_to(Print::print);
	match check {
		true => ensure!(input == output.as_str(), "File was not formatted"),
		false => {
			match file_path {
				Some(file_path) => if input != output.as_str() {
					fs::write(file_path, output.as_str())
						.context("Unable to write file")?;
				},
				None => io::stdout()
					.write_all(output.as_str().as_bytes())
					.context("Unable to write to stdout")?,
			}
		}
	}

	Ok(())
}

/// Returns a module's path
fn mod_path<'a>(
	file_path: &Path,
	attrs: impl IntoIterator<Item = &'a OuterAttrOrDocComment>,
	mod_: &Module,
) -> Result<PathBuf, AppError> {
	let path = match self::find_path_attr(attrs)? {
		// If it had a `#[path = ...]` attribute, use that
		Some(name) => file_path.with_file_name("").join(&*name),

		// Otherwise, use it's identifier
		None => {
			let name = match &mod_.ident {
				Identifier::Raw(ident) => {
					super let ident = ident.1.str();
					let ident = ident
						.strip_prefix("r#")
						.expect("Raw identified didn't start with `r#`");
					Cow::Borrowed(ident)
				},
				Identifier::NonKw(ident) => ident.0.1.str(),
			};

			// Try `<name>/mod.rs` first
			let mod_rs_path = file_path
				.parent()
				.expect("File had no parent")
				.join(&*name).join("mod.rs");
			match mod_rs_path
				.try_exists()
				.context("Unable to check if file exists")? {
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
fn find_path_attr<'a>(
	attrs: impl IntoIterator<Item = &'a OuterAttrOrDocComment>,
) -> Result<Option<Cow<'a, str>>, AppError> {
	for attr in attrs {
		let Some(attr) = attr.try_as_attr_ref() else {
			continue;
		};
		let Some(meta) = attr.open.value.try_as_meta_ref() else {
			continue;
		};
		if !(meta.path().is_str("path")) {
			continue;
		}
		let Some(meta) = meta.try_as_eq_expr_ref() else {
			bail!("Malformed `#[path = ...]` attribute");
		};
		// TODO: Support raw strings here
		let Some(literal) = meta.expr.as_string_literal() else {
			bail!("Expected a literal expression in `#[path = ...]` attribute");
		};

		return Ok(Some(literal.contents()));
	}

	Ok(None)
}
