//! Whitespace impls

// Imports
use {
	crate::{Format, FormatTag, Formattable, WhitespaceConfig},
	core::ops::ControlFlow,
	itertools::Itertools,
	rustidy_util::{
		AstStr,
		ast_str::AstStrRepr,
		whitespace::{Comment, Whitespace},
	},
	std::sync::Arc,
};

#[extend::ext(name = WhitespaceFormat)]
pub impl Whitespace {
	const CUR_INDENT: WhitespaceConfig = WhitespaceConfig {
		format: Some(WhitespaceFormatKind::Indent {
			offset:         0,
			remove_if_pure: false,
		}),
	};
	const NEXT_INDENT: WhitespaceConfig = WhitespaceConfig {
		format: Some(WhitespaceFormatKind::Indent {
			offset:         1,
			remove_if_pure: false,
		}),
	};
	const PRESERVE: WhitespaceConfig = WhitespaceConfig { format: None };
	const PREV_INDENT: WhitespaceConfig = WhitespaceConfig {
		format: Some(WhitespaceFormatKind::Indent {
			offset:         -1,
			remove_if_pure: false,
		}),
	};
	const PREV_INDENT_REMOVE_IF_PURE: WhitespaceConfig = WhitespaceConfig {
		format: Some(WhitespaceFormatKind::Indent {
			offset:         -1,
			remove_if_pure: true,
		}),
	};
	const REMOVE: WhitespaceConfig = WhitespaceConfig {
		format: Some(WhitespaceFormatKind::Remove),
	};
	const SINGLE: WhitespaceConfig = WhitespaceConfig {
		format: Some(WhitespaceFormatKind::Spaces { len: 1 }),
	};

	fn spaces(len: usize) -> WhitespaceConfig {
		let len = u16::try_from(len).expect("Cannot format more than 2^16 spaces");
		WhitespaceConfig {
			format: Some(WhitespaceFormatKind::Spaces { len }),
		}
	}

	fn indent(offset: i16, remove_if_pure: bool) -> WhitespaceConfig {
		WhitespaceConfig {
			format: Some(WhitespaceFormatKind::Indent { offset, remove_if_pure }),
		}
	}

	/// Returns if this whitespace only contains pure whitespace
	fn is_pure(&mut self, _ctx: &mut crate::Context) -> bool {
		self.0.get().rest.is_empty()
	}

	/// Joins `other` to this whitespace as a suffix
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

	/// Joins `other` to this whitespace as a prefix
	fn join_prefix(&mut self, mut other: Self) {
		replace_with::replace_with_or_abort(self, |this| {
			other.join_suffix(this);
			other
		});
	}
}

impl Formattable for Whitespace {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut crate::Context,
		f: &mut impl FnMut(&mut Self, &mut crate::Context) -> O,
	) -> Option<O> {
		Some(f(self, ctx))
	}

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
}

impl Format<()> for Whitespace {
	fn format(&mut self, ctx: &mut crate::Context, prefix_ws: WhitespaceConfig, _args: &mut ()) {
		let Some(format) = prefix_ws.format else {
			return;
		};

		self::format(self, ctx, format);
	}
}

#[derive(Clone, Copy, Debug)]
#[derive(strum::EnumIs)]
#[doc(hidden)]
pub enum WhitespaceFormatKind {
	Remove,

	Spaces {
		/// Number of spaces
		len: u16,
	},

	Indent {
		/// Indentation offset
		offset: i16,

		/// Remove if the whitespace is pure
		remove_if_pure: bool,
	},
}

impl WhitespaceFormatKind {
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
			Self::Indent { offset, remove_if_pure } => match remove_if_pure && is_last {
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
pub fn format(ws: &mut Whitespace, ctx: &mut crate::Context, kind: WhitespaceFormatKind) {
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
