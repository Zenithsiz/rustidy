//! Parse string

// Imports
use {
	super::ParserRange,
	crate::{
		Arenas,
		Format,
		Print,
		arena::{ArenaData, ArenaIdx},
		ast::whitespace::Whitespace,
		format,
	},
};

/// Parser string
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[must_use = "Parser output must not be discarded"]
pub struct ParserStr(pub(super) ArenaIdx<Self>);

impl ParserStr {
	/// Returns the parser range of this string
	#[must_use]
	pub fn range(&self, arenas: &Arenas) -> ParserRange {
		*arenas.get::<Self>().get(self.0)
	}
}

impl ArenaData for ParserStr {
	type Data = ParserRange;
}

impl Format for ParserStr {
	fn range(&mut self, ctx: &mut format::Context) -> Option<super::ParserRange> {
		Some(Self::range(self, ctx.arenas()))
	}

	fn len(&mut self, ctx: &mut format::Context) -> usize {
		Self::range(self, ctx.arenas()).len()
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
