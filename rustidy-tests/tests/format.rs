//! Format tests

// Features
#![feature(yeet_expr)]

// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other tests")]

// Imports
use {
	app_error::{AppError, Context, ensure},
	rustidy_format::FormatOutput,
	rustidy_print::Print,
	std::{env, fs, path::Path},
};

#[test]
pub fn format() -> Result<(), AppError> {
	let _logger = zutil_logger::Logger::new();

	env::set_current_dir("..")
		.context("Unable to ascend a directory")?;
	let tests_dir = Path::new("tests/format/");
	match env::var_os("RUSTIDY_FORMAT_UPDATE_TESTS") {
		Some(tests) => {
			let tests = tests
				.to_str()
				.context("`RUSTIDY_FORMAT_UPDATE_TESTS` must be utf-8")?;
			for test_dir in tests.split(':') {
				self::test_case(Path::new(test_dir))
					.with_context(|| format!("Test {test_dir:?} failed"))?;
			}
		},
		None => for test_dir in tests_dir
			.read_dir()
			.context("Unable to read tests directory")? {
			let test_dir = test_dir
				.context("Unable to read tests directory entry")?;
			let test_dir = test_dir.path();

			self::test_case(&test_dir)
				.with_context(|| format!("Test {test_dir:?} failed"))?;
		},
	}

	Ok(())
}

/// Tests a case from a directory
fn test_case(test_dir: &Path) -> Result<(), AppError> {
	let test_path = test_dir.join("input.rs");
	let input = fs::read_to_string(&test_path)
		.context("Unable to read file")?;

	let mut crate_ = rustidy::parse(&input, &test_path)
		.context("Unable to parse input")?;

	let config = rustidy_util::Config::default();
	let _: FormatOutput = rustidy::format(&input, &config, &mut crate_);

	let found_output = crate_.print_to(Print::print);

	{
		let _: FormatOutput = rustidy::format(&input, &config, &mut crate_);

		let found_output2 = crate_.print_to(Print::print);

		ensure!(
			found_output.as_str() == found_output2.as_str(),
			"Formatting twice did not yield the same output:\n{}",
			difference::Changeset::new(found_output.as_str(), found_output2.as_str(), "\n")
		);
	}

	let output_path = test_dir.join("output.rs");
	match env::var("RUSTIDY_FORMAT_UPDATE_OUTPUT")
		.is_ok_and(|value| !value.trim().is_empty()) {
		true => {
			fs::write(output_path, found_output.as_str())
				.context("Unable to update output")?;
		},
		false => {
			let output = fs::read_to_string(&output_path)
				.context("Unable to read output path")?;

			ensure!(
				found_output.as_str() == output,
				"Output differed:\n{}",
				difference::Changeset::new(found_output.as_str(), &output, "\n")
			);
		},
	}

	Ok(())
}
