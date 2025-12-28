//! Constants

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{expr::Expression, ident::Identifier, token, ty::Type},
};

/// `ConstantItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstantItem {
	pub const_: token::Const,
	pub name:   ConstantItemName,
	#[parse(fatal)]
	pub colon:  token::Colon,
	pub ty:     Type,
	pub value:  Option<ConstantItemValue>,
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ConstantItemName {
	Ident(Identifier),
	Underscore(token::Underscore),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstantItemValue {
	pub eq:   token::Eq,
	pub expr: Expression,
}
