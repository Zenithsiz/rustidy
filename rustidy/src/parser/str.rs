//! Parse string

// Imports
use crate::{parser::ParserRange, print::Print};

/// Parser string
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[must_use = "Parser output must not be discarded"]
pub struct ParserStr(pub(super) ParserRange);

impl Print for ParserStr {
	fn print(&self, f: &mut crate::PrintFmt) {
		f.write_str(self);
	}
}
