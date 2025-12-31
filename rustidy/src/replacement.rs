//! String replacements

// Imports
use {
	crate::{Parser, ParserStr},
	std::collections::HashMap,
};

/// String replacements
pub struct Replacements {
	replacements: HashMap<ParserStr, Replacement>,
}

impl Replacements {
	/// Creates new, empty, replacements
	#[must_use]
	pub fn new() -> Self {
		Self {
			replacements: HashMap::new(),
		}
	}

	/// Adds a replacement
	pub fn add(&mut self, parser: &Parser, s: ParserStr, replacement: impl Into<Replacement>) {
		let replacement = replacement.into();
		match replacement.is(parser.str(s)) {
			true => _ = self.replacements.remove(&s),
			false => _ = self.replacements.insert(s, replacement),
		}
	}

	/// Returns the replacement of a string
	#[must_use]
	pub fn get(&self, s: ParserStr) -> Option<&Replacement> {
		self.replacements.get(&s)
	}
}

impl Default for Replacements {
	fn default() -> Self {
		Self::new()
	}
}

/// String replacement
#[derive(Debug)]
#[derive(derive_more::From)]
pub enum Replacement {
	Static(&'static str),
	// TODO: Replace this with other variants to avoid allocations
	Dynamic(String),
}

impl Replacement {
	/// Returns if this replacement is equal to `s`
	#[must_use]
	pub fn is(&self, s: &str) -> bool {
		match self {
			Self::Static(replacement) => *replacement == s,
			Self::Dynamic(replacement) => replacement == s,
		}
	}

	/// Writes this replacement onto a string
	pub fn write(&self, output: &mut String) {
		match self {
			Self::Static(replacement) => *output += replacement,
			Self::Dynamic(replacement) => *output += replacement,
		}
	}
}
