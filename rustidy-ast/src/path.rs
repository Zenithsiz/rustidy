//! Path

// Imports
use {
	super::token,
	core::fmt::Debug,
	rustidy_ast_util::{Identifier, Punctuated, punct},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `SimplePath`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a simple path")]
pub struct SimplePath {
	pub prefix:   Option<token::PathSep>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if = self.prefix.is_some()))]
	#[format(args = punct::args(Whitespace::REMOVE, Whitespace::REMOVE))]
	pub segments: Punctuated<SimplePathSegment, token::PathSep>,
}

impl SimplePath {
	// TODO: Remove once `UseDecl` no longer needs this
	pub const fn prefix_ws(&mut self) -> &mut Whitespace {
		match &mut self.prefix {
			Some(prefix) => &mut prefix.ws,
			None => self.segments.first.prefix_ws(),
		}
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
				(Some(lhs), Some(rhs)) =>
					if !lhs.is_str(input, rhs) {
						return false;
					},
			}
		}

		true
	}
}

/// `SimplePathSegment`
#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum SimplePathSegment {
	Super(token::Super),
	SelfLower(token::SelfLower),
	Crate(token::Crate),
	DollarCrate(token::DollarCrate),
	Ident(Identifier),
}

impl SimplePathSegment {
	// TODO: Remove once `UseDecl` no longer needs this
	pub const fn prefix_ws(&mut self) -> &mut Whitespace {
		match self {
			Self::Super(super_) => &mut super_.ws,
			Self::SelfLower(self_lower) => &mut self_lower.ws,
			Self::Crate(crate_) => &mut crate_.ws,
			Self::DollarCrate(dollar_crate) => &mut dollar_crate.ws,
			Self::Ident(ident) => match ident {
				Identifier::Raw(ident) => &mut ident.0,
				Identifier::NonKw(ident) => &mut ident.0.0,
			},
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
