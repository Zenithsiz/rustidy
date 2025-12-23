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
	int:    DecLiteral,
	dot:    token::Dot,
	frac:   DecLiteral,
	suffix: Option<IdentifierOrKeyword>,
}
