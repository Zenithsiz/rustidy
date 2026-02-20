//! Ast string formatting

// Imports
use {crate::{Context, FormatOutput}, rustidy_util::AstStr};

#[extend::ext(name = AstStrFormat)]
pub impl AstStr {
	/// Gets the formatting output of this string
	fn format_output(&self, ctx: &mut Context) -> FormatOutput {
		FormatOutput {
			prefix_ws_len: None,
			len: self.len(),
			is_empty: self.is_empty(),
			is_blank: self.is_blank(ctx.input),
			has_newlines: self.has_newlines(ctx.input),
		}
	}
}
