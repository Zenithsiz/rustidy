//! Static item

// Imports
use {
	crate::{expr::Expression, ty::Type},
	super::function::ItemSafety,
	ast_literal::Identifier,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `StaticItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StaticItem {
	pub safety:  Option<ItemSafety>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.safety.is_some()))]
	pub static_: ast_token::Static,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub mut_:    Option<ast_token::Mut>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:   Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon:   ast_token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:      Type,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub value:   Option<StaticItemValue>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:    ast_token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StaticItemValue {
	pub eq:    ast_token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub value: Expression,
}
