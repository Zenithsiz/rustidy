//! Whitespace tests

// Features
#![feature(yeet_expr, anonymous_lifetime_in_impl_trait)]
// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other tests")]

use {
	app_error::{AppError, Context, ensure},
	rustidy_format::whitespace::{self, WhitespaceFormatKind},
	rustidy_parse::{ParseError, Parser},
	rustidy_print::{Print, PrintFmt},
	rustidy_util::Whitespace,
};

#[derive(Clone, Debug)]
struct Config {
	indent_depth: usize,
}

fn test_case_with(
	source: &str,
	expected: &str,
	fmt_config: &rustidy_util::Config,
	config: &Config,
	kind: WhitespaceFormatKind,
) -> Result<(), AppError> {
	let mut parser = Parser::new(source);
	let mut whitespace = parser
		.parse::<Whitespace>()
		.map_err(|err| err.to_app_error(&parser))
		.with_context(|| format!("Unable to parse whitespace: {source:?}"))?;
	ensure!(
		parser.is_finished(),
		"Parser didn't parse all the whitespace: {source:?}"
	);


	let mut fmt_ctx = rustidy_format::Context::new(source, fmt_config);
	fmt_ctx.set_indent_depth(config.indent_depth);
	whitespace::format(&mut whitespace, &mut fmt_ctx, kind);

	let mut print_fmt = PrintFmt::new(source);
	whitespace.print(&mut print_fmt);
	let output = print_fmt.output().to_owned();

	let source_fmt = source.replace(' ', "·").replace('\t', "⭾");
	let expected_fmt = expected.replace(' ', "·").replace('\t', "⭾");
	let output_fmt = output.replace(' ', "·").replace('\t', "⭾");

	ensure!(
		output == expected,
		"Found wrong output.\nKind    : {kind:?}\nInput   : {source_fmt:?}\nExpected: {expected_fmt:?}\nFound   : \
		 {output_fmt:?}"
	);

	{
		let mut fmt_ctx = rustidy_format::Context::new(source, fmt_config);
		fmt_ctx.set_indent_depth(config.indent_depth);
		whitespace::format(&mut whitespace, &mut fmt_ctx, kind);

		let mut print_fmt = PrintFmt::new(source);
		whitespace.print(&mut print_fmt);

		let output_fmt = print_fmt.output().replace(' ', "·").replace('\t', "⭾");

		app_error::ensure!(
			output == print_fmt.output(),
			"Formatting twice didn't preserve the formatting.\nKind    : {kind:?}\nInput   : {source_fmt:?}\nFirst   \
			 : {expected_fmt:?}\nSecond  : {output_fmt:?}",
		);
	}

	Ok(())
}

struct CaseKinds<'a> {
	source: &'a str,
	expected_remove: &'a str,
	expected_set_single: &'a str,
	expected_set_indent: &'a str,
	expected_set_prev_indent: &'a str,
	expected_set_prev_indent_or_remove: &'a str,
}

fn test_cases_with(
	cases: impl IntoIterator<Item = CaseKinds<'_>>,
	fmt_config: &rustidy_util::Config,
	config: &Config,
) -> Result<(), AppError> {
	cases
		.into_iter()
		.map(|case| {
			[
				(case.expected_remove, WhitespaceFormatKind::Remove),
				(case.expected_set_single, WhitespaceFormatKind::Spaces { len: 1 }),
				(case.expected_set_indent, WhitespaceFormatKind::Indent {
					offset:         0,
					remove_if_pure: false,
				}),
				(case.expected_set_prev_indent, WhitespaceFormatKind::Indent {
					offset:         -1,
					remove_if_pure: false,
				}),
				(case.expected_set_prev_indent_or_remove, WhitespaceFormatKind::Indent {
					offset:         -1,
					remove_if_pure: true,
				}),
			]
			.into_iter()
			.map(|(expected, kind)| {
				let mods = [("", ""), ("  ", ""), ("", "  "), ("  ", "  ")];
				mods.into_iter()
					.map(|(prefix, suffix)| {
						let source = format!("{prefix}{}{suffix}", case.source);
						test_case_with(&source, expected, fmt_config, config, kind)
					})
					.collect::<app_error::AllErrs<()>>()?;

				Ok(())
			})
			.collect::<app_error::AllErrs<()>>()?;

			Ok(())
		})
		.collect::<app_error::AllErrs<()>>()?;

	Ok(())
}

fn main() -> Result<(), AppError> {
	let _logger = zutil_logger::Logger::new();

	let cases = [
		CaseKinds {
			source: "",
			expected_remove: "",
			expected_set_single: " ",
			expected_set_indent: "\n\t\t",
			expected_set_prev_indent: "\n\t",
			expected_set_prev_indent_or_remove: "",
		},
		CaseKinds {
			source: "//a  \n",
			expected_remove: "//a  \n",
			expected_set_single: " //a  \n",
			expected_set_indent: "\n\t\t//a  \n\t\t",
			expected_set_prev_indent: "\n\t\t//a  \n\t",
			expected_set_prev_indent_or_remove: "\n\t\t//a  \n\t",
		},
		CaseKinds {
			source: "/*  a  */",
			expected_remove: "/*  a  */",
			expected_set_single: " /*  a  */ ",
			expected_set_indent: "\n\t\t/*  a  */\n\t\t",
			expected_set_prev_indent: "\n\t\t/*  a  */\n\t",
			expected_set_prev_indent_or_remove: "\n\t\t/*  a  */\n\t",
		},
		CaseKinds {
			source: "/*  a  */  /*  b  */",
			expected_remove: "/*  a  *//*  b  */",
			expected_set_single: " /*  a  */ /*  b  */ ",
			expected_set_indent: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t\t",
			expected_set_prev_indent: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t",
			expected_set_prev_indent_or_remove: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t",
		},
		CaseKinds {
			source: "/*  a  */  /*  b  */  /*  c  */",
			expected_remove: "/*  a  *//*  b  *//*  c  */",
			expected_set_single: " /*  a  */ /*  b  */ /*  c  */ ",
			expected_set_indent: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t\t/*  c  */\n\t\t",
			expected_set_prev_indent: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t\t/*  c  */\n\t",
			expected_set_prev_indent_or_remove: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t\t/*  c  */\n\t",
		},
		CaseKinds {
			source: "\n\n\n\n",
			expected_remove: "",
			expected_set_single: " ",
			expected_set_indent: "\n\n\n\t\t",
			expected_set_prev_indent: "\n\n\n\t",
			expected_set_prev_indent_or_remove: "",
		},
		CaseKinds {
			source: "//a\n\n\n\n//b\n",
			expected_remove: "//a\n//b\n",
			expected_set_single: " //a\n//b\n",
			expected_set_indent: "\n\t\t//a\n\n\n\t\t//b\n\t\t",
			expected_set_prev_indent: "\n\t\t//a\n\n\n\t\t//b\n\t",
			expected_set_prev_indent_or_remove: "\n\t\t//a\n\n\n\t\t//b\n\t",
		},
		CaseKinds {
			source: "//a\n//b\n",
			expected_remove: "//a\n//b\n",
			expected_set_single: " //a\n//b\n",
			expected_set_indent: "\n\t\t//a\n\t\t//b\n\t\t",
			expected_set_prev_indent: "\n\t\t//a\n\t\t//b\n\t",
			expected_set_prev_indent_or_remove: "\n\t\t//a\n\t\t//b\n\t",
		},
		CaseKinds {
			source: "/*a*/\n\n\n\n/*b*/",
			expected_remove: "/*a*//*b*/",
			expected_set_single: " /*a*/ /*b*/ ",
			expected_set_indent: "\n\t\t/*a*/\n\n\n\t\t/*b*/\n\t\t",
			expected_set_prev_indent: "\n\t\t/*a*/\n\n\n\t\t/*b*/\n\t",
			expected_set_prev_indent_or_remove: "\n\t\t/*a*/\n\n\n\t\t/*b*/\n\t",
		},
	];

	let fmt_config = rustidy_util::Config::default();
	let config = Config { indent_depth: 2 };
	self::test_cases_with(cases, &fmt_config, &config).map_err(AppError::flatten)
}
