//! Parsing tests

// Features
#![feature(exit_status_error, yeet_expr)]

// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other tests")]

// Imports
use {
	app_error::{AppError, Context, app_error, ensure},
	rustidy_print::Print,
	serde::Serialize,
	std::{env, fs, path::Path},
};

#[test]
pub fn parse() -> Result<(), AppError> {
	let _logger = zutil_logger::Logger::new();

	std::env::set_current_dir("..")
		.context("Unable to ascend a directory")?;
	let tests_dir = Path::new("tests/parse/");
	match env::var_os("RUSTIDY_PARSE_UPDATE_TESTS") {
		Some(tests) => {
			let tests = tests
				.to_str()
				.context("`RUSTIDY_PARSE_UPDATE_TESTS` must be utf-8")?;
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

	let crate_ = rustidy::parse(&input, &test_path)
		.context("Unable to parse input")?;

	let output = crate_.print_to(Print::print);
	ensure!(input == output.as_str(), "Crate output was not the same as input");

	let output_path = test_dir.join("output.json");
	match env::var("RUSTIDY_PARSE_UPDATE_OUTPUT")
		.is_ok_and(|value| !value.trim().is_empty()) {
		true => {
			let mut output = Vec::new();
			let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
			let mut serializer = serde_json::Serializer::with_formatter(&mut output, formatter);
			crate_
				.serialize(&mut serializer)
				.context("Unable to serialize input")?;

			fs::write(&output_path, &output)
				.context("Unable to update output")?;
		},
		false => {
			let output = fs::read_to_string(output_path)
				.context("Unable to read output path")?;
			let output = serde_json::from_str::<rustidy_ast::Crate>(&output)
				.context("Unable to deserialize output")?;

			assert_json_diff::assert_json_matches_no_panic(
				&crate_,
				&output,
				assert_json_diff::Config::new(assert_json_diff::CompareMode::Strict),
			)
				.map_err(
					|err| app_error!("Crate differed from expected:\n{err}")
				)?;
		},
	}

	Ok(())
}
