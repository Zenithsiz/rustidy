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
	crate::attr::{self, DelimTokenTreeBraces, DelimTokenTreeParens},
	super::{
		attr::{DelimTokenTree, OuterAttrOrDocComment, WithOuterAttributes},
		token,
		util::{Braced, Parenthesized},
		vis::Visibility,
	},
	itertools::Itertools,
	rustidy_ast_util::{AtLeast1, Identifier, PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{ArenaIdx, Whitespace, decl_arena},
};

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(before_with = Self::merge_use)]
pub struct Items(
	#[format(args = rustidy_ast_util::at_least::fmt_prefix_ws(Whitespace::INDENT))]
	pub AtLeast1<Item>,
);

impl Items {
	pub fn merge_use(&mut self, ctx: &mut rustidy_format::Context) {
		replace_with::replace_with_or_abort(&mut self.0, |items| {
			let mut items = items
				.into_iter()
				.peekable()
				.batching(|items| {
					let item = items.next()?;
					let item = match item.try_into_use_decl() {
						Ok((attrs, vis, mut first_use_decl)) => {
							while let Some(use_decl) = items.next_if_map(
								|item| item
									.try_into_just_use_decl(ctx, vis.as_ref())
							) {
								first_use_decl.merge(use_decl);
							}

							Item(ArenaIdx::new(
								WithOuterAttributes { attrs, inner: ItemInner::Vis(
									VisItem { vis, inner: VisItemInner::Use(first_use_decl), }
								), }
							))
						},
						Err(item) => item,
					};

					Some(item)
				});

			let first = items
				.next()
				.expect("Should have at least 1 item");

			AtLeast1 { first, rest: items.collect(), }
		});
	}
}

/// `Item`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Item(
	#[format(args = attr::with::fmt(Whitespace::INDENT))]
	pub ArenaIdx<WithOuterAttributes<ItemInner>>,
);

impl Item {
	#[expect(clippy::result_large_err, reason = "TODO")]
	fn try_into_use_decl(self) -> Result<(Vec<OuterAttrOrDocComment>, Option<Visibility>, UseDeclaration), Self> {
		self.0.try_take_map(|item| match item.inner {
			ItemInner::Vis(VisItem { vis, inner: VisItemInner::Use(use_decl), }) => Ok((item.attrs, vis, use_decl)),
			_ => Err(item),
		}).map_err(Self)
	}

	// TODO: This needs to check for comments in the prefix whitespace.
	#[expect(clippy::result_large_err, reason = "TODO")]
	fn try_into_just_use_decl(
		self,
		ctx: &mut rustidy_format::Context,
		expected_vis: Option<&Visibility>,
	) -> Result<UseDeclaration, Self> {
		self.0.try_take_map(|mut item| {
			// Note: If no prefix whitespace exists, we can merge them anyway.
			if matches!(item.prefix_ws_is_pure(ctx), Some(false)) {
				return Err(item);
			}
			if !item.attrs.is_empty() {
				return Err(item);
			}

			match item.inner {
				ItemInner::Vis(VisItem { vis, inner: VisItemInner::Use(use_decl), }) if vis.as_ref() == expected_vis => Ok(use_decl),
				_ => Err(item),
			}
		}).map_err(Self)
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "an item")]
pub enum ItemInner {
	Vis(VisItem),
	Macro(MacroItem),
}

decl_arena! { WithOuterAttributes<ItemInner> }

/// `VisItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct VisItem {
	pub vis:   Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.vis.is_some()))]
	pub inner: VisItemInner,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
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
#[derive(Parse, Formattable, Format, Print)]
pub enum MacroItem {
	Invocation(MacroInvocationSemi),
	Definition(MacroRulesDefinition),
}


// Note: Nightly-only
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct DeclMacro {
	pub macro_: token::Macro,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:  Identifier,
	#[format(prefix_ws = match self.body {
		DeclMacroBody::Branches(_) => Whitespace::SINGLE,
		DeclMacroBody::Inline(_) => Whitespace::REMOVE,
	})]
	pub body:   DeclMacroBody,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum DeclMacroBody {
	Branches(DeclMacroBodyBranches),
	Inline(DeclMacroBodyInline),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct DeclMacroBodyInline {
	pub args: DelimTokenTreeParens,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub body: DelimTokenTreeBraces,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct DeclMacroBodyBranches(
	#[format(args = delimited::fmt_indent_if_non_blank())]
	pub Braced<DeclMacroBodyBranchesInner>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct DeclMacroBodyBranchesInner(
	#[format(args = punct::fmt(Whitespace::INDENT, Whitespace::REMOVE))]
	pub PunctuatedTrailing<DeclMacroBranch, token::Comma>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct DeclMacroBranch {
	pub extra: Option<DeclMacroBranchExtra>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.extra.is_some()))]
	pub args:  DelimTokenTree,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub arrow: token::FatArrow,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub body:  DelimTokenTree,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum DeclMacroBranchExtra {
	Attr(DeclMacroBranchAttr),
	Derive(DeclMacroBranchDerive),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct DeclMacroBranchAttr {
	pub attr: token::Attr,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub args: DelimTokenTreeParens,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct DeclMacroBranchDerive {
	pub derive: token::Derive,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtArgs {
		indent: false,
		value_non_blank: (),
		value_blank: (),
		suffix_non_blank: Whitespace::REMOVE,
		suffix_blank: Whitespace::REMOVE,
		prefix_args: (),
		value_args: (),
		suffix_args: ()
	})]
	pub args:   Parenthesized<()>,
}
