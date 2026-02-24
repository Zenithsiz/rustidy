//! Delimited

// Imports
use {
	rustidy_format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// A value `T` delimited by prefix `L` and suffix `R`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
pub struct Delimited<T, L, R> {
	pub prefix: L,

	// TODO: Should we always remove all tags here?
	#[parse(without_tags)]
	pub value:  T,

	pub suffix: R,
}

impl<T, L, R> Delimited<T, L, R> {
	/// Creates a new delimited from it's inner value
	pub fn from_value(value: T) -> Self
	where
		L: Default,
		R: Default,
	{
		Self {
			prefix: L::default(),
			value,
			suffix: R::default(),
		}
	}
}

// TODO: Create another impl where we don't care about empty/non-empty?
impl<T, L, R, LPrefixWs, TPrefixWs, RPrefixWs, LArgs, TArgs, RArgs> Format<LPrefixWs, FmtArgs<TPrefixWs, RPrefixWs, LArgs, TArgs, RArgs>> for Delimited<T, L, R>
where
	L: Format<LPrefixWs, LArgs>,
	T: Format<TPrefixWs, TArgs>,
	R: Format<RPrefixWs, RArgs>,
	// TODO: Not need this and get 2 copies of the arguments for empty and non-empty.
	TArgs: Clone, {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: LPrefixWs,
		args: FmtArgs<TPrefixWs, RPrefixWs, LArgs, TArgs, RArgs>,
	) -> FormatOutput {
		// TODO: Should we handle the case of the prefix being empty and needing to
		//       pass the prefix whitespace along?
		let mut output = ctx
			.format_with(&mut self.prefix, prefix_ws, args.prefix_args);
		if !output.has_prefix_ws() {
			tracing::warn!("Delimited prefix had no prefix whitespace");
		}

		ctx
			.with_indent_if(
				args.indent,
				|ctx| {
					let value_output = ctx
						.format_with(
							&mut self.value,
							args.value_non_blank,
							args.value_args.clone()
						);
					let value_output = match value_output.is_blank {
						true => ctx
							.format_with(
								&mut self.value,
								args.value_blank,
								args.value_args
							),
						false => value_output,
					};
					value_output.append_to(&mut output);

					let suffix_prefix_ws = match value_output.is_blank {
						true => args.suffix_blank,
						false => args.suffix_non_blank,
					};
					ctx
						.format_with(
							&mut self.suffix,
							suffix_prefix_ws,
							args.suffix_args
						)
						.append_to(&mut output);
				}
			);

		output
	}
}

/// Formatting arguments
// TODO: Switch order of generics to `T, L, R` like `Delimited`.
#[derive(Clone, Copy, Debug)]
pub struct FmtArgs<TPrefixWs, SPrefixWs, LArgs, TArgs, RArgs> {
	pub indent:           bool,

	pub value_non_blank:  TPrefixWs,
	pub suffix_non_blank: SPrefixWs,

	pub value_blank:      TPrefixWs,
	pub suffix_blank:     SPrefixWs,

	pub prefix_args:      LArgs,
	pub value_args:       TArgs,
	pub suffix_args:      RArgs,
}

#[must_use]
pub const fn fmt_preserve_with<LArgs, TArgs, RArgs>(prefix_args: LArgs, value_args: TArgs, suffix_args: RArgs,) -> FmtArgs<WhitespaceConfig, WhitespaceConfig, LArgs, TArgs, RArgs> {
	FmtArgs {
		indent: false,

		value_non_blank: Whitespace::PRESERVE,
		suffix_non_blank: Whitespace::PRESERVE,

		value_blank: Whitespace::PRESERVE,
		suffix_blank: Whitespace::PRESERVE,

		prefix_args,
		value_args,
		suffix_args,
	}
}

// TODO: Use a builder for most of these?
#[must_use]
pub const fn fmt_preserve() -> FmtArgs<WhitespaceConfig, WhitespaceConfig, (), (), ()> {
	self::fmt_preserve_with((), (), ())
}

#[must_use]
pub const fn fmt_single_if_non_blank_with<LArgs, TArgs, RArgs>(prefix_args: LArgs, value_args: TArgs, suffix_args: RArgs,) -> FmtArgs<WhitespaceConfig, WhitespaceConfig, LArgs, TArgs, RArgs> {
	FmtArgs {
		indent: false,

		value_non_blank: Whitespace::SINGLE,
		suffix_non_blank: Whitespace::SINGLE,

		value_blank: Whitespace::REMOVE,
		suffix_blank: Whitespace::REMOVE,

		prefix_args,
		value_args,
		suffix_args,
	}
}

#[must_use]
pub const fn fmt_single_if_non_blank_with_value<TArgs>(value_args: TArgs) -> FmtArgs<WhitespaceConfig, WhitespaceConfig, (), TArgs, ()> {
	self::fmt_single_if_non_blank_with((), value_args, ())
}

#[must_use]
pub const fn fmt_single_if_non_blank() -> FmtArgs<WhitespaceConfig, WhitespaceConfig, (), (), ()> {
	self::fmt_single_if_non_blank_with((), (), ())
}

#[must_use]
pub const fn fmt_indent_if_non_blank_with<LArgs, TArgs, RArgs>(prefix_args: LArgs, value_args: TArgs, suffix_args: RArgs,) -> FmtArgs<WhitespaceConfig, WhitespaceConfig, LArgs, TArgs, RArgs> {
	FmtArgs {
		indent: true,

		value_non_blank: Whitespace::INDENT,
		suffix_non_blank: Whitespace::INDENT_CLOSE,

		value_blank: Whitespace::REMOVE,
		suffix_blank: Whitespace::INDENT_CLOSE_REMOVE_IF_PURE,

		prefix_args,
		value_args,
		suffix_args,
	}
}

#[must_use]
pub const fn fmt_indent_if_non_blank_with_value<TArgs>(value_args: TArgs) -> FmtArgs<WhitespaceConfig, WhitespaceConfig, (), TArgs, ()> {
	self::fmt_indent_if_non_blank_with((), value_args, ())
}

#[must_use]
pub const fn fmt_indent_if_non_blank() -> FmtArgs<WhitespaceConfig, WhitespaceConfig, (), (), ()> {
	self::fmt_indent_if_non_blank_with((), (), ())
}

pub struct FmtRemoveWith<TArgs>(pub TArgs);

impl<T: Format<WhitespaceConfig, TArgs>, L: Format<WhitespaceConfig, ()>, R: Format<WhitespaceConfig, ()>, TArgs> Format<WhitespaceConfig, FmtRemoveWith<TArgs>> for Delimited<T, L, R> {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtRemoveWith<TArgs>
	) -> FormatOutput {
		#[rustidy::config(max_chain_len = 100)]
		FormatOutput::from(
			[
				ctx.format(&mut self.prefix, prefix_ws),
				ctx.format_with(&mut self.value, Whitespace::REMOVE, args.0),
				ctx.format(&mut self.suffix, Whitespace::REMOVE),
			]
		)
	}
}

pub struct FmtRemove;

impl<T: Format<WhitespaceConfig, ()>, L: Format<WhitespaceConfig, ()>, R: Format<WhitespaceConfig, ()>> Format<WhitespaceConfig, FmtRemove> for Delimited<T, L, R> {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		_args: FmtRemove
	) -> FormatOutput {
		self.format(ctx, prefix_ws, FmtRemoveWith(()))
	}
}

/// Formatting arguments for [`fmt_single_or_indent_if_non_blank`]
#[derive(Clone, Copy, Debug)]
pub struct FmtArgsSingleOrIndentIfNonBlank<TArgs> {
	max_len:           usize,
	value_args_single: TArgs,
	value_args_indent: TArgs,
}

/// Formats a delimited with [`fmt_single_if_non_blank`] if under or equal to
/// `max_len`, otherwise formats with [`fmt_indent_if_non_blank`].
#[must_use]
pub const fn fmt_single_or_indent_if_non_blank<TArgs>(
	max_len: usize,
	value_args_single: TArgs,
	value_args_indent: TArgs
) -> FmtArgsSingleOrIndentIfNonBlank<TArgs> {
	FmtArgsSingleOrIndentIfNonBlank { max_len, value_args_single, value_args_indent }
}

impl<T, L, R, TArgs> Format<WhitespaceConfig, FmtArgsSingleOrIndentIfNonBlank<TArgs>> for Delimited<T, L, R>
where
	L: Format<WhitespaceConfig, ()>,
	T: Format<WhitespaceConfig, TArgs>,
	R: Format<WhitespaceConfig, ()>,
	TArgs: Clone {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtArgsSingleOrIndentIfNonBlank<TArgs>
	) -> FormatOutput {
		let output = self
			.format(
				ctx,
				prefix_ws,
				self::fmt_single_if_non_blank_with_value(args.value_args_single)
			);

		match output.len_non_multiline_ws() <= args.max_len {
			true => output,
			false => self
				.format(
					ctx,
					prefix_ws,
					self::fmt_indent_if_non_blank_with_value(args.value_args_indent)
				),
		}
	}
}

/// Formatting arguments for [`fmt_remove_or_indent_if_non_blank`]
#[derive(Clone, Copy, Debug)]
pub struct FmtArgsRemoveOrIndentIfNonBlank<TArgs> {
	max_len:           usize,
	value_args_remove: TArgs,
	value_args_indent: TArgs,
}

/// Formats a delimited with [`fmt_remove_if_non_blank`] if under or equal to
/// `max_len`, otherwise formats with [`fmt_indent_if_non_blank`].
#[must_use]
pub const fn fmt_remove_or_indent_if_non_blank<TArgs>(
	max_len: usize,
	value_args_remove: TArgs,
	value_args_indent: TArgs
) -> FmtArgsRemoveOrIndentIfNonBlank<TArgs> {
	FmtArgsRemoveOrIndentIfNonBlank { max_len, value_args_remove, value_args_indent }
}

impl<T, L, R, TArgs> Format<WhitespaceConfig, FmtArgsRemoveOrIndentIfNonBlank<TArgs>> for Delimited<T, L, R>
where
	L: Format<WhitespaceConfig, ()>,
	T: Format<WhitespaceConfig, TArgs>,
	R: Format<WhitespaceConfig, ()>,
	TArgs: Clone {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtArgsRemoveOrIndentIfNonBlank<TArgs>
	) -> FormatOutput {
		let output = self
			.format(
				ctx,
				prefix_ws,
				FmtRemoveWith(args.value_args_remove)
			);

		match output.len_non_multiline_ws() <= args.max_len {
			true => output,
			false => self
				.format(
					ctx,
					prefix_ws,
					self::fmt_indent_if_non_blank_with_value(args.value_args_indent)
				),
		}
	}
}
