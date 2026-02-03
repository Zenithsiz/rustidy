//! Parsing tests

// Features
#![feature(exit_status_error)]

// Imports
use {
	assert_json_diff::assert_json_eq,
	rustidy_parse::Parser,
	rustidy_print::{Print, PrintFmt},
	rustidy_util::Config,
	serde::{Deserialize, Serialize},
	std::{env, fs, path::Path},
};

#[test]
pub fn parse() {
	std::env::set_current_dir("..").expect("Unable to ascend a directory");
	let tests_dir = Path::new("tests/parse/");
	for test_dir in tests_dir.read_dir().expect("Unable to read tests directory") {
		let test_dir = test_dir.expect("Unable to read tests directory entry");
		let test_dir = test_dir.path();

		let input_path = test_dir.join("input.rs");
		let input_file = fs::read_to_string(&input_path).expect("Unable to read file");
		let config = Config::default();
		let mut parser = Parser::new(&input_file, &config);

		let input = rustidy::parse(&input_path, &mut parser).expect("Unable to parse input");

		let mut print_fmt = PrintFmt::new(&input_file, &config);
		input.print(&mut print_fmt);
		let input_printed = print_fmt.output();
		assert_eq!(input_file, input_printed);

		let output_path = test_dir.join("output.json");
		match env::var("UPDATE_AST_OUTPUT").is_ok_and(|value| !value.trim().is_empty()) {
			true => {
				let mut output = Vec::new();
				let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");
				let mut serializer = serde_json::Serializer::with_formatter(&mut output, formatter);
				input.serialize(&mut serializer).expect("Unable to serialize input");

				fs::write(&output_path, &output).expect("Unable to update output");
			},
			false => {
				let output = fs::read_to_string(output_path).expect("Unable to read output path");
				let output = {
					let mut deserializer = serde_json::Deserializer::from_str(&output);
					deserializer.disable_recursion_limit();
					let mut deserializer = serde_stacker::Deserializer::new(&mut deserializer);
					deserializer.red_zone = 512 * 1024;
					deserializer.stack_size = 8 * 1024 * 1024;
					rustidy_ast::Crate::deserialize(deserializer).expect("Unable to deserialize output")
				};

				assert_json_eq!(input, output);
			},
		}
	}
}
