//! Ast string formatting

// Imports
use {crate::{Context, FormatOutput}, rustidy_util::AstStr};

#[extend::ext(name = AstStrFormat)]
pub impl AstStr {
	/// Gets the formatting output of this string
	fn format_output(&self, ctx: &mut Context) -> FormatOutput {
		// TODO: Optimize these by not iterating over the string multiple times.
		FormatOutput {
			prefix_ws_len: None,
			len: self.len(),
			newlines: self.count_newlines(ctx.input),
			is_empty: self.is_empty(),
			is_blank: self.is_blank(ctx.input),
		}
	}
}
