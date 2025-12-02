//! Constants

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{expr::Expression, ident::Ident, token, ty::Type},
};

/// `ConstantItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstantItem {
	const_: token::Const,
	name:   ConstantItemName,
	#[parse(fatal)]
	colon:  token::Colon,
	ty:     Type,
	value:  Option<ConstantItemValue>,
	semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ConstantItemName {
	Ident(Ident),
	Underscore(token::Underscore),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstantItemValue {
	eq:   token::Eq,
	expr: Expression,
}
