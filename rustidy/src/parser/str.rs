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
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[must_use = "Parser output must not be discarded"]
pub struct ParserStr(pub(super) ArenaIdx<Self>);

impl ParserStr {
	/// Returns the parser range of this string
	#[must_use]
	pub fn range(&self, arenas: &Arenas) -> ParserRange {
		arenas.get::<Self>().with_value(self.0, |range| *range)
	}
}

impl ArenaData for ParserStr {
	type Data = ParserRange;
}

impl FormatRef for ParserStr {
	fn range(&self, ctx: &format::Context) -> Option<super::ParserRange> {
		Some(Self::range(self, ctx.arenas()))
	}

	fn len(&self, ctx: &format::Context) -> usize {
		Self::range(self, ctx.arenas()).len()
	}
}

impl Format for ParserStr {
	fn format(&mut self, _ctx: &mut format::Context) {}

	fn with_prefix_ws(
		&mut self,
		_ctx: &mut format::Context,
		_f: impl Fn(&mut Whitespace, &mut format::Context),
	) -> bool {
		false
	}
}

impl Print for ParserStr {
	fn print(&self, f: &mut crate::PrintFmt) {
		f.write_str(*self);
	}
}
