//! Static item

// Imports
use {
	super::function::ItemSafety,
	crate::{expr::Expression, token, ty::Type},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `StaticItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StaticItem {
	pub safety:  Option<ItemSafety>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.safety.is_some()))]
	pub static_: token::Static,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub mut_:    Option<token::Mut>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:   Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon:   token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:      Type,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub value:   Option<StaticItemValue>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:    token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StaticItemValue {
	pub eq:    token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub value: Expression,
}
