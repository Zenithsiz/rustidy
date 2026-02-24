//! Whitespace impls

// Imports
use {
	crate::{Format, FormatOutput, FormatTag, Formattable, WhitespaceConfig},
	core::ops::ControlFlow,
	itertools::Itertools,
	rustidy_util::{AstStr, ast_str::AstStrRepr, whitespace::{Comment, Whitespace}},
	std::sync::Arc,
};

#[extend::ext(name = WhitespaceFormat)]
pub impl Whitespace {
	const INDENT: WhitespaceConfig = WhitespaceConfig {
		format: Some(
			WhitespaceFormatKind::Indent { use_prev: false, remove_if_pure: false, }
		),
	};
	const PRESERVE: WhitespaceConfig = WhitespaceConfig { format: None };
	const INDENT_CLOSE: WhitespaceConfig = WhitespaceConfig {
		format: Some(
			WhitespaceFormatKind::Indent { use_prev: true, remove_if_pure: false, }
		),
	};
	const INDENT_CLOSE_REMOVE_IF_PURE: WhitespaceConfig = WhitespaceConfig {
		format: Some(
			WhitespaceFormatKind::Indent { use_prev: true, remove_if_pure: true, }
		),
	};
	const INDENT_REMOVE_IF_PURE: WhitespaceConfig = WhitespaceConfig {
		format: Some(
			WhitespaceFormatKind::Indent { use_prev: false, remove_if_pure: true, }
		),
	};
	const REMOVE: WhitespaceConfig = WhitespaceConfig { format: Some(WhitespaceFormatKind::Remove), };
	const SINGLE: WhitespaceConfig = WhitespaceConfig {
		format: Some(WhitespaceFormatKind::Spaces { len: 1 }),
	};

	fn spaces(len: usize) -> WhitespaceConfig {
		let len = u16::try_from(len)
			.expect("Cannot format more than 2^16 spaces");
		WhitespaceConfig {
			format: Some(WhitespaceFormatKind::Spaces { len }),
		}
	}

	fn indent(remove_if_pure: bool) -> WhitespaceConfig {
		WhitespaceConfig {
			format: Some(
				WhitespaceFormatKind::Indent { use_prev: false, remove_if_pure }
			),
		}
	}

	fn prev_indent(remove_if_pure: bool) -> WhitespaceConfig {
		WhitespaceConfig {
			format: Some(
				WhitespaceFormatKind::Indent { use_prev: true, remove_if_pure }
			),
		}
	}

	/// Returns if this whitespace is empty
	fn is_empty(&mut self) -> bool {
		self.0.first.0.is_empty() && self.0.rest.is_empty()
	}

	/// Returns if this whitespace only contains pure whitespace
	fn is_pure(&mut self) -> bool {
		self.0.rest.is_empty()
	}

	/// Joins `other` to this whitespace as a suffix
	fn join_suffix(&mut self, other: Self) {
		let lhs = &mut *self.0;
		let mut rhs = other.0.take();

		let lhs_last = match lhs.rest.last_mut() {
			Some((_, last)) => last,
			None => &mut lhs.first,
		};

		replace_with::replace_with_or_abort(
			&mut lhs_last.0,
			|lhs_last| AstStr::join(lhs_last, rhs.first.0)
		);
		lhs.rest.append(&mut rhs.rest);
	}

	/// Joins `other` to this whitespace as a prefix
	fn join_prefix(&mut self, mut other: Self) {
		replace_with::replace_with_or_abort(
			self,
			|this| {
				other.join_suffix(this);
				other
			}
		);
	}
}

impl Formattable for Whitespace {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut crate::Context,
		f: &mut impl FnMut(&mut Self,&mut crate::Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		Ok(f(self, ctx))
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut crate::Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr,&mut crate::Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		let is_empty = self.0.first.0.is_empty() && self.0.rest.is_empty();

		if !exclude_prefix_ws {
			f(&mut self.0.first.0, ctx)?;
			for (comment, pure) in &mut self.0.rest {
				match comment {
					Comment::Line(comment) => f(&mut comment.0, ctx)?,
					Comment::Block(comment) => f(&mut comment.0, ctx)?,
				}
				f(&mut pure.0, ctx)?;
			}
		}

		ControlFlow::Continue(is_empty)
	}

	fn format_output(&mut self, ctx: &mut crate::Context) -> FormatOutput {
		let mut output = self.0.first.0.format_output(ctx);
		for (comment, ws) in &mut self.0.rest {
			match comment {
				Comment::Line(comment) => comment.0
					.format_output(ctx)
					.append_to(&mut output),
				Comment::Block(comment) => comment.0
					.format_output(ctx)
					.append_to(&mut output),
			}

			ws.0.format_output(ctx).append_to(&mut output);
		}
		output.prefix_ws_len = Some(output.len);

		output
	}
}

// Note: This impl is useful for types that have whitespace within them,
//       but require that whitespace to be empty.
// TODO: Remove this impl and just make it so any place
//       that skips whitespace during parsing instead
//       does it at the type level.
impl Format<(), ()> for Whitespace {
	fn format(
		&mut self,
		_ctx: &mut crate::Context,
		_prefix_ws: (),
		_args: ()
	) -> FormatOutput {
		if !self.is_empty() {
			tracing::warn!("Whitespace was not empty");
		}
		FormatOutput::default()
	}
}

impl Format<WhitespaceConfig, ()> for Whitespace {
	fn format(
		&mut self,
		ctx: &mut crate::Context,
		prefix_ws: WhitespaceConfig,
		_args: ()
	) -> FormatOutput {
		if let Some(format) = prefix_ws.format {
			self::format(self, ctx, format);
		}

		self.format_output(ctx)
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
		/// Use the previous indentation
		use_prev:       bool,

		/// Remove if the whitespace is pure
		remove_if_pure: bool,
	},
}

impl WhitespaceFormatKind {
	/// Returns the indentation string, with a newline *before*
	// TODO: Should we be checking for multiple newlines?
	fn indent_str_nl(
		ctx: &mut crate::Context,
		cur_str: &AstStr,
		after_newline: bool
	) -> AstStrRepr {
		let min_newlines = ctx.config().min_empty_lines;
		let max_newlines = ctx.config().max_empty_lines;
		let (min_newlines, max_newlines) = match after_newline {
			true => (min_newlines, max_newlines),
			false => (min_newlines + 1, max_newlines + 1),
		};
		// Note: We try to use the input's number of newlines, since we might
		//       have been changed.
		let newlines = match cur_str.input() {
			Some(input) => rustidy_util::str_count_newlines(input),
			None => cur_str.count_newlines(),
		};
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
		ctx: &mut crate::Context,
		cur_str: &AstStr,
		is_last: bool,
		after_newline: bool
	) -> AstStrRepr {
		match self {
			Self::Remove => "".into(),
			Self::Spaces {
				len
			} => AstStrRepr::Spaces { len },
			Self::Indent {
				use_prev,
				remove_if_pure
			} => match remove_if_pure && is_last {
				true => "".into(),
				false => ctx
					.with_indent_offset_if(
						-1,
						use_prev && is_last,
						|ctx| Self::indent_str_nl(ctx, cur_str, after_newline)
					),
			},
		}
	}

	/// Returns the string after a newline
	fn after_newline_str(
		self,
		ctx: &mut crate::Context,
		cur_str: &AstStr,
		is_last: bool
	) -> AstStrRepr {
		match self {
			Self::Remove | Self::Spaces {
				..
			} => "".into(),
			Self::Indent {
				use_prev,
				..
			} => match is_last {
				true => ctx
					.with_indent_offset_if(
						-1,
						use_prev,
						|ctx| Self::indent_str_nl(ctx, cur_str, true)
					),
				false => Self::indent_str_nl(ctx, cur_str, true),
			},
		}
	}

	/// Returns the normal string
	fn normal_str(
		self,
		ctx: &mut crate::Context,
		cur_str: &AstStr,
		is_last: bool
	) -> AstStrRepr {
		match self {
			Self::Remove => "".into(),
			Self::Spaces {
				len
			} => AstStrRepr::Spaces { len },
			Self::Indent {
				use_prev,
				..
			} => match is_last {
				true => ctx
					.with_indent_offset_if(
						-1,
						use_prev,
						|ctx| Self::indent_str_nl(ctx, cur_str, false)
					),
				false => Self::indent_str_nl(ctx, cur_str, false),
			},
		}
	}
}

#[doc(hidden)]
pub fn format(
	ws: &mut Whitespace,
	ctx: &mut crate::Context,
	kind: WhitespaceFormatKind
) {
	// Note: If we're whitespace after a line doc comment, then we have a newline
	//       prior to us that we need to take into account.
	// TODO: We should do this even when we're preserving the whitespace
	let after_newline = ctx.take_tag(FormatTag::AfterNewline);

	let prefix_str = kind
		.prefix_str(
			ctx,
			&ws.0.first.0,
			ws.0.rest.is_empty(),
			after_newline
		);
	ws.0.first.0.replace(prefix_str);

	for (pos, (comment, ws)) in ws.0.rest.iter_mut().with_position() {
		let is_last = matches!(pos, itertools::Position::Last | itertools::Position::Only);
		let ws_str = match comment.is_line() {
			true => kind.after_newline_str(ctx, &ws.0, is_last),
			false => kind.normal_str(ctx, &ws.0, is_last),
		};
		ws.0.replace(ws_str);

		if is_last && let Comment::Line(comment) = comment && !comment.0.has_newlines() {
			let mut s = comment.0.str().into_owned();
			s.push('\n');
			comment.0.replace(AstStrRepr::String(s.into()));
		}
	}
}
