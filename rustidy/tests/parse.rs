//! Parsing tests

// Features
#![feature(exit_status_error)]

// Imports
use {
	assert_json_diff::assert_json_eq,
	rustidy::ast,
	rustidy_parse::Parser,
	rustidy_print::{Print, PrintFmt},
	rustidy_util::Replacements,
	serde::Deserialize,
	std::{env, fs, path::Path, process},
};

#[test]
pub fn parse() {
	let mut prettier_procs = vec![];

	std::env::set_current_dir("..").expect("Unable to ascend a directory");
	let tests_dir = Path::new("tests/parse/");
	for test_dir in tests_dir.read_dir().expect("Unable to read tests directory") {
		let test_dir = test_dir.expect("Unable to read tests directory entry");
		let test_dir = test_dir.path();

		let input_path = test_dir.join("input.rs");
		let input_file = fs::read_to_string(&input_path).expect("Unable to read file");
		let mut parser = Parser::new(&input_file);

		let input = rustidy::parse(&input_path, &mut parser).expect("Unable to parse input");

		let replacements = Replacements::new();
		let mut print_fmt = PrintFmt::new(&input_file, &replacements);
		input.print(&mut print_fmt);
		let input_printed = print_fmt.output();
		assert_eq!(input_file, input_printed);

		let output_path = test_dir.join("output.json");
		match env::var("UPDATE_AST_OUTPUT").is_ok_and(|value| !value.trim().is_empty()) {
			true => {
				let output = serde_json::to_string(&input).expect("Unable to serialize input");
				fs::write(&output_path, &output).expect("Unable to update output");

				// TODO: Better solution than shelling out to `prettier`?
				let cmd = process::Command::new("prettier")
					.arg("-w")
					.arg(&output_path)
					.stdout(process::Stdio::null())
					.spawn()
					.expect("Unable to spawn `prettier`");
				prettier_procs.push(cmd);
			},
			false => {
				let output = fs::read_to_string(output_path).expect("Unable to read output path");
				let output = {
					let mut deserializer = serde_json::Deserializer::from_str(&output);
					deserializer.disable_recursion_limit();
					let mut deserializer = serde_stacker::Deserializer::new(&mut deserializer);
					deserializer.red_zone = 512 * 1024;
					deserializer.stack_size = 8 * 1024 * 1024;
					ast::Crate::deserialize(deserializer).expect("Unable to deserialize output")
				};

				assert_json_eq!(input, output);
			},
		}
	}

	for mut cmd in prettier_procs {
		cmd.wait()
			.expect("`prettier` was killed")
			.exit_ok()
			.expect("`prettier` failed");
	}
}
