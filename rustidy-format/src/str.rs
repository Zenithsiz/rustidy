//! Ast string formatting

// Imports
use {
	crate::{Context, FormatOutput},
	rustidy_util::AstStr,
};

#[extend::ext(name = AstStrFormat)]
pub impl AstStr {
	/// Gets the formatting output of this string
	fn format_output(&self, ctx: &mut Context) -> FormatOutput {
		FormatOutput {
			prefix_ws_len: None,
			len:           self.0.len(),
			is_empty:      self.0.is_empty(),
			is_blank:      self.0.is_blank(ctx.input),
			has_newlines:  self.0.has_newlines(ctx.input),
		}
	}
}
