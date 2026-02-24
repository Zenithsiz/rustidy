//! Arguments

// Imports
use std::{ffi::OsString, path::PathBuf};

/// Rustidy formatter
#[derive(Debug)]
#[derive(clap::Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "cargo")]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub struct Args {
	#[clap(subcommand)]
	pub command: Command,
}

#[derive(Debug)]
#[derive(clap::Subcommand)]
pub enum Command {
	Rustidy(Rustidy),
}

#[derive(Debug)]
#[derive(clap::Args)]
#[clap(author, version, about)]
pub struct Rustidy {
	/// Logs output to a file.
	///
	/// You can use `RUST_FILE_LOG` to set filtering options
	#[clap(long = "log-file")]
	pub log_file:      Option<PathBuf>,

	/// Specify package to format
	#[arg(short = 'p', long = "package")]
	pub packages:      Vec<String>,

	/// Specify path to Cargo.toml
	#[arg(long = "manifest-path")]
	pub manifest_path: Option<PathBuf>,

	/// Run rustidy with `--check`
	#[arg(long = "check")]
	pub check:         bool,

	/// Arguments to pass to `rustidy`
	pub extra_args:    Vec<OsString>,
}
