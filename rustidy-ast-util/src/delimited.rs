//! Delimited

// Imports
use {
	rustidy_format::{Format, WhitespaceFormat, WsFmtFn},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// A value `T` delimited by prefix `L` and suffix `R`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(args(
	ty = "FmtArgs<WT, WR, AL, AT, AR>",
	generic = "WT: WsFmtFn",
	generic = "WR: WsFmtFn",
	generic = "AL",
	generic = "AT",
	generic = "AR",
))]
#[format(where_format = "where L: Format<AL>, T: Format<AT>, R: Format<AR>")]
pub struct Delimited<T, L, R> {
	#[format(args = args.prefix_args)]
	pub prefix: L,

	// TODO: Should we always remove all tags here?
	#[parse(without_tags)]
	#[format(prefix_ws = match self.value.is_blank(ctx, false) {
		true => &args.value_empty,
		false => &args.value_non_empty,
	})]
	#[format(args = args.value_args)]
	pub value: T,

	#[format(prefix_ws = match self.value.is_blank(ctx, false) {
		true => &args.suffix_empty,
		false => &args.suffix_non_empty,
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
pub struct FmtArgs<WT, WR, AL, AT, AR> {
	pub value_non_empty:  WT,
	pub suffix_non_empty: WR,

	pub value_empty:  WT,
	pub suffix_empty: WR,

	pub prefix_args: AL,
	pub value_args:  AT,
	pub suffix_args: AR,
}

impl<AL, AT, AR>
	FmtArgs<
		fn(&mut Whitespace, &mut rustidy_format::Context),
		fn(&mut Whitespace, &mut rustidy_format::Context),
		AL,
		AT,
		AR,
	>
{
	#[must_use]
	pub const fn preserve(prefix_args: AL, value_args: AT, suffix_args: AR) -> Self {
		Self {
			value_non_empty: Whitespace::preserve,
			suffix_non_empty: Whitespace::preserve,

			value_empty: Whitespace::preserve,
			suffix_empty: Whitespace::preserve,

			prefix_args,
			value_args,
			suffix_args,
		}
	}

	#[must_use]
	pub const fn single_if_non_blank(prefix_args: AL, value_args: AT, suffix_args: AR) -> Self {
		Self {
			value_non_empty: Whitespace::set_single,
			suffix_non_empty: Whitespace::set_single,

			value_empty: Whitespace::remove,
			suffix_empty: Whitespace::set_single,

			prefix_args,
			value_args,
			suffix_args,
		}
	}

	#[must_use]
	pub const fn indent_if_non_blank(prefix_args: AL, value_args: AT, suffix_args: AR) -> Self {
		Self {
			value_non_empty: Whitespace::set_cur_indent,
			suffix_non_empty: Whitespace::set_prev_indent,

			value_empty: Whitespace::remove,
			suffix_empty: Whitespace::set_prev_indent_remove_if_pure,

			prefix_args,
			value_args,
			suffix_args,
		}
	}

	#[must_use]
	pub const fn remove(prefix_args: AL, value_args: AT, suffix_args: AR) -> Self {
		Self {
			value_non_empty: Whitespace::remove,
			suffix_non_empty: Whitespace::remove,

			value_empty: Whitespace::remove,
			suffix_empty: Whitespace::remove,

			prefix_args,
			value_args,
			suffix_args,
		}
	}
}
