//! Whitespace

// Imports
use {
	super::punct::Punctuated,
	crate::{
		AstStr,
		Format,
		format,
		parser::{Parse, ParseError, Parser},
		print::Print,
	},
	itertools::Itertools,
	std::{borrow::Cow, fmt},
};

/// Whitespace
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Whitespace(Box<Punctuated<PureWhitespace, Comment>>);

impl Whitespace {
	/// Returns the length of this whitespace
	#[must_use]
	pub fn len(&self) -> usize {
		self.0
			.iter()
			.map(|value| match value {
				either::Either::Left(ws) => ws.0.len(),
				either::Either::Right(comment) => match comment {
					Comment::Line(comment) => comment.0.len(),
					Comment::Block(comment) => comment.0.len(),
				},
			})
			.sum()
	}

	/// Returns if this whitespace is empty
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Returns the suffix pure whitespace in this
	#[must_use]
	pub fn suffix_pure(&self) -> &PureWhitespace {
		match self.0.rest.last() {
			Some((_, pure)) => pure,
			None => &self.0.first,
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
	pub fn set_indent(&mut self, ctx: &mut format::Context) {
		self.format(ctx, FormatKind::Indent);
	}

	/// Sets this whitespace to the previous indentation.
	///
	/// Any comments before that will be at the current indentation
	pub fn set_prev_indent(&mut self, ctx: &mut format::Context) {
		self.format(ctx, FormatKind::PrevIndent);
	}

	/// Sets this whitespace to a newline + previous indentation if any comments exists, otherwise removes
	pub fn set_prev_indent_or_remove(&mut self, ctx: &mut format::Context) {
		self.format(ctx, FormatKind::PrevIndentOrRemove);
	}

	fn format(&mut self, ctx: &mut format::Context, kind: FormatKind) {
		let (prefix, rest) = self.0.split_first_mut();

		prefix.0.replace(kind.prefix_str(ctx, rest.is_empty()));
		for (pos, (comment, ws)) in rest.with_position() {
			let is_last = matches!(pos, itertools::Position::Last | itertools::Position::Only);
			match comment.is_line() {
				true => ws.0.replace(kind.after_newline_str(ctx, is_last)),
				false => ws.0.replace(kind.normal_str(ctx, is_last)),
			}
		}
	}
}

#[derive(Clone, Copy, Debug)]
#[derive(strum::EnumIs)]
enum FormatKind {
	Remove,
	Single,
	Indent,
	PrevIndent,
	PrevIndentOrRemove,
}

impl FormatKind {
	/// Returns the indentation string, without a newline
	fn indent_str(ctx: &format::Context) -> String {
		ctx.config().indent.repeat(ctx.indent())
	}

	/// Returns the indentation string, with a newline *before*
	fn indent_str_nl(ctx: &format::Context) -> String {
		"\n".to_owned() + &Self::indent_str(ctx)
	}

	/// Returns the prefix string
	fn prefix_str(self, ctx: &mut format::Context, is_last: bool) -> Cow<'static, str> {
		match self {
			Self::Remove => "".into(),
			Self::Single => " ".into(),
			Self::Indent => Self::indent_str_nl(ctx).into(),
			Self::PrevIndent => ctx.without_indent_if(is_last, |ctx| Self::indent_str_nl(ctx)).into(),
			Self::PrevIndentOrRemove => match is_last {
				true => "".into(),
				false => Self::indent_str_nl(ctx).into(),
			},
		}
	}

	/// Returns the string after a newline
	fn after_newline_str(self, ctx: &mut format::Context, is_last: bool) -> Cow<'static, str> {
		match self {
			Self::Remove | Self::Single => "".into(),
			Self::Indent => Self::indent_str(ctx).into(),
			Self::PrevIndent | Self::PrevIndentOrRemove =>
				ctx.without_indent_if(is_last, |ctx| Self::indent_str(ctx)).into(),
		}
	}

	/// Returns the normal string
	fn normal_str(self, ctx: &mut format::Context, is_last: bool) -> Cow<'static, str> {
		match self {
			Self::Remove => "".into(),
			Self::Single => " ".into(),
			Self::Indent => Self::indent_str_nl(ctx).into(),
			Self::PrevIndent | Self::PrevIndentOrRemove =>
				ctx.without_indent_if(is_last, |ctx| Self::indent_str_nl(ctx)).into(),
		}
	}
}

/// Pure whitespace
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct PureWhitespace(#[format(str)] pub AstStr);

impl Parse for PureWhitespace {
	type Error = !;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser
			.try_update_with(|s| {
				*s = s.trim_start();
				Ok(())
			})
			.map(Self)
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
#[derive(Format, Print)]
pub struct BlockComment(#[format(str)] pub AstStr);

/// Comment error
#[derive(Debug, ParseError)]
pub enum BlockCommentError {
	#[parse_error(fmt = "Expected `/*`")]
	NoComment,

	#[parse_error(fmt = "Expected `*/` after `/*`")]
	#[parse_error(fatal)]
	MissingCommentEnd,
}

impl Parse for BlockComment {
	type Error = BlockCommentError;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser
			.try_update_with(|s| match s.starts_with("/*") {
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
			})
			.map(Self)
	}
}

/// Line comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct LineComment(#[format(str)] pub AstStr);

/// Comment error
#[derive(Debug, ParseError)]
pub enum LineCommentError {
	#[parse_error(fmt = "Expected `//` (except `///` or `//!`)")]
	NoComment,

	#[parse_error(fmt = "Expected newline after `//`")]
	Newline,
}

impl Parse for LineComment {
	type Error = LineCommentError;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser
			.try_update_with(|s| {
				let is_doc_comment = (s.starts_with("///") && !s.starts_with("////")) || s.starts_with("//!");
				match s.starts_with("//") && !is_doc_comment {
					true => {
						let nl_idx = s.find('\n').ok_or(LineCommentError::Newline)?;
						*s = &s[nl_idx + 1..];
						Ok(())
					},
					false => Err(LineCommentError::NoComment),
				}
			})
			.map(Self)
	}
}

/// Trailing line comment ([`LineComment`] without a newline)
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct TrailingLineComment(#[format(str)] pub AstStr);

/// Trailing line comment error
#[derive(Debug, ParseError)]
pub enum TrailingLineCommentError {
	#[parse_error(fmt = "Expected `//` (except `///` or `//!`)")]
	NoComment,
}

impl Parse for TrailingLineComment {
	type Error = TrailingLineCommentError;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser
			.try_update_with(|s| {
				let is_doc_comment = (s.starts_with("///") && !s.starts_with("////")) || s.starts_with("//!");
				match s.starts_with("//") && !is_doc_comment {
					true => {
						*s = &s[s.len()..];
						Ok(())
					},
					false => Err(TrailingLineCommentError::NoComment),
				}
			})
			.map(Self)
	}
}


#[cfg(test)]
mod tests {
	use {
		super::*,
		crate::print,
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


		let mut fmt_ctx = format::Context::new(&parser, fmt_config);
		fmt_ctx.set_indent_depth(config.indent_depth);
		let mut whitespace_output = whitespace.clone();
		whitespace_output.format(&mut fmt_ctx, kind);

		let mut output = String::new();
		let mut print_fmt = print::PrintFmt::new(&parser, &mut output);
		whitespace_output
			.print(&mut print_fmt)
			.context("Unable to print output")?;

		let whitespace_debug = |parser: &Parser, whitespace: &Whitespace| {
			whitespace
				.0
				.iter()
				.map(|comment| match comment {
					either::Either::Left(ws) => ws.0.as_str(parser).replace(' ', "·"),
					either::Either::Right(comment) => match comment {
						Comment::Line(comment) => comment.0.as_str(parser).replace(' ', "·"),
						Comment::Block(comment) => comment.0.as_str(parser).replace(' ', "·"),
					},
				})
				.collect::<Vec<_>>()
		};

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
			source.replace(' ', "·"),
			whitespace_debug(&parser, &whitespace),
			expected.replace(' ', "·"),
			whitespace_debug(&parser_expected, &whitespace_expected),
			output.replace(' ', "·"),
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
		test_prefix: bool,
		test_suffix: bool,
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
					(case.expected_set_indent, FormatKind::Indent),
					(case.expected_set_prev_indent, FormatKind::PrevIndent),
					(case.expected_set_prev_indent_or_remove, FormatKind::PrevIndentOrRemove),
				]
				.into_iter()
				.map(|(expected, kind)| {
					let mut mods = vec![("", "")];
					if case.test_prefix {
						mods.push(("  ", ""));
					}
					if case.test_suffix {
						mods.push(("", "  "));
					}
					if case.test_prefix && case.test_suffix {
						mods.push(("  ", "  "));
					}

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
				test_prefix: true,
				test_suffix: true,
			},
			CaseKinds {
				source: "//a  \n",
				expected_remove: "//a  \n",
				expected_set_single: " //a  \n",
				expected_set_indent: "\n\t\t//a  \n\t\t",
				expected_set_prev_indent: "\n\t\t//a  \n\t",
				expected_set_prev_indent_or_remove: "\n\t\t//a  \n\t",
				test_prefix: true,
				test_suffix: true,
			},
			CaseKinds {
				source: "/*  a  */",
				expected_remove: "/*  a  */",
				expected_set_single: " /*  a  */ ",
				expected_set_indent: "\n\t\t/*  a  */\n\t\t",
				expected_set_prev_indent: "\n\t\t/*  a  */\n\t",
				expected_set_prev_indent_or_remove: "\n\t\t/*  a  */\n\t",
				test_prefix: true,
				test_suffix: true,
			},
			CaseKinds {
				source: "/*  a  */  /*  b  */",
				expected_remove: "/*  a  *//*  b  */",
				expected_set_single: " /*  a  */ /*  b  */ ",
				expected_set_indent: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t\t",
				expected_set_prev_indent: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t",
				expected_set_prev_indent_or_remove: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t",
				test_prefix: true,
				test_suffix: true,
			},
			CaseKinds {
				source: "/*  a  */  /*  b  */  /*  c  */",
				expected_remove: "/*  a  *//*  b  *//*  c  */",
				expected_set_single: " /*  a  */ /*  b  */ /*  c  */ ",
				expected_set_indent: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t\t/*  c  */\n\t\t",
				expected_set_prev_indent: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t\t/*  c  */\n\t",
				expected_set_prev_indent_or_remove: "\n\t\t/*  a  */\n\t\t/*  b  */\n\t\t/*  c  */\n\t",
				test_prefix: true,
				test_suffix: true,
			},
		];

		let fmt_config = format::Config {
			indent: "\t".to_owned(),
		};
		let config = Config { indent_depth: 2 };
		self::test_cases_with(cases, &fmt_config, &config).map_err(AppError::flatten)
	}
}
