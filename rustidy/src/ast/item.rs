//! Item

// Modules
pub mod const_;
pub mod enum_;
pub mod extern_block;
pub mod extern_crate;
pub mod function;
pub mod implementation;
pub mod macro_;
pub mod macro_rules;
pub mod mod_;
pub mod static_;
pub mod struct_;
pub mod trait_;
pub mod type_alias;
pub mod union;
pub mod use_;

// Exports
pub use self::{
	const_::ConstantItem,
	enum_::Enumeration,
	extern_block::ExternBlock,
	extern_crate::ExternCrate,
	function::Function,
	implementation::Implementation,
	macro_::MacroInvocationSemi,
	macro_rules::MacroRulesDefinition,
	mod_::Module,
	static_::StaticItem,
	struct_::Struct,
	trait_::Trait,
	type_alias::TypeAlias,
	union::Union,
	use_::UseDeclaration,
};

// Imports
use {
	super::{
		attr::{DelimTokenTree, DelimTokenTreeInner},
		delimited::{Braced, Parenthesized},
		ident::Ident,
		punct::PunctuatedTrailing,
		token,
		vis::Visibility,
		with_attrs::WithOuterAttributes,
	},
	crate::{Format, Parse, Print},
};

/// `Item`
pub type Item = WithOuterAttributes<ItemInner>;

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an item")]
pub enum ItemInner {
	Vis(VisItem),
	Macro(MacroItem),
}

/// `VisItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct VisItem {
	vis:   Option<Visibility>,
	inner: VisItemInner,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum VisItemInner {
	Module(Module),
	ExternCrate(ExternCrate),
	Use(UseDeclaration),
	Function(Function),
	TypeAlias(TypeAlias),
	Struct(Struct),
	Enum(Enumeration),
	Union(Union),
	Constant(ConstantItem),
	Static(StaticItem),
	Trait(Trait),
	Implementation(Implementation),
	ExternBlock(ExternBlock),
	DeclMacro(DeclMacro),
}

/// `MacroItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroItem {
	Invocation(MacroInvocationSemi),
	Definition(MacroRulesDefinition),
}


// TODO: The specification doesn't have this, so we need to refine it
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacro {
	macro_: token::Macro,
	#[parse(fatal)]
	ident:  Ident,
	body:   DeclMacroBody,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum DeclMacroBody {
	Branches(DeclMacroBodyBranches),
	Inline(DeclMacroBodyInline),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacroBodyInline {
	args: Parenthesized<DelimTokenTreeInner>,
	body: Braced<DelimTokenTreeInner>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacroBodyBranches(Braced<PunctuatedTrailing<DeclMacroBranch, token::Comma>>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacroBranch {
	args:  DelimTokenTree,
	arrow: token::FatArrow,
	body:  DelimTokenTree,
}
