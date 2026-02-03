//! Extern block

// Imports
use {
	super::{Function, MacroInvocationSemi, StaticItem, TypeAlias, function::Abi},
	crate::{
		token,
		util::Braced,
		vis::Visibility,
		with_attrs::{self, WithInnerAttributes, WithOuterAttributes},
	},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `ExternBlock`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternBlock {
	pub unsafe_: Option<token::Unsafe>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.unsafe_.is_some()))]
	pub extern_: token::Extern,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub abi:     Option<Abi>,
	#[format(before_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub inner:   Braced<WithInnerAttributes<ExternBlockItems>>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternBlockItems(
	#[format(and_with = rustidy_format::format_vec_each_with_all(Format::prefix_ws_set_cur_indent))] Vec<ExternalItem>,
);

/// `ExternalItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItem(
	#[format(and_with = with_attrs::format_outer_value_non_empty(Format::prefix_ws_set_cur_indent))]
	pub  WithOuterAttributes<ExternalItemInner>,
);

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
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.vis.is_some()))]
	pub static_: StaticItem,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItemFunction {
	pub vis:      Option<Visibility>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.vis.is_some()))]
	pub function: Function,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItemTypeAlias {
	pub vis:   Option<Visibility>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.vis.is_some()))]
	pub alias: TypeAlias,
}
