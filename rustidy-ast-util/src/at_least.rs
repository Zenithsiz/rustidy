//! At least N

// Imports
use {
	rustidy_format::{Format, WsFmtFn},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(args(ty = "FmtArgs<W>", generic = "W: WsFmtFn"))]
#[format(where_format = "where T: Format<()>")]
// TODO: Support arguments for `T`
pub struct AtLeast1<T> {
	pub first: T,
	#[format(args = rustidy_format::vec::args_prefix_ws(&args.rest_prefix_ws))]
	pub rest:  Vec<T>,
}

/// Formatting arguments
pub struct FmtArgs<W> {
	pub rest_prefix_ws: W,
}

impl<W> FmtArgs<W> {
	pub const fn from_prefix_ws(rest_prefix_ws: W) -> Self {
		Self { rest_prefix_ws }
	}
}
