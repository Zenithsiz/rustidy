//! Whitespace

// Imports
use {
	crate::Punctuated,
	itertools::Itertools,
	rustidy_format::{Format, Replacement, WhitespaceLike},
	rustidy_parse::{Parse, Parser},
	rustidy_print::Print,
	rustidy_util::{Arena, ArenaData, ArenaIdx, AstPos, AstRange, AstStr},
};

/// Whitespace
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Print)]
#[parse(try_with = Self::parse_skip)]
#[expect(clippy::use_self, reason = "`Parse` derive macro doesn't support `Self`")]
pub struct Whitespace(ArenaIdx<Whitespace>);

impl Whitespace {
	/// Creates an empty whitespace at a position
	pub fn empty(pos: AstPos) -> Self {
		let inner = Punctuated {
			first: PureWhitespace(AstStr::empty_at(pos)),
			rest:  vec![],
		};
		let idx = ARENA.push(inner);

		Self(idx)
	}

	#[expect(clippy::unnecessary_wraps, reason = "Necessary for type signature")]
	fn parse_skip(parser: &mut Parser) -> Result<Option<Self>, WhitespaceError> {
		Ok(parser.has_tag("skip:Whitespace").then(|| {
			let s = parser.update_with(|_| ());
			let inner = Punctuated {
				first: PureWhitespace(s),
				rest:  vec![],
			};
			let idx = ARENA.push(inner);

			Self(idx)
		}))
	}

	fn format(&self, ctx: &mut rustidy_format::Context, kind: FormatKind) {
		let mut inner = ARENA.get(&self.0);

		let prefix_str = kind.prefix_str(ctx, &inner.first.0, inner.rest.is_empty());
		ctx.replace(&inner.first.0, prefix_str);

		for (pos, (comment, ws)) in inner.rest.iter_mut().with_position() {
			let is_last = matches!(pos, itertools::Position::Last | itertools::Position::Only);
			let ws_str = match comment.is_line() {
				true => kind.after_newline_str(ctx, &ws.0, is_last),
				false => kind.normal_str(ctx, &ws.0, is_last),
			};
			ctx.replace(&ws.0, ws_str);
		}
	}
}

impl ArenaData for Whitespace {
	type Data = Punctuated<PureWhitespace, Comment>;

	const ARENA: &'static Arena<Self> = &ARENA;
}

static ARENA: Arena<Whitespace> = Arena::new();

impl WhitespaceLike for Whitespace {
	fn remove(&mut self, ctx: &mut rustidy_format::Context) {
		Self::format(self, ctx, FormatKind::Remove);
	}

	fn set_single(&mut self, ctx: &mut rustidy_format::Context) {
		Self::format(self, ctx, FormatKind::Single);
	}

	fn set_indent(&mut self, ctx: &mut rustidy_format::Context, offset: isize, remove_if_empty: bool) {
		Self::format(self, ctx, FormatKind::Indent {
			offset,
			remove_if_empty,
		});
	}
}

impl Format for Whitespace {
	fn input_range(&mut self, ctx: &mut rustidy_format::Context) -> Option<AstRange> {
		ARENA.get(&self.0).input_range(ctx)
	}

	fn with_output(
		&mut self,
		ctx: &mut rustidy_format::Context,
		f: &mut impl FnMut(&mut AstStr, &mut rustidy_format::Context),
	) {
		ARENA.get(&self.0).with_output(ctx, f);
	}

	fn format(&mut self, _ctx: &mut rustidy_format::Context) {
		// Note: By default no formatting is done
	}

	fn with_prefix_ws<V: rustidy_format::WhitespaceVisitor>(
		&mut self,
		ctx: &mut rustidy_format::Context,
		visitor: &mut V,
	) -> Option<V::Output> {
		Some(visitor.visit(self, ctx))
	}
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
	const fn indent_str(ctx: &rustidy_format::Context) -> Replacement {
		Replacement::Indentation {
			newlines: 0,
			depth:    ctx.indent(),
		}
	}

	/// Returns the indentation string, with a newline *before*
	fn indent_str_nl(ctx: &mut rustidy_format::Context, cur_str: &AstStr) -> Replacement {
		// TODO: Should we be checking for multiple newlines?
		let after_newline = cur_str.range().str_before(ctx.input()).ends_with('\n');

		let min_newlines = ctx.config().empty_line_spacing.min;
		let max_newlines = ctx.config().empty_line_spacing.max;
		let (min_newlines, max_newlines) = match after_newline {
			true => (min_newlines, max_newlines),
			false => (min_newlines + 1, max_newlines + 1),
		};
		let newlines = ctx.str(cur_str).chars().filter(|&ch| ch == '\n').count();
		let newlines = newlines.clamp(min_newlines, max_newlines);

		Replacement::Indentation {
			newlines,
			depth: ctx.indent(),
		}
	}

	/// Returns the prefix string
	fn prefix_str(self, ctx: &mut rustidy_format::Context, cur_str: &AstStr, is_last: bool) -> Replacement {
		match self {
			Self::Remove => "".into(),
			Self::Single => " ".into(),
			Self::Indent {
				offset,
				remove_if_empty,
			} => match remove_if_empty && is_last {
				true => "".into(),
				false => ctx.with_indent_offset_if(offset, is_last, |ctx| Self::indent_str_nl(ctx, cur_str)),
			},
		}
	}

	/// Returns the string after a newline
	fn after_newline_str(self, ctx: &mut rustidy_format::Context, cur_str: &AstStr, is_last: bool) -> Replacement {
		match self {
			Self::Remove | Self::Single => "".into(),
			Self::Indent { offset, .. } => match is_last {
				true => ctx.with_indent_offset(offset, |ctx| Self::indent_str(ctx)),
				false => Self::indent_str_nl(ctx, cur_str),
			},
		}
	}

	/// Returns the normal string
	fn normal_str(self, ctx: &mut rustidy_format::Context, cur_str: &AstStr, is_last: bool) -> Replacement {
		match self {
			Self::Remove => "".into(),
			Self::Single => " ".into(),
			Self::Indent { offset, .. } => match is_last {
				true => ctx.with_indent_offset(offset, |ctx| Self::indent_str_nl(ctx, cur_str)),
				false => Self::indent_str_nl(ctx, cur_str),
			},
		}
	}
}

/// Pure whitespace
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PureWhitespace(#[parse(update_with = Self::parse)] pub AstStr);

impl PureWhitespace {
	fn parse(s: &mut &str) {
		*s = s.trim_start();
	}
}

/// Comment
#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum Comment {
	Line(LineComment),
	Block(BlockComment),
}

/// Block Comment
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = NoComment, fmt = "Expected `/*` (except `/*!` or `/**`)"))]
#[parse(error(name = MissingCommentEnd, fmt = "Expected `*/` after `/*`", fatal))]
pub struct BlockComment(#[parse(try_update_with = Self::parse)] pub AstStr);

impl BlockComment {
	fn parse(s: &mut &str) -> Result<(), BlockCommentError> {
		let is_doc_comment =
			(s.starts_with("/**") && !s.starts_with("/***") && !s.starts_with("/**/")) || s.starts_with("/*!");

		match s.strip_prefix("/*") {
			Some(rest) if !is_doc_comment => {
				*s = rest;
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
			_ => Err(BlockCommentError::NoComment),
		}
	}
}

/// Line comment
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = NoComment, fmt = "Expected `//` (except `///` or `//!`)"))]
#[parse(error(name = Newline, fmt = "Expected newline after `//`"))]
pub struct LineComment(#[parse(try_update_with = Self::parse)] pub AstStr);

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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = NoComment, fmt = "Expected `//` (except `///` or `//!`)"))]
pub struct TrailingLineComment(#[parse(try_update_with = Self::parse)] pub AstStr);

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

/// Sets the whitespace to the current indentation
pub fn set_indent(offset: isize, remove_if_empty: bool) -> impl Fn(&mut Whitespace, &mut rustidy_format::Context) {
	move |ws, ctx| ws.set_indent(ctx, offset, remove_if_empty)
}

#[cfg(test)]
mod tests {
	use {
		super::*,
		app_error::{AppError, Context, ensure},
		rustidy_format::Replacements,
		rustidy_parse::ParseError,
		rustidy_print::{Print, PrintFmt},
	};

	#[derive(Clone, Debug)]
	struct Config {
		indent_depth: usize,
	}

	fn test_case_with(
		source: &str,
		expected: &str,
		fmt_config: &rustidy_format::Config,
		config: &Config,
		kind: FormatKind,
	) -> Result<(), AppError> {
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
		let mut fmt_ctx = rustidy_format::Context::new(source, &mut replacements, fmt_config);
		fmt_ctx.set_indent_depth(config.indent_depth);
		whitespace.format(&mut fmt_ctx, kind);

		let mut print_fmt = PrintFmt::new(source, fmt_config, &replacements);
		whitespace.print(&mut print_fmt);
		let output = print_fmt.output();

		let source = source.replace(' ', "·").replace('\t', "⭾");
		let expected = expected.replace(' ', "·").replace('\t', "⭾");
		let output = output.replace(' ', "·").replace('\t', "⭾");

		ensure!(
			output == expected,
			"Found wrong output.\nKind    : {kind:?}\nInput   : {source:?}\nExpected: {expected:?}\nFound   : \
			 {output:?}"
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
		fmt_config: &rustidy_format::Config,
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

		let fmt_config = rustidy_format::Config::default();
		let config = Config { indent_depth: 2 };
		self::test_cases_with(cases, &fmt_config, &config).map_err(AppError::flatten)
	}
}
