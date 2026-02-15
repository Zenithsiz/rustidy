//! [`Vec<T>`] formatting

// Imports
use {
	crate::{Context, Format, Formattable, WsFmtFn},
	core::ops::ControlFlow,
	rustidy_util::AstStr,
};

impl<T: Formattable> Formattable for Vec<T> {
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

impl<T, W, A> Format<Args<W, A>> for Vec<T>
where
	T: Format<A>,
	W: WsFmtFn,
{
	fn format(&mut self, ctx: &mut Context, prefix_ws: &impl WsFmtFn, args: &mut Args<W, A>) {
		// Note: Due to the way we're parsed, the first element will never be non-empty,
		//       but it's possible for the caller to create this value during formatting
		//       and have that not be true, so we always check.
		let mut has_prefix_ws = true;
		for value in self {
			match has_prefix_ws {
				true => value.format(ctx, prefix_ws, &mut args.args),
				false => value.format(ctx, &args.rest_prefix_ws, &mut args.args),
			}

			if has_prefix_ws && !value.is_empty(ctx, false) {
				has_prefix_ws = false;
			}
		}
	}
}

/// Arguments for formatting a [`Vec<T>`]
pub struct Args<W, A> {
	/// Whitespace formatter for the rest of the vector
	rest_prefix_ws: W,

	/// Arguments for the rest of the vector
	args: A,
}

/// Creates vector arguments
pub const fn args<W, A>(rest_prefix_ws: W, args: A) -> Args<W, A> {
	Args { rest_prefix_ws, args }
}

/// Creates vector arguments from just the prefix whitespace
pub const fn args_prefix_ws<W>(rest_prefix_ws: W) -> Args<W, ()> {
	Args {
		rest_prefix_ws,
		args: (),
	}
}
