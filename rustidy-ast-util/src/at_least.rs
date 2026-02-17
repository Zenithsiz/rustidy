//! At least N

// Imports
use {
	rustidy_format::{Format, Formattable, WhitespaceConfig},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args = FmtArgs)]
// TODO: Support arguments for `T`
pub struct AtLeast1<T> {
	pub first: T,
	#[format(args = rustidy_format::vec::args_prefix_ws(args.rest_prefix_ws))]
	pub rest:  Vec<T>,
}

/// Formatting arguments
pub struct FmtArgs {
	pub rest_prefix_ws: WhitespaceConfig,
}

#[must_use]
pub const fn fmt_prefix_ws(rest_prefix_ws: WhitespaceConfig) -> FmtArgs {
	FmtArgs { rest_prefix_ws }
}
