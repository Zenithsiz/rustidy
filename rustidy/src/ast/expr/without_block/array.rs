//! Array

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{
		delimited::Bracketed,
		expr::Expression,
		punct::{self, PunctuatedTrailing},
		token,
	},
};

/// `ArrayExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an array expression")]
pub struct ArrayExpression(#[format(and_with = Bracketed::format_remove)] Bracketed<Option<ArrayElements>>);

/// `ArrayElements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ArrayElements {
	Repeat(ArrayElementsRepeat),
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
	Punctuated(PunctuatedTrailing<Expression, token::Comma>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ArrayElementsRepeat {
	pub expr:  Expression,
	#[format(and_with = Format::prefix_ws_remove)]
	pub semi:  token::Semi,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub count: Expression,
}
