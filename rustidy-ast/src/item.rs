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
		attr::{DelimTokenTree, DelimTokenTreeInner, OuterAttrOrDocComment, WithOuterAttributes},
		token,
		util::{Braced, Parenthesized},
		vis::Visibility,
	},
	core::{mem, ops::ControlFlow},
	rustidy_ast_util::{Identifier, PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{Arena, ArenaData, ArenaIdx, Whitespace},
};

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(before_with = Self::merge_use)]
pub struct Items(#[format(args = rustidy_format::VecArgs::from_prefix_ws(Whitespace::set_cur_indent))] pub Vec<Item>);

impl Items {
	pub fn merge_use(&mut self, ctx: &mut rustidy_format::Context) {
		#[expect(clippy::unused_peekable, reason = "We use `Peekable::next_if_map`")]
		let mut items = mem::take(&mut self.0).into_iter().peekable();
		while let Some(mut item) = items.next() {
			item = match item.try_into_use_decl() {
				Ok((attrs, vis, mut first_use_decl)) => {
					let mut use_decls = vec![];
					while let Some(use_decl) = items.next_if_map(|item| item.try_into_just_use_decl(ctx, vis.as_ref()))
					{
						use_decls.push(use_decl);
					}

					first_use_decl.merge(use_decls);
					Item(ArenaIdx::new(WithOuterAttributes {
						attrs,
						inner: ItemInner::Vis(VisItem {
							vis,
							inner: VisItemInner::Use(first_use_decl),
						}),
					}))
				},
				Err(item) => item,
			};

			self.0.push(item);
		}
	}
}

/// `Item`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[expect(clippy::use_self, reason = "`Parse` derive macro doesn't support `Self`")]
pub struct Item(pub ArenaIdx<Item>);

impl Item {
	#[expect(clippy::result_large_err, reason = "TODO")]
	fn try_into_use_decl(self) -> Result<(Vec<OuterAttrOrDocComment>, Option<Visibility>, UseDeclaration), Self> {
		self.0
			.try_take_map(|item| match item.inner {
				ItemInner::Vis(VisItem {
					vis,
					inner: VisItemInner::Use(use_decl),
				}) => Ok((item.attrs, vis, use_decl)),
				_ => Err(item),
			})
			.map_err(Self)
	}

	// TODO: This needs to check for comments in the prefix whitespace.
	#[expect(clippy::result_large_err, reason = "TODO")]
	fn try_into_just_use_decl(
		self,
		ctx: &mut rustidy_format::Context,
		expected_vis: Option<&Visibility>,
	) -> Result<UseDeclaration, Self> {
		self.0
			.try_take_map(|mut item| {
				// TODO: Do this during formatting so we have access to the prefix whitespace more easily.
				let mut seen_first = false;
				let is_prefix_ws_impure = item.with_strings(ctx, false, &mut |s, ctx| {
					if !seen_first {
						seen_first = true;
						return ControlFlow::Continue(());
					}

					let s = s.str(ctx.input());
					ControlFlow::Break(s.starts_with("/*") || s.starts_with("//"))
				});

				if is_prefix_ws_impure == ControlFlow::Break(true) {
					return Err(item);
				}
				if !item.attrs.is_empty() {
					return Err(item);
				}

				match item.inner {
					ItemInner::Vis(VisItem {
						vis,
						inner: VisItemInner::Use(use_decl),
					}) if vis.as_ref() == expected_vis => Ok(use_decl),
					_ => Err(item),
				}
			})
			.map_err(Self)
	}
}

impl ArenaData for Item {
	type Data = WithOuterAttributes<ItemInner>;

	const ARENA: &'static Arena<Self> = &ITEM_ARENA;
}

static ITEM_ARENA: Arena<Item> = Arena::new();

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an item")]
pub enum ItemInner {
	Vis(VisItem),
	Macro(MacroItem),
}

/// `VisItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct VisItem {
	pub vis:   Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.vis.is_some()))]
	pub inner: VisItemInner,
}

#[derive(PartialEq, Eq, Debug)]
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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroItem {
	Invocation(MacroInvocationSemi),
	Definition(MacroRulesDefinition),
}


// Note: Nightly-only
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacro {
	pub macro_: token::Macro,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub ident:  Identifier,
	pub body:   DeclMacroBody,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum DeclMacroBody {
	#[format(prefix_ws = Whitespace::set_single)]
	Branches(DeclMacroBodyBranches),
	#[format(prefix_ws = Whitespace::remove)]
	Inline(DeclMacroBodyInline),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacroBodyInline {
	#[format(indent)]
	#[format(args = delimited::FmtArgs::indent_if_non_blank((), (), ()))]
	pub args: Parenthesized<DelimTokenTreeInner>,
	#[format(prefix_ws = Whitespace::set_single)]
	#[format(indent)]
	#[format(args = delimited::FmtArgs::indent_if_non_blank((), (), ()))]
	pub body: Braced<DelimTokenTreeInner>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacroBodyBranches(
	#[format(indent)]
	#[format(args = delimited::FmtArgs::indent_if_non_blank((), (), ()))]
	pub Braced<DeclMacroBodyBranchesInner>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacroBodyBranchesInner(
	#[format(args = punct::FmtArgs::new(Whitespace::set_cur_indent, Whitespace::remove))]
	pub  PunctuatedTrailing<DeclMacroBranch, token::Comma>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacroBranch {
	pub extra: Option<DeclMacroBranchExtra>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.extra.is_some()))]
	pub args:  DelimTokenTree,
	#[format(prefix_ws = Whitespace::set_single)]
	pub arrow: token::FatArrow,
	#[format(prefix_ws = Whitespace::set_single)]
	pub body:  DelimTokenTree,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum DeclMacroBranchExtra {
	Attr(DeclMacroBranchAttr),
	Derive(DeclMacroBranchDerive),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacroBranchAttr {
	pub attr: token::Attr,
	#[format(prefix_ws = Whitespace::remove)]
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	pub args: Parenthesized<DelimTokenTreeInner>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DeclMacroBranchDerive {
	pub derive: token::Derive,
	#[format(prefix_ws = Whitespace::remove)]
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	pub args:   Parenthesized<()>,
}
