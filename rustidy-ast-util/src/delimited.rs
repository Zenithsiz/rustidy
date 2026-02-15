//! Delimited

// Imports
use {
	rustidy_format::{Format, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// A value `T` delimited by prefix `L` and suffix `R`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(args(ty = "FmtArgs<AL, AT, AR>", generic = "AL", generic = "AT", generic = "AR",))]
#[format(where_format = "where L: Format<AL>, T: Format<AT>, R: Format<AR>")]
pub struct Delimited<T, L, R> {
	#[format(args = args.prefix_args)]
	pub prefix: L,

	// TODO: Should we always remove all tags here?
	#[parse(without_tags)]
	#[format(prefix_ws = match self.value.is_blank(ctx, false) {
		true => args.value_empty,
		false => args.value_non_empty,
	})]
	#[format(args = args.value_args)]
	pub value: T,

	#[format(prefix_ws = match self.value.is_blank(ctx, false) {
		true => args.suffix_empty,
		false => args.suffix_non_empty,
	})]
	#[format(args = args.suffix_args)]
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

/// Formatting arguments
pub struct FmtArgs<AL, AT, AR> {
	pub value_non_empty:  WhitespaceConfig,
	pub suffix_non_empty: WhitespaceConfig,

	pub value_empty:  WhitespaceConfig,
	pub suffix_empty: WhitespaceConfig,

	pub prefix_args: AL,
	pub value_args:  AT,
	pub suffix_args: AR,
}

#[must_use]
pub const fn fmt_preserve<AL, AT, AR>(prefix_args: AL, value_args: AT, suffix_args: AR) -> FmtArgs<AL, AT, AR> {
	FmtArgs {
		value_non_empty: Whitespace::PRESERVE,
		suffix_non_empty: Whitespace::PRESERVE,

		value_empty: Whitespace::PRESERVE,
		suffix_empty: Whitespace::PRESERVE,

		prefix_args,
		value_args,
		suffix_args,
	}
}

#[must_use]
pub const fn fmt_single_if_non_blank<AL, AT, AR>(
	prefix_args: AL,
	value_args: AT,
	suffix_args: AR,
) -> FmtArgs<AL, AT, AR> {
	FmtArgs {
		value_non_empty: Whitespace::SINGLE,
		suffix_non_empty: Whitespace::SINGLE,

		value_empty: Whitespace::REMOVE,
		suffix_empty: Whitespace::SINGLE,

		prefix_args,
		value_args,
		suffix_args,
	}
}

#[must_use]
pub const fn fmt_indent_if_non_blank<AL, AT, AR>(
	prefix_args: AL,
	value_args: AT,
	suffix_args: AR,
) -> FmtArgs<AL, AT, AR> {
	FmtArgs {
		value_non_empty: Whitespace::CUR_INDENT,
		suffix_non_empty: Whitespace::PREV_INDENT,

		value_empty: Whitespace::REMOVE,
		suffix_empty: Whitespace::PREV_INDENT_REMOVE_IF_PURE,

		prefix_args,
		value_args,
		suffix_args,
	}
}

#[must_use]
pub const fn fmt_remove<AL, AT, AR>(prefix_args: AL, value_args: AT, suffix_args: AR) -> FmtArgs<AL, AT, AR> {
	FmtArgs {
		value_non_empty: Whitespace::REMOVE,
		suffix_non_empty: Whitespace::REMOVE,

		value_empty: Whitespace::REMOVE,
		suffix_empty: Whitespace::REMOVE,

		prefix_args,
		value_args,
		suffix_args,
	}
}
