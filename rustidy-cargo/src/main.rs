//! Cargo integration for `rustidy`

// Features
#![feature(yeet_expr, exit_status_error)]

// Module
mod args;

// Imports
use {
	self::args::Args,
	app_error::{AppError, Context, app_error, ensure},
	clap::Parser,
	std::{
		collections::HashSet,
		env,
		ffi::{OsStr, OsString},
		io,
		path::{Path, PathBuf},
		process::{Command, ExitCode},
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
	let args::Command::Rustidy(args) = args.command;
	tracing::debug!(?args, "Arguments");

	// Set logger file from arguments
	logger.set_file(args.log_file.as_deref());

	let packages = args.packages.into_iter().collect::<HashSet<_>>();
	let manifest_path = match args.manifest_path {
		Some(manifest_path) => {
			let manifest_path = manifest_path
				.canonicalize()
				.context("Unable to canonicalize manifest path")?;
			ensure!(
				manifest_path.ends_with("Cargo.toml"),
				"The manifest-path must point to a `Cargo.toml` file"
			);
			Some(manifest_path)
		},
		None => None,
	};

	self::format(packages, manifest_path.as_deref(), &args.extra_args, args.check)
}

fn format(packages: HashSet<String>, manifest_path: Option<&Path>, extra_args: &[OsString], check: bool) -> Result<(), AppError> {
	// If we got no targets, error out
	let targets = self::get_targets(packages, manifest_path)?;
	ensure!(!targets.is_empty(), "No targets found for formatting");

	let rustidy = env::var_os("RUSTIDY");
	let rustidy = match &rustidy {
		Some(rustidy) => rustidy,
		None => OsStr::new("rustidy"),
	};

	let mut command = Command::new(rustidy);
	command.args(targets);
	command.args(extra_args);
	if check {
		command.arg("--check");
	}

	command
		.status()
		.map_err(|e| match e.kind() {
			io::ErrorKind::NotFound => app_error!("Unable to find `rustidy` binary {rustidy:?}, ensure it's in your `$PATH`"),
			_ => AppError::new(&e)
				.context("Unable to spawn rustidy"),
		})?
		.exit_ok()
		.context("rustidy returned an error")?;

	Ok(())
}

/// Based on the specified `CargoFmtStrategy`, returns a set of main source files.
fn get_targets(mut packages: HashSet<String>, manifest_path: Option<&Path>) -> Result<Vec<PathBuf>, AppError> {
	let mut targets = vec![];

	let metadata = {
		let mut cmd = cargo_metadata::MetadataCommand::new();
		cmd.no_deps();
		if let Some(manifest_path) = manifest_path {
			cmd.manifest_path(manifest_path);
		}
		cmd
			.exec()
			.context("Unable to get cargo metadata")
	}?;

	for package in metadata.packages {
		if !packages.is_empty() && !packages.remove(package.name.as_str()) {
			continue;
		}

		for target in package.targets {
			let target = target
				.src_path
				.canonicalize()
				.context("Unable to get target source path")?;
			targets.push(target);
		}
	}

	for package in packages {
		tracing::warn!("Package {package:?} wasn't found");
	}

	Ok(targets)
}
