//! Whitespace

// Imports
use {
	core::any::Any,
	itertools::Itertools,
	rustidy_format::{Format, WhitespaceLike},
	rustidy_parse::{Parse, ParseError, Parser, ParserError},
	rustidy_print::Print,
	rustidy_util::{Arena, ArenaData, ArenaIdx, AstStr, ast_str::AstStrRepr},
	std::sync::Arc,
};

/// Whitespace
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Print)]
#[parse(try_with = Self::parse_skip)]
#[expect(clippy::use_self, reason = "`Parse` derive macro doesn't support `Self`")]
pub struct Whitespace(ArenaIdx<Whitespace>);

impl Whitespace {
	/// Creates an empty whitespace
	#[must_use]
	pub fn empty() -> Self {
		let inner = WhitespaceInner {
			first: PureWhitespace(AstStr::new("")),
			rest:  vec![],
		};
		let idx = ArenaIdx::new(inner);

		Self(idx)
	}

	#[expect(clippy::unnecessary_wraps, reason = "Necessary for type signature")]
	fn parse_skip(parser: &mut Parser) -> Result<Option<Self>, WhitespaceError> {
		Ok(parser.has_tag("skip:Whitespace").then(|| {
			let s = parser.update_with(|_| ());
			let inner = WhitespaceInner {
				first: PureWhitespace(s),
				rest:  vec![],
			};
			let idx = ArenaIdx::new(inner);

			Self(idx)
		}))
	}

	#[doc(hidden)]
	pub fn format(&mut self, ctx: &mut rustidy_format::Context, kind: FormatKind) {
		let mut inner = self.0.get_mut();

		// Note: If we're whitespace after a line doc comment, then we have a newline
		//       prior to us that we need to take into account.
		// TODO: Using the input to check this isn't ideal and is just a hack, since it
		//       could have changed already. Ideally we'd need some `Format::with_strings_before` or alike.
		//       This even breaks when the same whitespace gets formatted multiple time, since we'll
		//       stop being a range.
		fn is_after_newline(repr: &AstStrRepr, ctx: &mut rustidy_format::Context) -> bool {
			match *repr {
				AstStrRepr::AstRange(ref range) => range.str_before(ctx.input()).ends_with('\n'),
				AstStrRepr::Join { ref lhs, .. } => is_after_newline(&lhs.repr(), ctx),
				_ => false,
			}
		}
		let after_newline = is_after_newline(&inner.first.0.repr(), ctx);

		let prefix_str = kind.prefix_str(ctx, &inner.first.0, inner.rest.is_empty(), after_newline);
		inner.first.0 = AstStr::new(prefix_str);

		for (pos, (comment, ws)) in inner.rest.iter_mut().with_position() {
			let is_last = matches!(pos, itertools::Position::Last | itertools::Position::Only);
			let ws_str = match comment.is_line() {
				true => kind.after_newline_str(ctx, &ws.0, is_last),
				false => kind.normal_str(ctx, &ws.0, is_last),
			};
			ws.0 = AstStr::new(ws_str);
		}
	}
}

impl ArenaData for Whitespace {
	type Data = WhitespaceInner;

	const ARENA: &'static Arena<Self> = &ARENA;
}

static ARENA: Arena<Whitespace> = Arena::new();

impl WhitespaceLike for Whitespace {
	fn as_concrete<W: 'static>(&mut self) -> &mut W {
		(self as &mut dyn Any).downcast_mut().expect("Wrong whitespace type")
	}

	fn is_pure(&mut self, _ctx: &mut rustidy_format::Context) -> bool {
		self.0.get().rest.is_empty()
	}

	fn remove(&mut self, ctx: &mut rustidy_format::Context) {
		Self::format(self, ctx, FormatKind::Remove);
	}

	fn set_spaces(&mut self, ctx: &mut rustidy_format::Context, len: usize) {
		Self::format(self, ctx, FormatKind::Spaces { len });
	}

	fn set_indent(&mut self, ctx: &mut rustidy_format::Context, offset: isize, remove_if_empty: bool) {
		Self::format(self, ctx, FormatKind::Indent {
			offset,
			remove_if_empty,
		});
	}

	fn join_suffix(&mut self, other: Self) {
		let mut lhs = self.0.get_mut();
		let mut rhs = other.0.take();

		let lhs_last = match lhs.rest.last_mut() {
			Some((_, last)) => last,
			None => &mut lhs.first,
		};

		replace_with::replace_with_or_abort(&mut lhs_last.0, |lhs_last| AstStr::join(lhs_last, rhs.first.0));
		lhs.rest.append(&mut rhs.rest);
	}

	fn join_prefix(&mut self, mut other: Self) {
		replace_with::replace_with_or_abort(self, |this| {
			other.join_suffix(this);
			other
		});
	}
}

impl Format for Whitespace {
	fn with_strings(
		&mut self,
		ctx: &mut rustidy_format::Context,
		f: &mut impl FnMut(&mut AstStr, &mut rustidy_format::Context),
	) {
		let mut inner = self.0.get_mut();
		f(&mut inner.first.0, ctx);
		for (comment, pure) in &mut inner.rest {
			match comment {
				Comment::Line(comment) => f(&mut comment.0, ctx),
				Comment::Block(comment) => f(&mut comment.0, ctx),
			}
			f(&mut pure.0, ctx);
		}
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

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WhitespaceInner {
	first: PureWhitespace,
	rest:  Vec<(Comment, PureWhitespace)>,
}

impl Parse for WhitespaceInner {
	type Error = WhitespaceInnerError;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let first = parser.parse::<PureWhitespace>()?;
		let mut rest = vec![];
		while let Ok(comment) = parser.try_parse::<Comment>()? {
			let pure = parser.parse::<PureWhitespace>()?;
			rest.push((comment, pure));
		}

		Ok(Self { first, rest })
	}
}

#[derive(Debug, derive_more::From, ParseError)]
pub enum WhitespaceInnerError {
	Pure(ParserError<PureWhitespace>),
	Comment(ParserError<Comment>),
}

impl Print for WhitespaceInner {
	fn print(&self, f: &mut rustidy_print::PrintFmt) {
		self.first.print(f);
		for (comment, pure) in &self.rest {
			comment.print(f);
			pure.print(f);
		}
	}
}

#[derive(Clone, Copy, Debug)]
#[derive(strum::EnumIs)]
#[doc(hidden)]
pub enum FormatKind {
	Remove,

	Spaces {
		/// Number of spaces
		len: usize,
	},

	Indent {
		/// Indentation offset
		offset: isize,

		/// Remove if no comments exist
		remove_if_empty: bool,
	},
}

impl FormatKind {
	/// Returns the indentation string, without a newline
	fn indent_str(ctx: &rustidy_format::Context) -> AstStrRepr {
		AstStrRepr::Indentation {
			indent:   Arc::clone(&ctx.config().indent),
			newlines: 0,
			depth:    ctx.indent(),
		}
	}

	/// Returns the indentation string, with a newline *before*
	// TODO: Should we be checking for multiple newlines?
	fn indent_str_nl(ctx: &mut rustidy_format::Context, cur_str: &AstStr, after_newline: bool) -> AstStrRepr {
		let min_newlines = ctx.config().empty_line_spacing.min;
		let max_newlines = ctx.config().empty_line_spacing.max;
		let (min_newlines, max_newlines) = match after_newline {
			true => (min_newlines, max_newlines),
			false => (min_newlines + 1, max_newlines + 1),
		};
		let newlines = ctx.str(cur_str).chars().filter(|&ch| ch == '\n').count();
		let newlines = newlines.clamp(min_newlines, max_newlines);

		AstStrRepr::Indentation {
			indent: Arc::clone(&ctx.config().indent),
			newlines,
			depth: ctx.indent(),
		}
	}

	/// Returns the prefix string
	fn prefix_str(
		self,
		ctx: &mut rustidy_format::Context,
		cur_str: &AstStr,
		is_last: bool,
		after_newline: bool,
	) -> AstStrRepr {
		match self {
			Self::Remove => "".into(),
			Self::Spaces { len } => AstStrRepr::Spaces { len },
			Self::Indent {
				offset,
				remove_if_empty,
			} => match remove_if_empty && is_last {
				true => "".into(),
				false =>
					ctx.with_indent_offset_if(offset, is_last, |ctx| Self::indent_str_nl(ctx, cur_str, after_newline)),
			},
		}
	}

	/// Returns the string after a newline
	fn after_newline_str(self, ctx: &mut rustidy_format::Context, cur_str: &AstStr, is_last: bool) -> AstStrRepr {
		match self {
			Self::Remove | Self::Spaces { .. } => "".into(),
			Self::Indent { offset, .. } => match is_last {
				true => ctx.with_indent_offset(offset, |ctx| Self::indent_str(ctx)),
				false => Self::indent_str_nl(ctx, cur_str, true),
			},
		}
	}

	/// Returns the normal string
	fn normal_str(self, ctx: &mut rustidy_format::Context, cur_str: &AstStr, is_last: bool) -> AstStrRepr {
		match self {
			Self::Remove => "".into(),
			Self::Spaces { len } => AstStrRepr::Spaces { len },
			Self::Indent { offset, .. } => match is_last {
				true => ctx.with_indent_offset(offset, |ctx| Self::indent_str_nl(ctx, cur_str, false)),
				false => Self::indent_str_nl(ctx, cur_str, false),
			},
		}
	}
}

/// Pure whitespace
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Print)]
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
#[derive(Parse, Print)]
pub enum Comment {
	Line(LineComment),
	Block(BlockComment),
}

/// Block Comment
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Print)]
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
#[derive(Parse, Print)]
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

/// Sets the whitespace to the current indentation
pub fn set_indent(offset: isize, remove_if_empty: bool) -> impl Fn(&mut Whitespace, &mut rustidy_format::Context) {
	move |ws, ctx| ws.set_indent(ctx, offset, remove_if_empty)
}
