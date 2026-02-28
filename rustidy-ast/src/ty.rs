//! Type

// Modules
pub mod array;
pub mod bare_function;
pub mod path;
pub mod pointer;
pub mod qualified;
pub mod slice;
pub mod tuple;

// Exports
pub use self::{
	array::ArrayType,
	bare_function::BareFunctionType,
	path::TypePath,
	pointer::RawPointerType,
	qualified::QualifiedPathInType,
	slice::SliceType,
	tuple::TupleType,
};

// Imports
use {
	super::{
		expr::without_block::MacroInvocation,
		item::function::{TraitBound, TypeParamBounds},
		util::Parenthesized,
	},
	ast_literal::Lifetime,
	ast_util::delimited,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::{ArenaData, ArenaIdx, Whitespace},
};

/// `Type`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Type(pub ArenaIdx<TypeInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(ArenaData)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a type")]
pub enum TypeInner {
	ImplTrait(ImplTraitType),
	TraitObject(TraitObjectType),
	#[parse(peek = MacroInvocation)]
	NoBounds(TypeNoBounds),
}

/// `TypeNoBounds`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(derive_more::From)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum TypeNoBounds {
	MacroInvocation(MacroInvocation),
	Path(TypePath),

	Parenthesized(ParenthesizedPath),
	ImplTraitOneBound(ImplTraitTypeOneBound),
	TraitObjectOneBound(TraitObjectTypeOneBound),
	Tuple(TupleType),
	Never(NeverType),
	RawPointer(RawPointerType),
	Reference(ReferenceType),
	Array(ArrayType),
	Slice(SliceType),
	Inferred(InferredType),
	QualifiedPathIn(QualifiedPathInType),
	BareFunction(BareFunctionType),
}

/// `ParenthesizedPath`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ParenthesizedPath(
	#[format(args = delimited::fmt_single_if_non_blank())]
	Parenthesized<Box<Type>>,
);

/// `NeverType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct NeverType(ast_token::Not);

/// `ReferenceType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a reference type")]
pub struct ReferenceType {
	pub ref_:     ast_token::AndTy,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub lifetime: Option<Lifetime>,
	#[format(prefix_ws = match self.lifetime.is_some() {
		true => Whitespace::SINGLE,
		false => Whitespace::REMOVE,
	})]
	pub mut_:     Option<ast_token::Mut>,
	#[format(prefix_ws = match self.lifetime.is_some() || self.mut_.is_some() {
		true => Whitespace::SINGLE,
		false => Whitespace::REMOVE,
	})]
	pub ty:       Box<TypeNoBounds>,
}

/// `InferredType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct InferredType(ast_token::Underscore);

/// `ImplTraitTypeOneBound`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ImplTraitTypeOneBound {
	pub impl_: ast_token::Impl,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub bound: TraitBound,
}

/// `TraitObjectTypeOneBound`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TraitObjectTypeOneBound {
	pub dyn_:  Option<ast_token::Dyn>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.dyn_.is_some()))]
	pub bound: TraitBound,
}

/// `ImplTraitType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ImplTraitType {
	pub impl_: ast_token::Impl,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub bound: TypeParamBounds,
}

/// `TraitObjectType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TraitObjectType {
	pub dyn_:  Option<ast_token::Dyn>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.dyn_.is_some()))]
	pub bound: TypeParamBounds,
}
