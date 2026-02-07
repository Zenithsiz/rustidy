//! Parsing tests

// Features
#![feature(exit_status_error)]
// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other targets")]

// Imports
use {
	assert_json_diff::assert_json_eq,
	rustidy_print::{Print, PrintFmt},
	serde::Serialize,
	std::{env, fs, path::Path},
};

#[test]
pub fn parse() {
	let _logger = zutil_logger::Logger::new();

	std::env::set_current_dir("..").expect("Unable to ascend a directory");
	let tests_dir = Path::new("tests/parse/");
	for test_dir in tests_dir.read_dir().expect("Unable to read tests directory") {
		let test_dir = test_dir.expect("Unable to read tests directory entry");
		let test_dir = test_dir.path();

		test_case(&test_dir);
	}
}

/// Tests a case from a directory
fn test_case(test_dir: &Path) {
	let test_path = test_dir.join("input.rs");
	let input = fs::read_to_string(&test_path).expect("Unable to read file");

	let crate_ = rustidy::parse(&input, &test_path).expect("Unable to parse input");

	let mut print_fmt = PrintFmt::new(&input);
	crate_.print(&mut print_fmt);
	assert_eq!(input, print_fmt.output(), "Crate output was not the same as input");

	let output_path = test_dir.join("output.json");
	match env::var("RUSTIDY_PARSE_UPDATE_OUTPUT").is_ok_and(|value| !value.trim().is_empty()) {
		true => {
			let mut output = Vec::new();
			let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
			let mut serializer = serde_json::Serializer::with_formatter(&mut output, formatter);
			crate_.serialize(&mut serializer).expect("Unable to serialize input");

			fs::write(&output_path, &output).expect("Unable to update output");
		},
		false => {
			let output = fs::read_to_string(output_path).expect("Unable to read output path");
			let output = serde_json::from_str::<rustidy_ast::Crate>(&output).expect("Unable to deserialize output");

			assert_json_eq!(crate_, output);
		},
	}
}
