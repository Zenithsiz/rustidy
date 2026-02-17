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
		let s = self.0.get();
		FormatOutput {
			prefix_ws_len: None,
			len:           s.len(),
			is_empty:      s.is_empty(),
			has_newlines:  s.has_newlines(ctx.input),
		}
	}
}
