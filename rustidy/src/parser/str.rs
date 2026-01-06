//! Parse string

// Imports
use crate::{Format, Print, ast::whitespace::Whitespace, format};

/// Parser string index
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct ParserStrIdx(pub u32);

/// Parser string
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[must_use = "Parser output must not be discarded"]
pub struct ParserStr(pub(super) ParserStrIdx);

impl Format for ParserStr {
	fn range(&mut self, ctx: &mut format::Context) -> Option<super::ParserRange> {
		Some(ctx.parser().str_range(*self))
	}

	fn len(&mut self, ctx: &mut format::Context) -> usize {
		ctx.parser().str_range(*self).len()
	}

	fn format(&mut self, _ctx: &mut format::Context) {}

	fn prefix_ws(&mut self, _ctx: &mut format::Context) -> Option<&mut Whitespace> {
		None
	}
}

impl Print for ParserStr {
	fn print(&self, f: &mut crate::PrintFmt) {
		f.write_str(*self);
	}
}
