//! Parse error tests

// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other tests")]

// Imports
use std::{env, fs, path::Path};

#[test]
pub fn parse_error() {
	let _logger = zutil_logger::Logger::new();

	std::env::set_current_dir("..").expect("Unable to ascend a directory");
	let tests_dir = Path::new("tests/parse-error/");
	match env::var_os("RUSTIDY_PARSE_ERROR_UPDATE_TESTS") {
		Some(tests) => {
			let tests = tests
				.to_str()
				.expect("`RUSTIDY_PARSE_ERROR_UPDATE_TESTS` must be utf-8");
			for test_dir in tests.split(':') {
				self::test_case(Path::new(test_dir));
			}
		},
		None =>
			for test_dir in tests_dir.read_dir().expect("Unable to read tests directory") {
				let test_dir = test_dir.expect("Unable to read tests directory entry");
				let test_dir = test_dir.path();

				self::test_case(&test_dir);
			},
	}
}

/// Tests a case from a directory
fn test_case(test_dir: &Path) {
	let test_path = test_dir.join("input.rs");
	let input = fs::read_to_string(&test_path).expect("Unable to read file");

	let err = rustidy::parse(&input, &test_path).expect_err("Input did not fail");
	let err = err.pretty().to_string();

	let output_path = test_dir.join("output.txt");
	match env::var("RUSTIDY_PARSE_ERROR_UPDATE_OUTPUT").is_ok_and(|value| !value.trim().is_empty()) {
		true => {
			let err = err + "\n";
			fs::write(output_path, err).expect("Unable to update output");
		},
		false => {
			let output = fs::read_to_string(output_path).expect("Unable to read output path");
			let output = output.strip_suffix('\n').expect("Missing newline at the end of output");

			assert!(
				err == output,
				"Test {test_dir:?} output differed\n\nExpected:\n---\n{}\n---\n\nFound:\n---\n{}\n---",
				output.replace(' ', "·").replace('\t', "⭾").replace('\n', "␤\n"),
				err.replace(' ', "·").replace('\t', "⭾").replace('\n', "␤\n")
			);
		},
	}
}
