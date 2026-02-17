//! [`Vec<T>`] formatting

// Imports
use {
	crate::{Context, Format, Formattable, WhitespaceConfig},
	core::ops::ControlFlow,
	rustidy_util::AstStr,
};

impl<T: Formattable> Formattable for Vec<T> {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut rustidy_util::Whitespace, &mut Context) -> O,
	) -> Option<O> {
		for value in self {
			if let Some(output) = value.with_prefix_ws(ctx, f) {
				return Some(output);
			}

			if !value.is_empty(ctx, false) {
				return None;
			}
		}

		None
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		for value in self {
			value.with_strings(ctx, exclude_prefix_ws, f)?;
		}

		ControlFlow::Continue(())
	}
}

impl<T, A> Format<Args<A>> for Vec<T>
where
	T: Format<A>,
{
	fn format(&mut self, ctx: &mut Context, prefix_ws: WhitespaceConfig, args: &mut Args<A>) {
		// Note: Due to the way we're parsed, the first element will never be non-empty,
		//       but it's possible for the caller to create this value during formatting
		//       and have that not be true, so we always check.
		let mut has_prefix_ws = true;
		for value in self {
			match has_prefix_ws {
				true => value.format(ctx, prefix_ws, &mut args.args),
				false => value.format(ctx, args.rest_prefix_ws, &mut args.args),
			}

			if has_prefix_ws && !value.is_empty(ctx, false) {
				has_prefix_ws = false;
			}
		}
	}
}

/// Arguments for formatting a [`Vec<T>`]
pub struct Args<A> {
	/// Whitespace formatter for the rest of the vector
	rest_prefix_ws: WhitespaceConfig,

	/// Arguments for the rest of the vector
	args: A,
}

/// Creates vector arguments
pub const fn args<A>(rest_prefix_ws: WhitespaceConfig, args: A) -> Args<A> {
	Args { rest_prefix_ws, args }
}

/// Creates vector arguments from just the prefix whitespace
#[must_use]
pub const fn args_prefix_ws(rest_prefix_ws: WhitespaceConfig) -> Args<()> {
	Args {
		rest_prefix_ws,
		args: (),
	}
}
