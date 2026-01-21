//! Parse error tests

// Imports
use {
	rustidy_parse::Parser,
	rustidy_util::Config,
	std::{env, fs, path::Path},
};

#[test]
pub fn parse_error() {
	std::env::set_current_dir("..").expect("Unable to ascend a directory");
	let tests_dir = Path::new("tests/parse-error/");
	for test_dir in tests_dir.read_dir().expect("Unable to read tests directory") {
		let test_dir = test_dir.expect("Unable to read tests directory entry");
		let test_dir = test_dir.path();

		let input_path = test_dir.join("input.rs");
		let file = fs::read_to_string(&input_path).expect("Unable to read file");
		let config = Config::default();
		let mut parser = Parser::new(&file, &config);

		let err = rustidy::parse(&input_path, &mut parser).expect_err("Input did not fail");
		let err = err.pretty().to_string();

		let output_path = test_dir.join("output.txt");
		match env::var("UPDATE_ERROR_OUTPUT").is_ok_and(|value| !value.trim().is_empty()) {
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
}
