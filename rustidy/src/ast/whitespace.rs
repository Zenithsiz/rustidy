//! Whitespace

// Imports
use {
	super::punct::Punctuated,
	crate::{
		Format,
		ParserStr,
		Print,
		Replacement,
		format,
		parser::{Parse, ParseError, ParserError},
	},
	itertools::Itertools,
};

/// Whitespace
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct Whitespace(Box<Punctuated<PureWhitespace, Comment>>);

impl Whitespace {
	/// Returns the suffix pure whitespace in this
	#[must_use]
	pub fn suffix_pure(&self) -> Option<&PureWhitespace> {
		match self.0.rest.last() {
			Some((_, pure)) => Some(pure),
			None => Some(&self.0.first),
		}
	}

	/// Removes this whitespace
	pub fn remove(&mut self, ctx: &mut format::Context) {
		self.format(ctx, FormatKind::Remove);
	}

	/// Sets this whitespace to a single space
	pub fn set_single(&mut self, ctx: &mut format::Context) {
		self.format(ctx, FormatKind::Single);
	}

	/// Sets this whitespace to a newline + indentation
	pub fn set_indent(&mut self, ctx: &mut format::Context, offset: isize, remove_if_empty: bool) {
		self.format(ctx, FormatKind::Indent {
			offset,
			remove_if_empty,
		});
	}

	fn format(&mut self, ctx: &mut format::Context, kind: FormatKind) {
		let (prefix, rest) = self.0.split_first_mut();

		let prefix_str = kind.prefix_str(ctx, prefix.0, rest.is_empty());
		ctx.replace(prefix.0, prefix_str);
		for (pos, (comment, ws)) in rest.with_position() {
			let is_last = matches!(pos, itertools::Position::Last | itertools::Position::Only);
			let ws_str = match comment.is_line() {
				true => kind.after_newline_str(ctx, ws.0, is_last),
				false => kind.normal_str(ctx, ws.0, is_last),
			};
			ctx.replace(ws.0, ws_str);
		}
	}
}

// TODO: Replace this impl once we add some `parse(try_with = ...)`
impl Parse for Whitespace {
	type Error = WhitespaceError;

	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	#[coverage(on)]
	fn parse_from(parser: &mut crate::parser::Parser) -> Result<Self, Self::Error> {
		if parser.has_tag("skip:Whitespace") {
			let s = parser.update_with(|_| ());
			let inner = Punctuated {
				first: PureWhitespace(s),
				rest:  vec![],
			};

			return Ok(Self(Box::new(inner)));
		}

		let inner = parser.parse().map_err(WhitespaceError::Inner)?;
		Ok(Self(inner))
	}
}
#[derive(ParseError, Debug)]
pub enum WhitespaceError {
	#[parse_error(transparent)]
	Inner(ParserError<Box<Punctuated<PureWhitespace, Comment>>>),
}

#[derive(Clone, Copy, Debug)]
#[derive(strum::EnumIs)]
enum FormatKind {
	Remove,
	Single,
	Indent {
		/// Indentation offset
		offset: isize,

		/// Remove if no comments exist
		remove_if_empty: bool,
	},
}

impl FormatKind {
	/// Returns the indentation string, without a newline
	fn indent_str(ctx: &format::Context) -> String {
		ctx.config().indent.repeat(ctx.indent())
	}

	/// Returns the indentation string, with a newline *before*
	fn indent_str_nl(ctx: &format::Context, cur_str: ParserStr) -> String {
		// TODO: Should we be checking for multiple newlines?
		let after_newline = ctx.parser().str_before(cur_str).ends_with('\n');

		let min_newlines = ctx.config().empty_line_spacing.min;
		let max_newlines = ctx.config().empty_line_spacing.max;
		let (min_newlines, max_newlines) = match after_newline {
			true => (min_newlines, max_newlines),
			false => (min_newlines + 1, max_newlines + 1),
		};
		let newlines = ctx.parser().str(cur_str).chars().filter(|&ch| ch == '\n').count();
		let newlines = newlines.clamp(min_newlines, max_newlines);

		"\n".repeat(newlines) + &Self::indent_str(ctx)
	}

	/// Returns the prefix string
	fn prefix_str(self, ctx: &mut format::Context, cur_str: ParserStr, is_last: bool) -> Replacement {
		match self {
			Self::Remove => "".into(),
			Self::Single => " ".into(),
			Self::Indent {
				offset,
				remove_if_empty,
			} => match remove_if_empty && is_last {
				true => "".into(),
				false => ctx
					.with_indent_offset_if(offset, is_last, |ctx| Self::indent_str_nl(ctx, cur_str))
					.into(),
			},
		}
	}

	/// Returns the string after a newline
	fn after_newline_str(self, ctx: &mut format::Context, cur_str: ParserStr, is_last: bool) -> Replacement {
		match self {
			Self::Remove | Self::Single => "".into(),
			Self::Indent { offset, .. } => match is_last {
				true => ctx.with_indent_offset(offset, |ctx| Self::indent_str(ctx)),
				false => Self::indent_str_nl(ctx, cur_str),
			}
			.into(),
		}
	}

	/// Returns the normal string
	fn normal_str(self, ctx: &mut format::Context, cur_str: ParserStr, is_last: bool) -> Replacement {
		match self {
			Self::Remove => "".into(),
			Self::Single => " ".into(),
			Self::Indent { offset, .. } => match is_last {
				true => ctx.with_indent_offset(offset, |ctx| Self::indent_str_nl(ctx, cur_str)),
				false => Self::indent_str_nl(ctx, cur_str),
			}
			.into(),
		}
	}
}

/// Pure whitespace
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PureWhitespace(
	#[parse(update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl PureWhitespace {
	fn parse(s: &mut &str) {
		*s = s.trim_start();
	}
}

/// Comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum Comment {
	Line(LineComment),
	Block(BlockComment),
}

/// Block Comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = NoComment, fmt = "Expected `/*`"))]
#[parse(error(name = MissingCommentEnd, fmt = "Expected `*/` after `/*`", fatal))]
pub struct BlockComment(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl BlockComment {
	fn parse(s: &mut &str) -> Result<(), BlockCommentError> {
		match s.starts_with("/*") {
			true => {
				*s = &s[2..];
				let mut depth = 1;
				while depth != 0 {
					let close_idx = s.find("*/").ok_or(BlockCommentError::MissingCommentEnd)?;

					match s[..close_idx].find("/*") {
						Some(open_idx) => {
							*s = &s[open_idx + 2..];
							depth += 1;
						},
						None => {
							*s = &s[close_idx + 2..];
							depth -= 1;
						},
					}
				}
				Ok(())
			},
			false => Err(BlockCommentError::NoComment),
		}
	}
}

/// Line comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = NoComment, fmt = "Expected `//` (except `///` or `//!`)"))]
#[parse(error(name = Newline, fmt = "Expected newline after `//`"))]
pub struct LineComment(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl LineComment {
	fn parse(s: &mut &str) -> Result<(), LineCommentError> {
		let is_doc_comment = (s.starts_with("///") && !s.starts_with("////")) || s.starts_with("//!");
		match s.starts_with("//") && !is_doc_comment {
			true => {
				let nl_idx = s.find('\n').ok_or(LineCommentError::Newline)?;
				*s = &s[nl_idx + 1..];
				Ok(())
			},
			false => Err(LineCommentError::NoComment),
		}
	}
}

/// Trailing line comment ([`LineComment`] without a newline)
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = NoComment, fmt = "Expected `//` (except `///` or `//!`)"))]
pub struct TrailingLineComment(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl TrailingLineComment {
	fn parse(s: &mut &str) -> Result<(), TrailingLineCommentError> {
		let is_doc_comment = (s.starts_with("///") && !s.starts_with("////")) || s.starts_with("//!");
		match s.starts_with("//") && !is_doc_comment {
			true => {
				*s = &s[s.len()..];
				Ok(())
			},
			false => Err(TrailingLineCommentError::NoComment),
		}
	}
}

#[cfg(test)]
mod tests {
	use {
		super::*,
		crate::{Parser, Replacements, print},
		app_error::{AppError, Context, ensure},
	};

	#[derive(Clone, Debug)]
	struct Config {
		indent_depth: usize,
	}

	fn test_case_with(
		source: &str,
		expected: &str,
		fmt_config: &format::Config,
		config: &Config,
		kind: FormatKind,
	) -> Result<(), AppError> {
		let mut parser_expected = Parser::new(expected);
		let whitespace_expected = parser_expected
			.parse::<Whitespace>()
			.map_err(|err| err.to_app_error(&parser_expected))
			.with_context(|| format!("Unable to parse expected whitespace: {expected:?}"))?;
		ensure!(
			parser_expected.is_finished(),
			"Parser didn't parse all the expected whitespace: {expected:?}"
		);

		let mut parser = Parser::new(source);
		let whitespace = parser
			.parse::<Whitespace>()
			.map_err(|err| err.to_app_error(&parser))
			.with_context(|| format!("Unable to parse whitespace: {source:?}"))?;
		ensure!(
			parser.is_finished(),
			"Parser didn't parse all the whitespace: {source:?}"
		);


		let mut replacements = Replacements::new();
		let mut fmt_ctx = format::Context::new(&parser, &mut replacements, fmt_config);
		fmt_ctx.set_indent_depth(config.indent_depth);
		let mut whitespace_output = whitespace.clone();
		whitespace_output.format(&mut fmt_ctx, kind);

		let mut print_fmt = print::PrintFmt::new(&parser, &replacements);
		whitespace_output.print(&mut print_fmt);
		let output = print_fmt.output();

		let whitespace_debug = |parser: &Parser, whitespace: &Whitespace| {
			whitespace
				.0
				.iter()
				.map(|comment| match comment {
					either::Either::Left(ws) => parser.str(ws.0).replace(' ', "·").replace('\t', "⭾"),
					either::Either::Right(comment) => match comment {
						Comment::Line(comment) => parser.str(comment.0).replace(' ', "·").replace('\t', "⭾"),
						Comment::Block(comment) => parser.str(comment.0).replace(' ', "·").replace('\t', "⭾"),
					},
				})
				.collect::<Vec<_>>()
		};

		let source = source.replace(' ', "·").replace('\t', "⭾");
		let expected = expected.replace(' ', "·").replace('\t', "⭾");
		let output = output.replace(' ', "·").replace('\t', "⭾");

		// TODO: We only have ascii in the tests, but this breaks on non-ascii,
		//       we should use a proper table to represent the errors.
		let source_debug_len = source.chars().map(|ch| ch.escape_debug().len()).sum::<usize>();
		let expected_debug_len = expected.chars().map(|ch| ch.escape_debug().len()).sum::<usize>();
		let output_debug_len = output.chars().map(|ch| ch.escape_debug().len()).sum::<usize>();
		let max_input_len = [source_debug_len, expected_debug_len, output_debug_len]
			.into_iter()
			.max()
			.expect("Exists");
		let source_debug_padding = " ".repeat(max_input_len - source_debug_len);
		let expected_debug_padding = " ".repeat(max_input_len - expected_debug_len);
		let output_debug_padding = " ".repeat(max_input_len - output_debug_len);

		ensure!(
			output == expected,
			"Found wrong output.\nKind    : {:?}\nInput   : {:?}{source_debug_padding} {:?}\nExpected: \
			 {:?}{expected_debug_padding} {:?}\nFound   : {:?}{output_debug_padding} {:?}",
			kind,
			source,
			whitespace_debug(&parser, &whitespace),
			expected,
			whitespace_debug(&parser_expected, &whitespace_expected),
			output,
			whitespace_debug(&parser, &whitespace_output),
		);

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
		fmt_config: &format::Config,
		config: &Config,
	) -> Result<(), AppError> {
		cases
			.into_iter()
			.map(|case| {
				[
					(case.expected_remove, FormatKind::Remove),
					(case.expected_set_single, FormatKind::Single),
					(case.expected_set_indent, FormatKind::Indent {
						offset:          0,
						remove_if_empty: false,
					}),
					(case.expected_set_prev_indent, FormatKind::Indent {
						offset:          -1,
						remove_if_empty: false,
					}),
					(case.expected_set_prev_indent_or_remove, FormatKind::Indent {
						offset:          -1,
						remove_if_empty: true,
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

	#[test]
	fn kinds() -> Result<(), AppError> {
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

		let fmt_config = format::Config::default();
		let config = Config { indent_depth: 2 };
		self::test_cases_with(cases, &fmt_config, &config).map_err(AppError::flatten)
	}
}
