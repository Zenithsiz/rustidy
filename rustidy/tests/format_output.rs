//! Format output tests

// Features
#![feature(yeet_expr, decl_macro)]

// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other crates in this package")]

// Imports
use {
	app_error::{AppError, Context, ensure},
	ast::Crate,
	ast_literal::IdentifierOrKeyword,
	format::{FormatMultilineOutput, FormatOutput, Formattable},
	parse::{Parse, ParseError, Parser},
	util::Config,
};

#[test]
pub fn format_output() -> Result<(), AppError> {
	let _logger = zutil_logger::Logger::new();

	macro test_cases(
		$($input:literal as $T:ty: $expected:expr),* $(,)?
	) {
		$(
			self::test_case::<$T>($input, $expected)
				.with_context(|| format!("Test case {:?} failed", $input))?;
		)*
	}

	test_cases! {
		"" as Crate: FormatOutput {
			prefix_ws_len: Some(0),
			..FormatOutput::default()
		},
		"abc" as IdentifierOrKeyword: FormatOutput {
			prefix_ws_len: Some(0),
			len: 3,
			is_empty: false,
			is_blank: false,
			multiline: None,
		},
		"  abc" as IdentifierOrKeyword: FormatOutput {
			prefix_ws_len: Some(2),
			len: 5,
			is_empty: false,
			is_blank: false,
			multiline: None,
		},
		"//\nabc" as IdentifierOrKeyword: FormatOutput {
			prefix_ws_len: Some(3),
			len: 6,
			is_empty: false,
			is_blank: false,
			multiline: Some(FormatMultilineOutput { prefix_ws_len: Some(2), prefix_len: 2, suffix_len: 3 }),
		},
		"//\n//\nabc" as IdentifierOrKeyword: FormatOutput {
			prefix_ws_len: Some(6),
			len: 9,
			is_empty: false,
			is_blank: false,
			multiline: Some(FormatMultilineOutput { prefix_ws_len: Some(2), prefix_len: 2, suffix_len: 3 }),
		},
	}

	Ok(())
}

pub fn test_case<T: Parse + Formattable>(input: &str, expected: FormatOutput) -> Result<(), AppError> {
	let mut parser = Parser::new(input);
	let mut value = parser
		.parse::<T>()
		.map_err(|err| err.to_app_error(&parser))
		.context("Unable to parse")?;
	ensure!(parser.is_finished(), "Parser didn't parse all of input");


	let config = Config::default();
	let mut ctx = format::Context::new(input, &config);
	let output = value.format_output(&mut ctx);
	ensure!(output == expected, "Format output was different\n{}", difference::Changeset::new(
		&format!("{expected:#?}"),
		&format!("{output:#?}"),
		"\n")
	);

	Ok(())
}

#[test]
pub fn format_multiline_output_from_str() -> Result<(), AppError> {
	let cases = [
		("", None),
		("abc", None),
		("01234\n567", Some(FormatMultilineOutput {
			prefix_ws_len: None,
			prefix_len: 5,
			suffix_len: 3,
		})),
	];

	for (input, expected) in cases {
		let output = FormatMultilineOutput::from_str(input);
		ensure!(output == expected, "Format output was different for input {input:?}\n{}", difference::Changeset::new(
			&format!("{expected:#?}"),
			&format!("{output:#?}"),
			"\n")
		);
	}

	Ok(())
}
