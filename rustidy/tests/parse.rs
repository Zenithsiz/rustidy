//! Parsing tests

// Features
#![feature(exit_status_error)]

// Imports
use {
	assert_json_diff::assert_json_eq,
	rustidy::{Arenas, Parser, Print, Replacements, ast, print},
	std::{env, fs, path::Path, process, thread},
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
		let mut arenas = Arenas::new();
		let mut parser = Parser::new(&input_file, &mut arenas);

		let input = rustidy::parse(&input_path, &mut parser).expect("Unable to parse input");

		let replacements = Replacements::new();
		let mut print_fmt = print::PrintFmt::new(&input_file, &replacements, &mut arenas);
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
				// TODO: Don't spawn a new thread due to the stack being too small
				let output = thread::scope(|s| {
					thread::Builder::new()
						.stack_size(8 * 1024 * 1024)
						.spawn_scoped(s, || {
							serde_json::from_str::<ast::Crate>(&output).expect("Unable to deserialize output")
						})
						.expect("Unable to spawn thread")
						.join()
						.expect("Unable to join thread")
				});

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
