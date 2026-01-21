//! Static item

// Imports
use {
	super::function::ItemSafety,
	crate::ast::{expr::Expression, token, ty::Type},
	rustidy_ast_util::Identifier,
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `StaticItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StaticItem {
	pub safety:  Option<ItemSafety>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.safety.is_some()))]
	pub static_: token::Static,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub mut_:    Option<token::Mut>,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ident:   Identifier,
	#[format(before_with = Format::prefix_ws_remove)]
	pub colon:   token::Colon,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ty:      Type,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub value:   Option<StaticItemValue>,
	#[format(before_with = Format::prefix_ws_remove)]
	pub semi:    token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StaticItemValue {
	pub eq:    token::Eq,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub value: Expression,
}
