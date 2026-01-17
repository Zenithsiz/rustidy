//! Parse string

// Imports
use {
	super::ParserRange,
	crate::{
		Arenas,
		Format,
		FormatRef,
		Print,
		arena::{ArenaData, ArenaIdx},
		ast::whitespace::Whitespace,
		format,
	},
};

/// Parser string
#[derive(PartialEq, Eq, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[must_use = "Parser output must not be discarded"]
pub struct ParserStr(pub ArenaIdx<Self>);

impl ParserStr {
	/// Returns the parser range of this string
	#[must_use]
	pub fn range(&self, arenas: &Arenas) -> ParserRange {
		*arenas.get(&self.0)
	}
}

impl ArenaData for ParserStr {
	type Data = ParserRange;
}

impl FormatRef for ParserStr {
	fn input_range(&self, ctx: &format::Context) -> Option<super::ParserRange> {
		Some(Self::range(self, ctx.arenas()))
	}
}

impl Format for ParserStr {
	fn format(&mut self, _ctx: &mut format::Context) {}

	fn with_prefix_ws<R, F: Fn(&mut Whitespace, &mut format::Context) -> R + Copy>(
		&mut self,
		_ctx: &mut format::Context,
		_f: F,
	) -> Option<R> {
		None
	}
}

impl Print for ParserStr {
	fn print(&self, f: &mut crate::PrintFmt) {
		f.write_str(self);
	}
}
