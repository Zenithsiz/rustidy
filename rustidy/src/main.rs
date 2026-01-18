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
	try_trait_v2
)]

// Modules
mod args;

// Imports
use {
	self::args::Args,
	app_error::{AppError, Context},
	clap::Parser as _,
	rustidy::{Format, format},
	rustidy_parse::Parser,
	rustidy_print::{Print, PrintFmt},
	rustidy_util::Replacements,
	std::{fs, process::ExitCode},
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

	for file_path in &args.files {
		println!("{file_path:?}:");

		// Parse
		let file = fs::read_to_string(file_path).context("Unable to read file")?;
		let mut parser = Parser::new(&file);
		let mut crate_ = rustidy::parse(file_path, &mut parser).context("Unable to parse file")?;

		// Format
		let mut replacements = Replacements::new();
		let config = format::Config::default();
		let mut ctx = format::Context::new(&file, &mut replacements, &config);
		crate_.format(&mut ctx);

		// Then output it to file
		let mut print_fmt = PrintFmt::new(&file, &replacements);
		crate_.print(&mut print_fmt);
		fs::write(file_path, print_fmt.output()).context("Unable to write file")?;
	}

	Ok(())
}
