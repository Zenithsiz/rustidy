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
	pub value: T,

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

impl<T, L, R, AL, AT, AR> Format<FmtArgs<AL, AT, AR>> for Delimited<T, L, R>
where
	AT: Clone,
	L: Format<AL>,
	T: Format<AT>,
	R: Format<AR>,
{
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtArgs<AL, AT, AR>,
	) -> FormatOutput {
		// TODO: Should we handle the case of the prefix being empty and needing to
		//       pass the prefix whitespace along?
		let mut output = self.prefix.format(ctx, prefix_ws, args.prefix_args);

		let value_output = self.value.format(ctx, args.value_non_blank, args.value_args.clone());
		let value_output = match value_output.is_blank {
			true => self.value.format(ctx, args.value_blank, args.value_args),
			false => value_output,
		};
		value_output.append_to(&mut output);

		let suffix_prefix_ws = match value_output.is_blank {
			true => args.suffix_blank,
			false => args.suffix_non_blank,
		};
		self.suffix
			.format(ctx, suffix_prefix_ws, args.suffix_args)
			.append_to(&mut output);

		output
	}
}

/// Formatting arguments
// TODO: Switch order of generics to `T, L, R` like `Delimited`.
#[derive(Clone, Copy, Debug)]
pub struct FmtArgs<AL, AT, AR> {
	pub value_non_blank:  WhitespaceConfig,
	pub suffix_non_blank: WhitespaceConfig,

	pub value_blank:  WhitespaceConfig,
	pub suffix_blank: WhitespaceConfig,

	pub prefix_args: AL,
	pub value_args:  AT,
	pub suffix_args: AR,
}

#[must_use]
pub const fn fmt_preserve_with<AL, AT, AR>(prefix_args: AL, value_args: AT, suffix_args: AR) -> FmtArgs<AL, AT, AR> {
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
pub const fn fmt_preserve() -> FmtArgs<(), (), ()> {
	self::fmt_preserve_with((), (), ())
}

#[must_use]
pub const fn fmt_single_if_non_blank_with<AL, AT, AR>(
	prefix_args: AL,
	value_args: AT,
	suffix_args: AR,
) -> FmtArgs<AL, AT, AR> {
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
pub const fn fmt_single_if_non_blank() -> FmtArgs<(), (), ()> {
	self::fmt_single_if_non_blank_with((), (), ())
}

#[must_use]
pub const fn fmt_indent_if_non_blank_with<AL, AT, AR>(
	prefix_args: AL,
	value_args: AT,
	suffix_args: AR,
) -> FmtArgs<AL, AT, AR> {
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
pub const fn fmt_indent_if_non_blank() -> FmtArgs<(), (), ()> {
	self::fmt_indent_if_non_blank_with((), (), ())
}

#[must_use]
pub const fn fmt_remove_with<AL, AT, AR>(prefix_args: AL, value_args: AT, suffix_args: AR) -> FmtArgs<AL, AT, AR> {
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
pub const fn fmt_remove() -> FmtArgs<(), (), ()> {
	self::fmt_remove_with((), (), ())
}
