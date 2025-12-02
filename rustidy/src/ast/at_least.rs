//! At least N

// Imports
use crate::{Format, Parse, Print};

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AtLeast1<T> {
	pub first: T,
	pub rest:  Vec<T>,
}
