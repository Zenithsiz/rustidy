//! Extern block

// Imports
use {
	super::{Function, MacroInvocationSemi, StaticItem, TypeAlias, function::Abi},
	crate::{
		attr::{BracedWithInnerAttributes, WithOuterAttributes},
		token,
		vis::Visibility,
	},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ExternBlock`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternBlock {
	pub unsafe_: Option<token::Unsafe>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.unsafe_.is_some()))]
	pub extern_: token::Extern,
	#[format(prefix_ws = Whitespace::set_single)]
	pub abi:     Option<Abi>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub inner:   BracedWithInnerAttributes<ExternBlockItems>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternBlockItems(
	#[format(args = rustidy_format::vec::args_prefix_ws(Whitespace::set_cur_indent))] Vec<ExternalItem>,
);

/// `ExternalItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItem(pub WithOuterAttributes<ExternalItemInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ExternalItemInner {
	Macro(MacroInvocationSemi),
	Static(ExternalItemStatic),
	Function(ExternalItemFunction),
	// Note: Nightly-only
	TypeAlias(ExternalItemTypeAlias),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItemStatic {
	pub vis:     Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.vis.is_some()))]
	pub static_: StaticItem,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItemFunction {
	pub vis:      Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.vis.is_some()))]
	pub function: Function,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItemTypeAlias {
	pub vis:   Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.vis.is_some()))]
	pub alias: TypeAlias,
}
