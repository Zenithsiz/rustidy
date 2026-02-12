//! Static item

// Imports
use {
	super::function::ItemSafety,
	crate::{expr::Expression, token, ty::Type},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `StaticItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StaticItem {
	pub safety:  Option<ItemSafety>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.safety.is_some()))]
	pub static_: token::Static,
	#[format(prefix_ws = Whitespace::set_single)]
	pub mut_:    Option<token::Mut>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub ident:   Identifier,
	#[format(prefix_ws = Whitespace::remove)]
	pub colon:   token::Colon,
	#[format(prefix_ws = Whitespace::set_single)]
	pub ty:      Type,
	#[format(prefix_ws = Whitespace::set_single)]
	pub value:   Option<StaticItemValue>,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi:    token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StaticItemValue {
	pub eq:    token::Eq,
	#[format(prefix_ws = Whitespace::set_single)]
	pub value: Expression,
}
