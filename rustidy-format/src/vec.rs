//! [`Vec<T>`] formatting

// Imports
use {
	crate::{Context, Format, FormatOutput, Formattable},
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

impl<T, PrefixWs, A> Format<PrefixWs, Args<PrefixWs, A>> for Vec<T>
where
	T: Format<PrefixWs, A>,
	PrefixWs: Clone,
	A: Clone,
{
	fn format(&mut self, ctx: &mut Context, prefix_ws: PrefixWs, args: Args<PrefixWs, A>) -> FormatOutput {
		// Note: Due to the way we're parsed, the first element will never be non-empty,
		//       but it's possible for the caller to create this value during formatting
		//       and have that not be true, so we always check.
		let mut output = FormatOutput::default();
		let mut prefix_ws = Some(prefix_ws);
		for value in self {
			let value_output = match &prefix_ws {
				Some(prefix_ws) => value.format(ctx, prefix_ws.clone(), args.args.clone()),
				None => value.format(ctx, args.rest_prefix_ws.clone(), args.args.clone()),
			};
			value_output.append_to(&mut output);

			if prefix_ws.is_some() && value_output.has_prefix_ws() {
				prefix_ws = None;
			}
		}

		output
	}
}

/// Arguments for formatting a [`Vec<T>`]
pub struct Args<PrefixWs, A> {
	/// Whitespace formatter for the rest of the vector
	rest_prefix_ws: PrefixWs,

	/// Arguments for the rest of the vector
	args: A,
}

/// Creates vector arguments
pub const fn args<PrefixWs, A>(rest_prefix_ws: PrefixWs, args: A) -> Args<PrefixWs, A> {
	Args { rest_prefix_ws, args }
}

/// Creates vector arguments from just the prefix whitespace
#[must_use]
pub const fn args_prefix_ws<PrefixWs>(rest_prefix_ws: PrefixWs) -> Args<PrefixWs, ()> {
	Args {
		rest_prefix_ws,
		args: (),
	}
}
