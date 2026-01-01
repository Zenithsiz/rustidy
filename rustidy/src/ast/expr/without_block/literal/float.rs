//! Float literal

// Imports
use {
	super::int::DecLiteral,
	crate::{
		Format,
		Parse,
		Print,
		ast::{ident::IdentifierOrKeyword, token},
	},
};


/// `FLOAT_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a floating point literal")]
pub struct FloatLiteral {
	pub int:    DecLiteral,
	#[parse(with_tag = "skip:Whitespace")]
	pub dot:    token::Dot,
	#[parse(with_tag = "skip:Whitespace")]
	pub frac:   DecLiteral,
	// TODO: This should just be `Suffix`.
	#[parse(with_tag = "skip:Whitespace")]
	pub suffix: Option<IdentifierOrKeyword>,
}
