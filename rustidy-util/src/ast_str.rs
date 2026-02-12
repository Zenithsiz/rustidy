//! Ast string

// Imports
use {
	crate::{Arena, ArenaData, ArenaIdx, ArenaRef, AstRange},
	std::{borrow::Cow, sync::Arc},
};

/// Ast string
// TODO: Add an "empty" position for newly created ast nodes?
#[derive(PartialEq, Eq, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[must_use = "Ast output must not be discarded"]
pub struct AstStr(pub ArenaIdx<Self>);

impl AstStr {
	/// Creates a new ast string
	pub fn new(repr: impl Into<AstStrRepr>) -> Self {
		Self(ArenaIdx::new(repr.into()))
	}

	/// Replaces this ast string
	pub fn replace(&mut self, input: &str, new_repr: impl Into<AstStrRepr>) {
		let mut cur_repr = self.0.get_mut();
		let new_repr = new_repr.into();

		// TODO: Should we only check if it's cheap to do so?
		if !cur_repr.is_str_eq_to(&new_repr, input) {
			*cur_repr = new_repr;
		}
	}

	/// Joins two strings
	pub fn join(self, other: Self) -> Self {
		Self(ArenaIdx::new(AstStrRepr::Join { lhs: self, rhs: other }))
	}

	/// Returns the inner representation of this string
	#[must_use]
	pub fn repr(&self) -> ArenaRef<'_, Self> {
		self.0.get()
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

	/// Returns the string of this string
	// TODO: This can be somewhat expensive, should we replace it with
	//       functions performing whatever checks the callers need instead?
	#[must_use]
	pub fn str<'input>(&self, input: &'input str) -> Cow<'input, str> {
		let repr = self.repr();
		match repr.str_input(input) {
			Some(s) => s.into(),
			None => repr.str(input).into_owned().into(),
		}
	}
}

impl ArenaData for AstStr {
	type Data = AstStrRepr;

	const ARENA: &'static Arena<Self> = &ARENA;
}

static ARENA: Arena<AstStr> = Arena::new();

#[derive(PartialEq, Eq, Debug)]
#[derive(derive_more::From)]
#[derive(serde::Serialize)]
#[serde(untagged)]
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
		len: usize,
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
			Self::Spaces { len } => len,
			Self::Indentation {
				ref indent,
				newlines,
				depth,
			} => newlines + depth * indent.len(),
			Self::Join { ref lhs, ref rhs } => lhs.len() + rhs.len(),
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
			Self::Spaces { .. } | Self::Indentation { .. } => true,
			Self::Join { ref lhs, ref rhs } => lhs.is_blank(input) && rhs.is_blank(input),
			Self::Dynamic(ref s) => crate::is_str_blank(s),
		}
	}

	/// Returns if this representation has newlines
	#[must_use]
	pub fn has_newlines(&self, input: &str) -> bool {
		match *self {
			Self::AstRange(range) => range.str(input).contains('\n'),
			Self::String(s) => s.contains('\n'),
			Self::Char(ch) => ch == '\n',
			Self::Spaces { .. } => false,
			Self::Indentation { newlines, .. } => newlines != 0,
			Self::Join { ref lhs, ref rhs } => lhs.has_newlines(input) && rhs.has_newlines(input),
			Self::Dynamic(ref s) => s.contains('\n'),
		}
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
			Self::Spaces { len } =>
				for _ in 0..len {
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
			Self::Join { ref lhs, ref rhs } => {
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
			Self::Spaces { len: 0 } => "".into(),
			Self::Spaces { len: 1 } => " ".into(),

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

impl<'de> serde::Deserialize<'de> for AstStrRepr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		// TODO: Should we try to deserialize a replacement?
		//       This impl is only used for testing the parsing output,
		//       which only contains ranges, so it might be unnecessary.
		AstRange::deserialize(deserializer).map(Self::AstRange)
	}
}
