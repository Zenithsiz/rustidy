//! Ast string

// Imports
use {
	crate::{Arena, ArenaData, ArenaIdx, ArenaRef, AstRange, Config},
	std::borrow::Cow,
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
	pub fn len(&self, config: &Config) -> usize {
		match *self.repr() {
			AstStrRepr::AstRange(range) => range.len(),
			AstStrRepr::String(s) => s.len(),
			AstStrRepr::Char(ch) => ch.len_utf8(),
			AstStrRepr::Indentation { newlines, depth } => newlines + depth * config.indent.len(),
			AstStrRepr::Join { ref lhs, ref rhs } => lhs.len(config) + rhs.len(config),
			AstStrRepr::Dynamic(ref s) => s.len(),
		}
	}

	/// Returns if this string is blank
	#[must_use]
	pub fn is_blank(&self, input: &str) -> bool {
		match *self.repr() {
			AstStrRepr::AstRange(range) => crate::is_str_blank(range.str(input)),
			AstStrRepr::String(s) => crate::is_str_blank(s),
			AstStrRepr::Char(ch) => ch.is_ascii_whitespace(),
			AstStrRepr::Indentation { .. } => true,
			AstStrRepr::Join { ref lhs, ref rhs } => lhs.is_blank(input) && rhs.is_blank(input),
			AstStrRepr::Dynamic(ref s) => crate::is_str_blank(s),
		}
	}

	/// Returns if this string is equal to `other`
	#[must_use]
	pub fn is_str(&self, input: &str, config: &Config, other: &str) -> bool {
		let repr = self.repr();
		match *repr {
			AstStrRepr::AstRange(range) => range.str(input) == other,
			AstStrRepr::String(s) => s == other,
			AstStrRepr::Dynamic(ref s) => s == other,

			// TODO: Properly implement these to avoid allocating a string
			_ => {
				drop(repr);
				self.str(input, config) == other
			},
		}
	}

	/// Writes this string
	pub fn write(&self, config: &Config, input: &str, output: &mut String) {
		match *self.repr() {
			AstStrRepr::AstRange(range) => output.push_str(range.str(input)),
			AstStrRepr::String(s) => output.push_str(s),
			AstStrRepr::Char(ch) => output.push(ch),
			AstStrRepr::Indentation { newlines, depth } => {
				for _ in 0..newlines {
					output.push('\n');
				}
				for _ in 0..depth {
					output.push_str(&config.indent);
				}
			},
			AstStrRepr::Join { ref lhs, ref rhs } => {
				lhs.write(config, input, output);
				rhs.write(config, input, output);
			},
			AstStrRepr::Dynamic(ref s) => output.push_str(s),
		}
	}

	/// Returns the string of this string
	// TODO: This can be somewhat expensive, should we replace it with
	//       functions performing whatever checks the callers need instead?
	#[must_use]
	pub fn str<'input>(&self, input: &'input str, config: &Config) -> Cow<'input, str> {
		let repr = self.repr();
		match *repr {
			AstStrRepr::AstRange(range) => range.str(input).into(),
			AstStrRepr::String(s) => s.into(),
			AstStrRepr::Char(ch) => ch.to_string().into(),
			AstStrRepr::Dynamic(ref s) => s.clone().into(),

			AstStrRepr::Indentation { .. } | AstStrRepr::Join { .. } => {
				drop(repr);
				let mut output = String::new();
				self.write(config, input, &mut output);
				output.into()
			},
		}
	}
}

impl ArenaData for AstStr {
	type Data = AstStrRepr;

	const ARENA: &'static Arena<Self> = &ARENA;
}

static ARENA: Arena<AstStr> = Arena::new();

#[derive(Debug)]
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

	/// Indentation
	#[from]
	Indentation {
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
