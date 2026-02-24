//! Ast string

// Imports
use {
	crate::{ArenaData, ArenaIdx, StrChunk},
	arcstr::Substr,
	std::{borrow::Cow, sync::Arc},
};

/// Ast string
// TODO: Add an "empty" position for newly created ast nodes?
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[must_use = "Ast output must not be discarded"]
pub struct AstStr(ArenaIdx<Inner>);

impl AstStr {
	/// Creates a new ast string without any associated input range
	pub fn new(repr: impl Into<AstStrRepr>) -> Self {
		Self(
			ArenaIdx::new(Inner { repr: repr.into(), input: None, })
		)
	}

	/// Creates a new ast string from an input range
	pub fn from_input(input: Substr) -> Self {
		Self(
			ArenaIdx::new(
				Inner {
					repr: AstStrRepr::String(Substr::clone(&input)),
					input: Some(input),
				}
			)
		)
	}

	/// Replaces this ast string
	pub fn replace(&mut self, repr: impl Into<AstStrRepr>) {
		self.0.repr = repr.into();
	}

	/// Joins two strings
	pub fn join(self, other: Self) -> Self {
		let input = match (&self.0.input, &other.0.input) {
			// TODO: Keep the range more than just when contiguous?
			(Some(lhs), Some(rhs)) => {
				let input = lhs.parent();
				let lhs = lhs.range();
				let rhs = rhs.range();

				match lhs.end == rhs.start {
					true => Some(input.substr(lhs.start..rhs.end)),
					false => None,
				}
			},
			_ => None,
		};

		Self(
			ArenaIdx::new(
				Inner {
					repr: AstStrRepr::Join { lhs: self, rhs: other },
					input
				}
			)
		)
	}

	/// Returns the inner representation of this string
	#[must_use]
	pub fn repr(&self) -> &AstStrRepr {
		&self.0.repr
	}

	/// Returns the input range of this string
	#[must_use]
	pub fn input(&self) -> Option<&Substr> {
		self.0.input.as_ref()
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
	pub fn is_blank(&self) -> bool {
		self.repr().is_blank()
	}

	/// Returns the number of newlines in this string
	#[must_use]
	pub fn count_newlines(&self) -> usize {
		self.repr().count_newlines()
	}

	/// Returns if this string has newlines
	#[must_use]
	pub fn has_newlines(&self) -> bool {
		self.repr().has_newlines()
	}

	/// Returns if this string is equal to `other`
	#[must_use]
	pub fn is_str(&self, other: &str) -> bool {
		self.repr().is_str(other)
	}

	/// Writes this string
	pub fn write(&self, output: &mut String) {
		self.repr().write(output);
	}

	/// Returns the string of this string, if it's cheap to get
	#[must_use]
	pub fn str_cheap(&self) -> Option<&str> {
		self.repr().str_cheap()
	}

	/// Returns the string of this string
	#[must_use]
	pub fn str(&self) -> Cow<'_, str> {
		self.repr().str()
	}
}

// TODO: Make serialization not lossy?
impl serde::Serialize for AstStr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer
	{
		self.str().serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for AstStr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		Ok(Self::new(AstStrRepr::String(s.into())))
	}
}


#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(ArenaData)]
#[derive(derive_more::From)]
struct Inner {
	repr:  AstStrRepr,
	input: Option<Substr>,
}

/// Ast string representation
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(ArenaData)]
#[derive(derive_more::From)]
pub enum AstStrRepr {
	/// String
	#[from]
	String(Substr),

	/// Static string
	// TODO: Merge this with `Self::String`?
	#[from]
	Static(&'static str),

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
}

impl AstStrRepr {
	/// Returns the length of this representation
	#[must_use]
	pub fn len(&self) -> usize {
		match *self {
			Self::String(ref s) => s.len(),
			Self::Static(s) => s.len(),
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
		}
	}

	/// Returns if this representation is empty
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Returns if this repr has the same string as another
	#[must_use]
	pub fn is_str_eq_to(&self, other: &Self) -> bool {
		match (self, other) {
			// If they're equal, we're done
			(lhs, rhs) if lhs == rhs => true,

			// If one of them can be cheaply represented as a string, compare it.
			(lhs, rhs) if let Some(lhs) = lhs.str_cheap() => rhs.is_str(lhs),
			(lhs, rhs) if let Some(rhs) = rhs.str_cheap() => lhs.is_str(rhs),

			// Otherwise, turn one of them into a string and compare.
			// TODO: This is expensive, ensure we only get
			//       here for very rare representations
			_ => self.is_str(&other.str()),
		}
	}

	/// Returns if this representation is blank
	#[must_use]
	pub fn is_blank(&self) -> bool {
		match *self {
			Self::String(ref s) => crate::is_str_blank(s),
			Self::Static(s) => crate::is_str_blank(s),
			Self::Char(ch) => ch.is_ascii_whitespace(),
			Self::Spaces {
				..
			} | Self::Indentation {
				..
			} => true,
			Self::Join {
				ref lhs,
				ref rhs
			} => lhs.is_blank() && rhs.is_blank(),
		}
	}

	/// Returns the number of newlines in this string
	#[must_use]
	pub fn count_newlines(&self) -> usize {
		match *self {
			Self::String(ref s) => crate::str_count_newlines(s),
			Self::Static(s) => crate::str_count_newlines(s),
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
			} => lhs.count_newlines() + rhs.count_newlines(),
		}
	}

	/// Returns if this representation has newlines
	#[must_use]
	pub fn has_newlines(&self) -> bool {
		self.count_newlines() != 0
	}

	/// Returns if this representation is equal to `other`
	#[must_use]
	pub fn is_str(&self, other: &str) -> bool {
		match *self {
			Self::String(ref s) => s == other,
			Self::Static(s) => s == other,

			Self::Char(ch) => {
				let mut other = other.chars();
				if other.next() != Some(ch) {
					return false;
				}

				other.next().is_none()
			},

			Self::Spaces {
				len
			} => other.len() == usize::from(len) && other.chars().all(|ch| ch == ' '),

			Self::Indentation {
				ref indent,
				newlines,
				depth
			} => other.len() == newlines + depth && other[..newlines].chars().all(|ch| ch == '\n') && other[newlines..]
				.chunk(indent.len())
				.all(|other_indent| other_indent == other),

			Self::Join {
				ref lhs,
				ref rhs
			} => {
				let Some((lhs_other, rhs_other)) = other.split_at_checked(lhs.len()) else {
					return false;
				};
				lhs.is_str(lhs_other) && rhs.is_str(rhs_other)
			},
		}
	}

	/// Writes this representation
	pub fn write(&self, output: &mut String) {
		match *self {
			Self::String(ref s) => output.push_str(s),
			Self::Static(s) => output.push_str(s),
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
				lhs.write(output);
				rhs.write(output);
			},
		}
	}

	/// Returns the string of this representation, if it's cheap to get it
	#[must_use]
	pub fn str_cheap(&self) -> Option<&str> {
		match self {
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

	/// Returns a string of this representation
	// TODO: This can be somewhat expensive, should we replace it with
	//       functions performing whatever checks the callers need instead?
	#[must_use]
	pub fn str(&self) -> Cow<'_, str> {
		match self.str_cheap() {
			Some(s) => s.into(),
			None => {
				let mut output = String::new();
				self.write(&mut output);
				output.into()
			},
		}
	}
}
