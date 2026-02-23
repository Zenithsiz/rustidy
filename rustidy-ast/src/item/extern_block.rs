//! Extern block

// Imports
use {
	crate::{
		attr::{BracedWithInnerAttributes, WithOuterAttributes},
		token,
		vis::Visibility,
	},
	super::{Function, MacroInvocationSemi, StaticItem, TypeAlias, function::Abi},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ExternBlock`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExternBlock {
	pub unsafe_: Option<token::Unsafe>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.unsafe_.is_some()))]
	pub extern_: token::Extern,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub abi:     Option<Abi>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub inner:   BracedWithInnerAttributes<ExternBlockItems>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExternBlockItems(
	#[format(args = rustidy_format::vec::args_prefix_ws(Whitespace::INDENT))]
	Vec<ExternalItem>,
);

/// `ExternalItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExternalItem(pub WithOuterAttributes<ExternalItemInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ExternalItemInner {
	Macro(MacroInvocationSemi),
	Static(ExternalItemStatic),
	Function(ExternalItemFunction),
	// Note: Nightly-only
	TypeAlias(ExternalItemTypeAlias),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExternalItemStatic {
	pub vis:     Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.vis.is_some()))]
	pub static_: StaticItem,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExternalItemFunction {
	pub vis:      Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.vis.is_some()))]
	pub function: Function,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExternalItemTypeAlias {
	pub vis:   Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.vis.is_some()))]
	pub alias: TypeAlias,
}
