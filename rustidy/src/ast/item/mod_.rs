//! Module item

// Imports
use {
	super::Item,
	crate::ast::{token, util::Braced, with_attrs::WithInnerAttributes},
	rustidy_ast_util::Identifier,
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `Module`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "module declaration")]
pub struct Module {
	pub unsafe_: Option<token::Unsafe>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.unsafe_.is_some()))]
	pub mod_:    token::Mod,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ident:   Identifier,
	#[format(and_with = match self.inner.is_none() {
		true => Format::prefix_ws_remove,
		false => Format::prefix_ws_set_single,
	})]
	pub inner:   ModuleInner,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ModuleInner {
	None(token::Semi),
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	Def(Braced<WithInnerAttributes<ModuleItems>>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ModuleItems(
	#[format(and_with = rustidy_format::format_vec_each_with_all(Format::prefix_ws_set_cur_indent))] pub Vec<Item>,
);
