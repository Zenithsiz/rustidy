//! Module item

// Imports
use {
	super::Items,
	crate::{attr::BracedWithInnerAttributes, token},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Module`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "module declaration")]
pub struct Module {
	pub unsafe_: Option<token::Unsafe>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.unsafe_.is_some()))]
	pub mod_:    token::Mod,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:   Identifier,
	#[format(prefix_ws = match self.inner.is_none() {
		true => Whitespace::REMOVE,
		false => Whitespace::SINGLE,
	})]
	pub inner:   ModuleInner,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ModuleInner {
	None(token::Semi),
	Def(BracedWithInnerAttributes<Items>),
}
