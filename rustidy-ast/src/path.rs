//! Path

// Imports
use {
	super::token,
	core::fmt::Debug,
	rustidy_ast_util::{Identifier, Punctuated, punct},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
	std::borrow::Cow,
};

/// `SimplePath`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a simple path")]
pub struct SimplePath {
	pub prefix:   Option<token::PathSep>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if_ = self.prefix.is_some()))]
	#[format(args = punct::fmt(Whitespace::REMOVE, Whitespace::REMOVE))]
	pub segments: Punctuated<SimplePathSegment, token::PathSep>,
}

impl SimplePath {
	/// Returns this path as a string
	#[must_use]
	pub fn as_str<'a>(&'a self, input: &'a str) -> Cow<'a, str> {
		// Optimize single segment paths with no prefix first
		if self.prefix.is_none() && self.segments.rest.is_empty() {
			return self.segments.first.as_str(input);
		}

		todo!();
	}

	/// Returns if the first segment of this path is `segment`
	#[must_use]
	pub fn starts_with(&self, input: &str, segment: &str) -> bool {
		// TODO: Should we care about the prefix here?
		self.segments.first.is_str(input, segment)
	}

	/// Returns if this path is the same as `path`.
	#[must_use]
	pub fn is_str(&self, input: &str, path: &str) -> bool {
		// Check for prefix
		let (path, has_prefix) = match path.strip_prefix("::") {
			Some(path) => (path, true),
			None => (path, false),
		};
		if self.prefix.is_some() != has_prefix {
			return false;
		}

		// Then check each segment
		let mut lhs_segments = self.segments.values();
		let mut rhs_segments = path.split("::");
		loop {
			match (lhs_segments.next(), rhs_segments.next()) {
				(None, None) => break,
				(None, Some(_)) | (Some(_), None) => return false,
				(Some(lhs), Some(rhs)) => if !lhs.is_str(input, rhs) {
					return false;
				},
			}
		}

		true
	}
}

/// `SimplePathSegment`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum SimplePathSegment {
	Super(token::Super),
	SelfLower(token::SelfLower),
	Crate(token::Crate),
	DollarCrate(token::DollarCrate),
	Ident(Identifier),
}

impl SimplePathSegment {
	/// Returns this path as a string
	#[must_use]
	pub fn as_str<'a>(&'a self, input: &'a str) -> Cow<'a, str> {
		match self {
			Self::Super(_) => "super".into(),
			Self::SelfLower(_) => "self".into(),
			Self::Crate(_) => "crate".into(),
			Self::DollarCrate(_) => "$crate".into(),
			Self::Ident(ident) => ident.as_str(input),
		}
	}

	/// Returns if this segment is the same as `segment`.
	#[must_use]
	pub fn is_str(&self, input: &str, segment: &str) -> bool {
		match self {
			Self::Super(_) => segment == "super",
			Self::SelfLower(_) => segment == "self",
			Self::Crate(_) => segment == "crate",
			Self::DollarCrate(_) => segment == "$crate",
			Self::Ident(ident) => ident.is_str(input, segment),
		}
	}
}
