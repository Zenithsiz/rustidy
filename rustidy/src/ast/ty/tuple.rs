//! Tuple type

// Imports
use {
	super::Type,
	crate::{
		Format,
		Parse,
		Print,
		ast::{delimited::Parenthesized, token},
	},
};

/// `TupleType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a tuple type")]
pub struct TupleType(Parenthesized<Option<TupleTypeInner>>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleTypeInner {
	tys: Vec<(Type, token::Comma)>,
	end: Option<Box<Type>>,
}
