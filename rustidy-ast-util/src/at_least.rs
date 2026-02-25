//! At least N

// Imports
use {
	itertools::chain,
	rustidy_format::{Format, Formattable, WhitespaceConfig},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args = FmtArgs)]
// TODO: Support arguments for `T`
pub struct AtLeast1<T> {
	pub first: T,
	#[format(prefix_ws = args.rest_prefix_ws)]
	#[format(args = rustidy_format::vec::args_prefix_ws(args.rest_prefix_ws))]
	pub rest:  Vec<T>,
}

impl<T> AtLeast1<T> {
	pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
		self.into_iter()
	}
}

impl<T> IntoIterator for AtLeast1<T> {
	type Item = T;

	type IntoIter = impl Iterator<Item = Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		chain![[self.first], self.rest]
	}
}

impl<'a, T> IntoIterator for &'a AtLeast1<T> {
	type Item = &'a T;

	type IntoIter = impl Iterator<Item = Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		chain![[&self.first], &self.rest]
	}
}

/// Formatting arguments
#[derive(Clone, Copy, Debug)]
pub struct FmtArgs {
	pub rest_prefix_ws: WhitespaceConfig,
}

#[must_use]
pub const fn fmt_prefix_ws(rest_prefix_ws: WhitespaceConfig) -> FmtArgs {
	FmtArgs { rest_prefix_ws }
}
