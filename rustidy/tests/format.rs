//! Format tests

// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other targets")]

// Imports
use {
	rustidy_format::Format,
	rustidy_parse::Parser,
	rustidy_print::{Print, PrintFmt},
	rustidy_util::AstPos,
	std::{env, fs, path::Path},
};

#[test]
pub fn format() {
	let _logger = zutil_logger::Logger::new();

	std::env::set_current_dir("..").expect("Unable to ascend a directory");
	let tests_dir = Path::new("tests/format/");
	for test_dir in tests_dir.read_dir().expect("Unable to read tests directory") {
		let test_dir = test_dir.expect("Unable to read tests directory entry");
		let test_dir = test_dir.path();

		self::test_case(&test_dir);
	}
}

/// Tests a case from a directory
fn test_case(test_dir: &Path) {
	let test_path = test_dir.join("input.rs");
	let input = fs::read_to_string(&test_path).expect("Unable to read file");

	let mut crate_ = rustidy::parse(&input, &test_path).expect("Input did not fail");

	let config = rustidy_util::Config::default();
	let mut ctx = rustidy_format::Context::new(&input, &config);
	crate_.format(&mut ctx);

	let mut print_fmt = PrintFmt::new(&input);
	crate_.print(&mut print_fmt);
	let found_output = print_fmt.output().to_owned();

	{
		let mut ctx = rustidy_format::Context::new(&input, &config);
		crate_.format(&mut ctx);

		let mut print_fmt = PrintFmt::new(&input);
		crate_.print(&mut print_fmt);

		assert_eq!(
			found_output,
			print_fmt.output(),
			"Formatting twice did not yield the same output"
		);
	}

	let output_path = test_dir.join("output.rs");
	match env::var("UPDATE_FORMAT_OUTPUT").is_ok_and(|value| !value.trim().is_empty()) {
		true => {
			fs::write(output_path, found_output).expect("Unable to update output");
		},
		false => {
			let output = fs::read_to_string(&output_path).expect("Unable to read output path");

			if let Some(idx) = found_output
				.char_indices()
				.zip(output.char_indices())
				.find_map(|((idx, lhs), (_, rhs))| (lhs != rhs).then_some(idx))
				.or_else(|| (found_output.len() != output.len()).then(|| usize::min(found_output.len(), output.len())))
			{
				let mut parser = Parser::new(&input);
				parser.set_pos(AstPos::from_usize(idx));
				parser.reverse_line();
				let idx = parser.cur_pos().to_usize();

				let len = output[idx..]
					.find('\n')
					.map_or_else(|| output[idx..].len(), |idx| idx + 1);
				let found_len = found_output[idx..]
					.find('\n')
					.map_or_else(|| found_output[idx..].len(), |idx| idx + 1);
				assert!(
					found_output == output,
					"Test {test_dir:?} differed at {}:{}\n\nExpected:\n---\n{}\n---\n\nFound:\n---\n{}\n---",
					output_path.display(),
					parser.cur_loc(),
					output[idx..][..len]
						.replace(' ', "·")
						.replace('\t', "⭾")
						.replace('\n', "␤"),
					found_output[idx..][..found_len]
						.replace(' ', "·")
						.replace('\t', "⭾")
						.replace('\n', "␤"),
				);
			}
		},
	}
}
