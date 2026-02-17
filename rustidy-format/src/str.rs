//! Ast string formatting

// Imports
use {crate::FormatOutput, rustidy_util::AstStr};

#[extend::ext(name = AstStrFormat)]
pub impl AstStr {
	/// Gets the formatting output of this string
	fn format_output(&self) -> FormatOutput {
		let s = self.0.get();
		FormatOutput { is_empty: s.is_empty() }
	}
}
