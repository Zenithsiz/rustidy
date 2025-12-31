//! Ast String

// TODO: Currently we box the inner representation because
//       the replacement field is pretty big (16 bytes).
//       If we could move it elsewhere, we could become 8
//       bytes.

// Imports
use {
	crate::{
		Parser,
		Print,
		parser::{ParserPos, ParserRange},
	},
	std::borrow::Cow,
};

#[derive(PartialEq, Eq, Clone, Debug)]
struct Inner {
	/// Range
	range: ParserRange,

	/// A replacement for this string
	replacement: Option<Cow<'static, str>>,
}

/// Ast string
#[derive(PartialEq, Eq, Clone, Debug)]
#[must_use = "Parser output must not be discarded"]
pub struct AstStr(Box<Inner>);

impl AstStr {
	/// Creates a new ast string
	pub fn new(range: ParserRange) -> Self {
		Self(Box::new(Inner {
			range,
			replacement: None,
		}))
	}

	/// Creates a new ast string with a replacement
	pub fn from_string(range: ParserRange, replacement: impl Into<Cow<'static, str>>) -> Self {
		Self(Box::new(Inner {
			range,
			replacement: Some(replacement.into()),
		}))
	}

	/// Returns the length of this string
	#[must_use]
	pub fn len(&self) -> usize {
		self.0.range.len()
	}

	/// Returns if this string is empty
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Returns the range of this string
	#[must_use]
	pub fn range(&self) -> ParserRange {
		self.0.range
	}

	/// Sets the replacement of this string
	pub fn replace(&mut self, replacement: impl Into<Cow<'static, str>>) {
		self.0.replacement = Some(replacement.into());
	}

	/// Returns the inner string of this
	#[must_use]
	pub fn as_str<'a>(&'a self, parser: &Parser<'a>) -> &'a str {
		match &self.0.replacement {
			Some(s) => s,
			None => parser.str(self),
		}
	}
}

impl Print for AstStr {
	fn print(&self, f: &mut crate::PrintFmt) {
		let s = match &self.0.replacement {
			Some(replacement) => &**replacement,
			None => f.parser().str(self),
		};
		f.write_str(s);
	}
}

impl serde::Serialize for AstStr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let repr = match &self.0.replacement {
			Some(replacement) =>
				SerdeRepr::WithReplacement(self.0.range.start.to_usize(), self.0.range.end.to_usize(), replacement),
			None => SerdeRepr::Basic(self.0.range.start.to_usize(), self.0.range.end.to_usize()),
		};

		repr.serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for AstStr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let (range, replacement) = match SerdeRepr::deserialize(deserializer)? {
			SerdeRepr::WithReplacement(start, end, replacement) => (
				ParserRange::new(ParserPos::from_usize(start), ParserPos::from_usize(end)),
				Some(replacement),
			),
			SerdeRepr::Basic(start, end) => (
				ParserRange::new(ParserPos::from_usize(start), ParserPos::from_usize(end)),
				None,
			),
		};

		Ok(Self(Box::new(Inner { range, replacement })))
	}
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum SerdeRepr<S> {
	WithReplacement(usize, usize, S),
	Basic(usize, usize),
}
