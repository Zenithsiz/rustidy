//! Ast String

// Imports
use crate::{
	Parser,
	Print,
	parser::{ParserPos, ParserRange},
};

/// Ast string
#[derive(PartialEq, Eq, Clone, Debug)]
#[must_use = "Parser output must not be discarded"]
pub struct AstStr(ParserRange);

impl AstStr {
	/// Creates a new ast string
	pub const fn new(range: ParserRange) -> Self {
		Self(range)
	}

	/// Returns the length of this string
	#[must_use]
	pub const fn len(&self) -> usize {
		self.0.len()
	}

	/// Returns if this string is empty
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Returns the range of this string
	#[must_use]
	pub const fn range(&self) -> ParserRange {
		self.0
	}

	/// Returns the inner string of this
	#[must_use]
	pub fn as_str<'a>(&self, parser: &Parser<'a>) -> &'a str {
		parser.str(self)
	}
}

impl Print for AstStr {
	fn print(&self, f: &mut crate::PrintFmt) {
		f.write_str(self);
	}
}

impl serde::Serialize for AstStr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let repr = SerdeRepr(self.0.start.to_usize(), self.0.end.to_usize());

		repr.serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for AstStr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let repr = SerdeRepr::deserialize(deserializer)?;
		let range = ParserRange::new(ParserPos::from_usize(repr.0), ParserPos::from_usize(repr.1));

		Ok(Self(range))
	}
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SerdeRepr(usize, usize);
