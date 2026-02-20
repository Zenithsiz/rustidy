//! Ast string

// Imports
use {
	crate::{ArenaData, ArenaIdx, AstRange},
	std::{borrow::Cow, sync::Arc},
	serde::ser::Error,
};

/// Ast string
// TODO: Add an "empty" position for newly created ast nodes?
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[must_use = "Ast output must not be discarded"]
pub struct AstStr(ArenaIdx<Inner>);

impl AstStr {
	/// Creates a new ast string without any associated input range
	pub fn new(repr: impl Into<AstStrRepr>) -> Self {
		Self(ArenaIdx::new(Inner { repr: repr.into(), input_range: None, }))
	}

	/// Creates a new ast string from an input range
	pub fn from_input(input_range: AstRange) -> Self {
		Self(ArenaIdx::new(Inner {
			repr: AstStrRepr::AstRange(input_range),
			input_range: Some(input_range),
		}))
	}

	/// Replaces this ast string
	pub fn replace(&mut self, repr: impl Into<AstStrRepr>) {
		self.0.repr = repr.into();
	}

	/// Joins two strings
	pub fn join(self, other: Self) -> Self {
		let input_range = match (self.0.input_range, other.0.input_range) {
			// TODO: Keep the range more than just when contiguous?
			(Some(lhs), Some(rhs)) if lhs.end == rhs.start => Some(AstRange { start: lhs.start, end: rhs.end }),
			_ => None,
		};

		Self(ArenaIdx::new(Inner {
			repr: AstStrRepr::Join { lhs: self, rhs: other },
			input_range
		}))
	}

	/// Returns the inner representation of this string
	#[must_use]
	pub fn repr(&self) -> &AstStrRepr {
		&self.0.repr
	}

	/// Returns the input range of this string
	#[must_use]
	pub fn input_range(&self) -> Option<AstRange> {
		self.0.input_range
	}

	/// Returns the length of this string
	#[must_use]
	pub fn len(&self) -> usize {
		self.repr().len()
	}

	/// Returns if this string is empty
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.repr().is_empty()
	}

	/// Returns if this string is blank
	#[must_use]
	pub fn is_blank(&self, input: &str) -> bool {
		self.repr().is_blank(input)
	}

	/// Returns the number of newlines in this string
	#[must_use]
	pub fn count_newlines(&self, input: &str) -> usize {
		self.repr().count_newlines(input)
	}

	/// Returns if this string has newlines
	#[must_use]
	pub fn has_newlines(&self, input: &str) -> bool {
		self.repr().has_newlines(input)
	}

	/// Returns if this string is equal to `other`
	#[must_use]
	pub fn is_str(&self, input: &str, other: &str) -> bool {
		self.repr().is_str(input, other)
	}

	/// Writes this string
	pub fn write(&self, input: &str, output: &mut String) {
		self.repr().write(input, output);
	}

	/// Returns the string of this string, if it comes from the input (or is static)
	#[must_use]
	pub fn str_input<'input>(&self, input: &'input str) -> Option<&'input str> {
		self.repr().str_input(input)
	}

	/// Returns the string of this string, if it's cheap to get
	#[must_use]
	pub fn str_cheap<'a>(&'a self, input: &'a str) -> Option<&'a str> {
		self.repr().str_input(input)
	}

	/// Returns the string of this string
	// TODO: This can be somewhat expensive, should we replace it with
	//       functions performing whatever checks the callers need instead?
	#[must_use]
	pub fn str<'a>(&'a self, input: &'a str) -> Cow<'a, str> {
		match self.0.repr.str_cheap(input) {
			Some(s) => s.into(),
			None => self.repr().str(input).into_owned().into(),
		}
	}
}

// TODO: Make serialization not lossy?
impl serde::Serialize for AstStr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer
	{
		match self.input_range() {
			Some(input_range) => input_range.serialize(serializer),
			None => Err(S::Error::custom(format!("Only strings with an input range may be serialized, found {:?}", *self.0))),
		}
	}
}

impl<'de> serde::Deserialize<'de> for AstStr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let range = AstRange::deserialize(deserializer)?;
		Ok(Self(ArenaIdx::new(Inner {
			repr: AstStrRepr::AstRange(range),
			input_range: Some(range)
		})))
	}
}


#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(ArenaData)]
#[derive(derive_more::From)]
struct Inner {
	repr:        AstStrRepr,
	input_range: Option<AstRange>,
}

/// Ast string representation
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(ArenaData)]
#[derive(derive_more::From)]
pub enum AstStrRepr {
	/// Input range
	#[from]
	AstRange(AstRange),

	/// Static string
	#[from]
	String(&'static str),

	/// Single character
	#[from]
	Char(char),

	/// Spaces
	Spaces {
		len: u16,
	},

	/// Indentation
	#[from]
	Indentation {
		indent:   Arc<str>,
		newlines: usize,
		depth:    usize,
	},

	/// Joined strings
	Join {
		lhs: AstStr,
		rhs: AstStr,
	},

	// Dynamic string
	// Note: Not `#[from]` to make it clear this is expensive
	Dynamic(String),
}

impl AstStrRepr {
	/// Returns the length of this representation
	#[must_use]
	pub fn len(&self) -> usize {
		match *self {
			Self::AstRange(range) => range.len(),
			Self::String(s) => s.len(),
			Self::Char(ch) => ch.len_utf8(),
			Self::Spaces {
				len
			} => usize::from(len),
			Self::Indentation {
				ref indent,
				newlines,
				depth,
			} => newlines + depth * indent.len(),
			Self::Join {
				ref lhs,
				ref rhs
			} => lhs.len() + rhs.len(),
			Self::Dynamic(ref s) => s.len(),
		}
	}

	/// Returns if this representation is empty
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Returns if this repr has the same string as another
	#[must_use]
	pub fn is_str_eq_to(&self, other: &Self, input: &str) -> bool {
		match (self, other) {
			// If they're equal, we're done
			(lhs, rhs) if lhs == rhs => true,

			// If one of them can be cheaply represented as a string, compare it.
			(lhs, rhs) if let Some(lhs) = lhs.str_cheap(input) => rhs.is_str(input, lhs),
			(lhs, rhs) if let Some(rhs) = rhs.str_cheap(input) => lhs.is_str(input, rhs),

			// Otherwise, turn one of them into a string and compare.
			// TODO: This is expensive, ensure we only get
			//       here for very rare representations
			_ => self.is_str(input, &other.str(input)),
		}
	}

	/// Returns if this representation is blank
	#[must_use]
	pub fn is_blank(&self, input: &str) -> bool {
		match *self {
			Self::AstRange(range) => crate::is_str_blank(range.str(input)),
			Self::String(s) => crate::is_str_blank(s),
			Self::Char(ch) => ch.is_ascii_whitespace(),
			Self::Spaces {
				..
			} | Self::Indentation {
				..
			} => true,
			Self::Join {
				ref lhs,
				ref rhs
			} => lhs.is_blank(input) && rhs.is_blank(input),
			Self::Dynamic(ref s) => crate::is_str_blank(s),
		}
	}

	/// Returns the number of newlines in this string
	#[must_use]
	pub fn count_newlines(&self, input: &str) -> usize {
		match *self {
			Self::AstRange(range) => crate::str_count_newlines(range.str(input)),
			Self::String(s) => crate::str_count_newlines(s),
			Self::Char(ch) => match ch == '\n' {
				true => 1,
				false => 0,
			},
			Self::Spaces {
				..
			} => 0,
			Self::Indentation {
				newlines,
				..
			} => newlines,
			Self::Join {
				ref lhs,
				ref rhs
			} => lhs.count_newlines(input) + rhs.count_newlines(input),
			Self::Dynamic(ref s) => crate::str_count_newlines(s),
		}
	}

	/// Returns if this representation has newlines
	#[must_use]
	pub fn has_newlines(&self, input: &str) -> bool {
		self.count_newlines(input) == 0
	}

	/// Returns if this representation is equal to `other`
	#[must_use]
	pub fn is_str(&self, input: &str, other: &str) -> bool {
		match *self {
			Self::AstRange(range) => range.str(input) == other,
			Self::String(s) => s == other,
			Self::Dynamic(ref s) => s == other,

			// TODO: Properly implement these to avoid allocating a string
			_ => self.str(input) == other,
		}
	}

	/// Writes this representation
	pub fn write(&self, input: &str, output: &mut String) {
		match *self {
			Self::AstRange(range) => output.push_str(range.str(input)),
			Self::String(s) => output.push_str(s),
			Self::Char(ch) => output.push(ch),
			Self::Spaces {
				len
			} => for _ in 0..len {
				output.push(' ');
			},
			Self::Indentation {
				ref indent,
				newlines,
				depth,
			} => {
				for _ in 0..newlines {
					output.push('\n');
				}
				for _ in 0..depth {
					output.push_str(indent);
				}
			},
			Self::Join {
				ref lhs,
				ref rhs
			} => {
				lhs.write(input, output);
				rhs.write(input, output);
			},
			Self::Dynamic(ref s) => output.push_str(s),
		}
	}

	/// Returns the string of this representation, if it comes from the input (or is static).
	#[must_use]
	pub fn str_input<'input>(&self, input: &'input str) -> Option<&'input str> {
		match self {
			Self::AstRange(range) => Some(range.str(input)),
			Self::String(s) => Some(s),

			// Special case these to avoid a `String` allocation
			Self::Spaces {
				len: 0
			} => "".into(),
			Self::Spaces {
				len: 1
			} => " ".into(),

			_ => None,
		}
	}

	/// Returns the string of this representation, if it's cheap to get it
	#[must_use]
	pub fn str_cheap<'a>(&'a self, input: &'a str) -> Option<&'a str> {
		match self.str_input(input) {
			Some(s) => Some(s),
			None => match self {
				Self::Dynamic(s) => Some(s),
				_ => None,
			},
		}
	}

	/// Returns a string of this representation
	// TODO: This can be somewhat expensive, should we replace it with
	//       functions performing whatever checks the callers need instead?
	#[must_use]
	pub fn str<'a>(&'a self, input: &'a str) -> Cow<'a, str> {
		match self.str_cheap(input) {
			Some(s) => s.into(),
			None => {
				let mut output = String::new();
				self.write(input, &mut output);
				output.into()
			},
		}
	}
}
