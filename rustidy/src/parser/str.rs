//! Parse string

// Imports
use {
	super::{ParserPos, ParserRange},
	crate::{
		Format,
		FormatRef,
		Print,
		arena::{Arena, ArenaData, ArenaIdx},
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
	/// Creates a new parser string from a range
	pub fn new(range: ParserRange) -> Self {
		Self(ARENA.push(range))
	}

	/// Creates a new 0-sized parser string from a position
	pub fn empty_at(pos: ParserPos) -> Self {
		Self::new(ParserRange { start: pos, end: pos })
	}

	/// Returns the parser range of this string
	#[must_use]
	pub fn range(&self) -> ParserRange {
		*ARENA.get(&self.0)
	}
}

impl ArenaData for ParserStr {
	type Data = ParserRange;

	const ARENA: &'static Arena<Self> = &ARENA;
}

static ARENA: Arena<ParserStr> = Arena::new();

impl FormatRef for ParserStr {
	fn input_range(&self, _ctx: &format::Context) -> Option<super::ParserRange> {
		Some(Self::range(self))
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
