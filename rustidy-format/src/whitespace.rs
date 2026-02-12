//! Whitespace impls

// Imports
use {
	crate::{Format, FormatFn, FormatTag},
	core::ops::ControlFlow,
	itertools::Itertools,
	rustidy_util::{
		AstStr,
		ast_str::AstStrRepr,
		whitespace::{Comment, Whitespace},
	},
	std::sync::Arc,
};

// TODO: This needs documentation since we removed `Format::prefix_ws_*`.
#[extend::ext(name = WhitespaceFormat)]
pub impl Whitespace {
	fn is_pure(&mut self, _ctx: &mut crate::Context) -> bool {
		self.0.get().rest.is_empty()
	}

	fn preserve(&mut self, _ctx: &mut crate::Context) {}

	fn remove(&mut self, ctx: &mut crate::Context) {
		self::format(self, ctx, FormatKind::Remove);
	}

	fn set_spaces(&mut self, ctx: &mut crate::Context, len: usize) {
		self::format(self, ctx, FormatKind::Spaces { len });
	}

	fn set_single(&mut self, ctx: &mut crate::Context) {
		self.set_spaces(ctx, 1);
	}

	fn set_indent(&mut self, ctx: &mut crate::Context, offset: isize, remove_if_empty: bool) {
		self::format(self, ctx, FormatKind::Indent {
			offset,
			remove_if_empty,
		});
	}

	fn set_cur_indent(&mut self, ctx: &mut crate::Context) {
		self.set_indent(ctx, 0, false);
	}

	fn set_prev_indent(&mut self, ctx: &mut crate::Context) {
		self.set_indent(ctx, -1, false);
	}

	fn set_prev_indent_remove_if_empty(&mut self, ctx: &mut crate::Context) {
		self.set_indent(ctx, -1, true);
	}

	fn set_next_indent(&mut self, ctx: &mut crate::Context) {
		self.set_indent(ctx, 1, false);
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
	fn with_strings<O>(
		&mut self,
		ctx: &mut crate::Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut crate::Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		if exclude_prefix_ws {
			return ControlFlow::Continue(());
		}

		let mut inner = self.0.get_mut();
		f(&mut inner.first.0, ctx)?;
		for (comment, pure) in &mut inner.rest {
			match comment {
				Comment::Line(comment) => f(&mut comment.0, ctx)?,
				Comment::Block(comment) => f(&mut comment.0, ctx)?,
			}
			f(&mut pure.0, ctx)?;
		}

		ControlFlow::Continue(())
	}

	fn format(&mut self, ctx: &mut crate::Context, prefix_ws: &mut impl FormatFn<Self>) {
		prefix_ws(self, ctx);
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
	fn indent_str(ctx: &crate::Context) -> AstStrRepr {
		AstStrRepr::Indentation {
			indent:   Arc::clone(&ctx.config().indent),
			newlines: 0,
			depth:    ctx.indent(),
		}
	}

	/// Returns the indentation string, with a newline *before*
	// TODO: Should we be checking for multiple newlines?
	fn indent_str_nl(ctx: &mut crate::Context, cur_str: &AstStr, after_newline: bool) -> AstStrRepr {
		let min_newlines = ctx.config().min_empty_lines;
		let max_newlines = ctx.config().max_empty_lines;
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
	fn prefix_str(self, ctx: &mut crate::Context, cur_str: &AstStr, is_last: bool, after_newline: bool) -> AstStrRepr {
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
	fn after_newline_str(self, ctx: &mut crate::Context, cur_str: &AstStr, is_last: bool) -> AstStrRepr {
		match self {
			Self::Remove | Self::Spaces { .. } => "".into(),
			Self::Indent { offset, .. } => match is_last {
				true => ctx.with_indent_offset(offset, |ctx| Self::indent_str(ctx)),
				false => Self::indent_str_nl(ctx, cur_str, true),
			},
		}
	}

	/// Returns the normal string
	fn normal_str(self, ctx: &mut crate::Context, cur_str: &AstStr, is_last: bool) -> AstStrRepr {
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

#[doc(hidden)]
pub fn format(ws: &mut Whitespace, ctx: &mut crate::Context, kind: FormatKind) {
	let mut inner = ws.0.get_mut();

	// Note: If we're whitespace after a line doc comment, then we have a newline
	//       prior to us that we need to take into account.
	let after_newline = ctx.take_tag(FormatTag::AfterNewline);

	let prefix_str = kind.prefix_str(ctx, &inner.first.0, inner.rest.is_empty(), after_newline);
	inner.first.0.replace(ctx.input, prefix_str);

	for (pos, (comment, ws)) in inner.rest.iter_mut().with_position() {
		let is_last = matches!(pos, itertools::Position::Last | itertools::Position::Only);
		let ws_str = match comment.is_line() {
			true => kind.after_newline_str(ctx, &ws.0, is_last),
			false => kind.normal_str(ctx, &ws.0, is_last),
		};
		ws.0.replace(ctx.input, ws_str);
	}
}
