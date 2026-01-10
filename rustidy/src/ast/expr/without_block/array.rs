//! Array

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{delimited::Bracketed, expr::Expression, punct::PunctuatedTrailing, token},
};

/// `ArrayExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an array expression")]
pub struct ArrayExpression(Bracketed<Option<ArrayElements>>);

/// `ArrayElements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ArrayElements {
	Repeat(ArrayElementsRepeat),
	Punctuated(PunctuatedTrailing<Expression, token::Comma>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ArrayElementsRepeat {
	pub expr:  Expression,
	pub semi:  token::Semi,
	pub count: Expression,
}
