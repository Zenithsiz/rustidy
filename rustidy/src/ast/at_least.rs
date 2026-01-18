//! At least N

// Imports
use {
	crate::{Format, format::FormatFn},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AtLeast1<T> {
	pub first: T,
	pub rest:  Vec<T>,
}

/// Formats all non-first elements of `AtLeast1<T>`
pub fn format<T>(f: impl FormatFn<T>) -> impl FormatFn<AtLeast1<T>> {
	move |at_least, ctx| {
		for value in &mut at_least.rest {
			f(value, ctx);
		}
	}
}
