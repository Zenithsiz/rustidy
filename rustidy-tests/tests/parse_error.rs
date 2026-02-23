//! Parse error tests

// Features
#![feature(yeet_expr)]

// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other tests")]

// Imports
use {app_error::{AppError, Context, ensure}, std::{env, fs, path::Path}};

#[test]
pub fn parse_error() -> Result<(), AppError> {
	let _logger = zutil_logger::Logger::new();

	std::env::set_current_dir("..")
		.context("Unable to ascend a directory")?;
	let tests_dir = Path::new("tests/parse-error/");
	match env::var_os("RUSTIDY_PARSE_ERROR_UPDATE_TESTS") {
		Some(tests) => {
			let tests = tests
				.to_str()
				.context("`RUSTIDY_PARSE_ERROR_UPDATE_TESTS` must be utf-8")?;
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

	let err = rustidy::parse(&input, &test_path)
		.expect_err("Input did not fail");
	let err = err.pretty().to_string();

	let output_path = test_dir.join("output.txt");
	match env::var("RUSTIDY_PARSE_ERROR_UPDATE_OUTPUT")
		.is_ok_and(|value| !value.trim().is_empty()) {
		true => {
			let err = err + "\n";
			fs::write(output_path, err)
				.context("Unable to update output")?;
		},
		false => {
			let output = fs::read_to_string(output_path)
				.context("Unable to read output path")?;
			let output = output
				.strip_suffix('\n')
				.context("Missing newline at the end of output")?;

			ensure!(
				err == output,
				"Output differed:\n{}",
				difference::Changeset::new(&err, output, "\n")
			);
		},
	}

	Ok(())
}
