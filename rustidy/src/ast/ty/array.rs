//! Array type

// Imports
use {
	super::Type,
	crate::{
		Format,
		Parse,
		Print,
		ast::{delimited::Bracketed, expr::Expression, token},
	},
};

/// `ArrayType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ArrayType(Bracketed<ArrayTypeInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ArrayTypeInner {
	ty:   Box<Type>,
	semi: token::Semi,
	expr: Box<Expression>,
}
