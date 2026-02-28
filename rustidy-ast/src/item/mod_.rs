//! Module item

// Imports
use {
	crate::attr::{self, BracedWithInnerAttributes},
	super::Items,
	ast_literal::Identifier,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `Module`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "module declaration")]
pub struct Module {
	pub unsafe_: Option<ast_token::Unsafe>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.unsafe_.is_some()))]
	pub mod_:    ast_token::Mod,
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
	None(ast_token::Semi),
	#[format(args = attr::with::fmt_braced_indent())]
	Def(BracedWithInnerAttributes<Option<Items>>),
}
