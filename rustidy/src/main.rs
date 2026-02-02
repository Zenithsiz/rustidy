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
	try_blocks
)]

// Modules
mod args;

// Imports
use {
	self::args::Args,
	app_error::{AppError, Context, bail},
	clap::Parser as _,
	rustidy_format::Format,
	rustidy_parse::Parser,
	rustidy_print::{Print, PrintFmt},
	std::{fs, io, path::Path, process::ExitCode, time::Instant},
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
	let logger = {
		let default_filters = |default| [(None, default)];
		Logger::new(std::io::stderr, (), default_filters("info"), default_filters("debug"))
	};

	// Parse arguments
	let args = Args::parse();
	tracing::debug!(?args, "Arguments");

	// Set logger file from arguments
	logger.set_file(args.log_file.as_deref());

	let default_config_path = Path::new(".rustidy.toml");
	let config_path = args.config_file.as_deref().unwrap_or(&default_config_path);
	let config = match fs::read_to_string(config_path) {
		Ok(config) => toml::from_str(&config).context("Unable to parse configuration file")?,
		Err(err) if err.kind() == io::ErrorKind::NotFound => {
			tracing::info!("Config file wasn't found, creating a default configuration");
			let config = rustidy_util::Config::default();

			let res: Result<(), AppError> = try {
				let config = toml::to_string_pretty(&config).context("Unable to serialize configuration")?;
				fs::write(config_path, config).context("Unable to write configuration to file")?;
			};
			if let Err(err) = res {
				tracing::warn!("Unable to write configuration file: {err:?}")
			}

			config
		},
		Err(err) => bail!("Unable to read configuration file: {:?}", AppError::<()>::new(&err)),
	};

	for file_path in &args.files {
		let start = Instant::now();

		// Parse
		let file = fs::read_to_string(file_path).context("Unable to read file")?;
		let mut parser = Parser::new(&file, &config);
		let mut crate_ = rustidy::parse(file_path, &mut parser).context("Unable to parse file")?;

		// Format
		let mut ctx = rustidy_format::Context::new(&file, &config);
		crate_.format(&mut ctx);

		// Then output it to file
		let mut print_fmt = PrintFmt::new(&file, &config);
		crate_.print(&mut print_fmt);
		fs::write(file_path, print_fmt.output()).context("Unable to write file")?;

		let duration = start.elapsed();

		tracing::info!("{file_path:?}: {duration:.2?}");
	}

	Ok(())
}
