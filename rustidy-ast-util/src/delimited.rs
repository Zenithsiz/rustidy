//! Delimited

// Imports
use {
	rustidy_format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// A value `T` delimited by prefix `L` and suffix `R`
#[derive(PartialEq, Eq, Debug)]
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

impl<T, L, R, LPrefixWs, TPrefixWs, RPrefixWs, LArgs, TArgs, RArgs> Format<LPrefixWs, FmtArgs<TPrefixWs, RPrefixWs, LArgs, TArgs, RArgs>> for Delimited<T, L, R>
where
	TArgs: Clone,
	L: Format<LPrefixWs, LArgs>,
	T: Format<TPrefixWs, TArgs>,
	R: Format<RPrefixWs, RArgs>, {
	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: LPrefixWs, args: FmtArgs<TPrefixWs, RPrefixWs, LArgs, TArgs, RArgs>,) -> FormatOutput {
		// TODO: Should we handle the case of the prefix being empty and needing to
		//       pass the prefix whitespace along?
		let mut output = self
			.prefix
			.format(ctx, prefix_ws, args.prefix_args);
		assert!(output.has_prefix_ws(), "Delimited prefix had no prefix whitespace");

		let value_output = self
			.value
			.format(ctx, args.value_non_blank, args.value_args.clone());
		let value_output = match value_output.is_blank {
			true => self
				.value
				.format(ctx, args.value_blank, args.value_args),
			false => value_output,
		};
		value_output.append_to(&mut output);

		let suffix_prefix_ws = match value_output.is_blank {
			true => args.suffix_blank,
			false => args.suffix_non_blank,
		};
		self
			.suffix
			.format(ctx, suffix_prefix_ws, args.suffix_args)
			.append_to(&mut output);

		output
	}
}

/// Formatting arguments
// TODO: Switch order of generics to `T, L, R` like `Delimited`.
#[derive(Clone, Copy, Debug)]
pub struct FmtArgs<TPrefixWs, SPrefixWs, LArgs, TArgs, RArgs> {
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
		value_non_blank: Whitespace::SINGLE,
		suffix_non_blank: Whitespace::SINGLE,

		value_blank: Whitespace::REMOVE,
		suffix_blank: Whitespace::SINGLE,

		prefix_args,
		value_args,
		suffix_args,
	}
}

#[must_use]
pub const fn fmt_single_if_non_blank() -> FmtArgs<WhitespaceConfig, WhitespaceConfig, (), (), ()> {
	self::fmt_single_if_non_blank_with((), (), ())
}

#[must_use]
pub const fn fmt_indent_if_non_blank_with<LArgs, TArgs, RArgs>(prefix_args: LArgs, value_args: TArgs, suffix_args: RArgs,) -> FmtArgs<WhitespaceConfig, WhitespaceConfig, LArgs, TArgs, RArgs> {
	FmtArgs {
		value_non_blank: Whitespace::CUR_INDENT,
		suffix_non_blank: Whitespace::PREV_INDENT,

		value_blank: Whitespace::REMOVE,
		suffix_blank: Whitespace::PREV_INDENT_REMOVE_IF_PURE,

		prefix_args,
		value_args,
		suffix_args,
	}
}

#[must_use]
pub const fn fmt_indent_if_non_blank() -> FmtArgs<WhitespaceConfig, WhitespaceConfig, (), (), ()> {
	self::fmt_indent_if_non_blank_with((), (), ())
}

#[must_use]
pub const fn fmt_remove_with<LArgs, TArgs, RArgs>(prefix_args: LArgs, value_args: TArgs, suffix_args: RArgs,) -> FmtArgs<WhitespaceConfig, WhitespaceConfig, LArgs, TArgs, RArgs> {
	FmtArgs {
		value_non_blank: Whitespace::REMOVE,
		suffix_non_blank: Whitespace::REMOVE,

		value_blank: Whitespace::REMOVE,
		suffix_blank: Whitespace::REMOVE,

		prefix_args,
		value_args,
		suffix_args,
	}
}

#[must_use]
pub const fn fmt_remove() -> FmtArgs<WhitespaceConfig, WhitespaceConfig, (), (), ()> {
	self::fmt_remove_with((), (), ())
}
