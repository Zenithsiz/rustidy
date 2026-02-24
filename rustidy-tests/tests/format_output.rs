//! Format output tests

// Features
#![feature(yeet_expr, decl_macro)]

// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other tests")]

// Imports
use {
	app_error::{AppError, Context, ensure},
	rustidy_ast::Crate,
	rustidy_ast_util::IdentifierOrKeyword,
	rustidy_format::{FormatOutput, Formattable},
	rustidy_parse::{Parse, ParseError, Parser},
	rustidy_util::Config,
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
			newlines: 0,
			is_empty: false,
			is_blank: false,
		},
		"  abc" as IdentifierOrKeyword: FormatOutput {
			prefix_ws_len: Some(2),
			len: 5,
			newlines: 0,
			is_empty: false,
			is_blank: false,
		},
		"//\nabc" as IdentifierOrKeyword: FormatOutput {
			prefix_ws_len: Some(3),
			len: 6,
			newlines: 1,
			is_empty: false,
			is_blank: false,
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
	let mut ctx = rustidy_format::Context::new(input, &config);
	let output = value.format_output(&mut ctx);
	ensure!(output == expected, "Format output was different\n{}", difference::Changeset::new(
		&format!("{expected:#?}"),
		&format!("{output:#?}"),
		"\n")
	);

	Ok(())
}
