//! [`Vec<T>`] formatting

// Imports
use {
	crate::{Context, Format, FormatOutput, Formattable, WhitespaceConfig},
	core::ops::ControlFlow,
	rustidy_util::AstStr,
};

impl<T: Formattable> Formattable for Vec<T> {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut rustidy_util::Whitespace, &mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		for value in self {
			match value.with_prefix_ws(ctx, f) {
				Ok(output) => return Ok(output),
				Err(ControlFlow::Continue(())) => (),
				Err(ControlFlow::Break(())) => return Err(ControlFlow::Break(())),
			}
		}

		Err(ControlFlow::Continue(()))
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		mut exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		let mut is_empty = true;
		for value in self {
			is_empty &= value.with_strings(ctx, exclude_prefix_ws, f)?;

			if !is_empty {
				exclude_prefix_ws = false;
			}
		}

		ControlFlow::Continue(is_empty)
	}
}

impl<T, A> Format<Args<A>> for Vec<T>
where
	T: Format<A>,
	A: Clone,
{
	fn format(&mut self, ctx: &mut Context, prefix_ws: WhitespaceConfig, args: Args<A>) -> FormatOutput {
		// Note: Due to the way we're parsed, the first element will never be non-empty,
		//       but it's possible for the caller to create this value during formatting
		//       and have that not be true, so we always check.
		let mut output = FormatOutput::default();
		let mut has_prefix_ws = true;
		for value in self {
			let value_output = match has_prefix_ws {
				true => value.format(ctx, prefix_ws, args.args.clone()),
				false => value.format(ctx, args.rest_prefix_ws, args.args.clone()),
			};
			value_output.append_to(&mut output);

			if has_prefix_ws && value_output.has_prefix_ws() {
				has_prefix_ws = false;
			}
		}

		output
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
