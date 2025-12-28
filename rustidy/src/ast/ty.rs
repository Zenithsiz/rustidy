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
		delimited::Parenthesized,
		expr::without_block::MacroInvocation,
		item::function::{TraitBound, TypeParamBounds},
		lifetime::Lifetime,
		token,
	},
	crate::{Format, Parse, Print},
};

/// `Type`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a type")]
pub enum Type {
	ImplTrait(ImplTraitType),
	TraitObject(TraitObjectType),
	#[parse(peek = MacroInvocation)]
	NoBounds(TypeNoBounds),
}

/// `TypeNoBounds`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
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
#[derive(Parse, Format, Print)]
pub struct ParenthesizedPath(Parenthesized<Box<Type>>);

/// `NeverType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct NeverType(token::Not);

/// `ReferenceType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a reference type")]
pub struct ReferenceType {
	pub ref_:     token::And,
	pub lifetime: Option<Lifetime>,
	pub mut_:     Option<token::Mut>,
	pub ty:       Box<TypeNoBounds>,
}

/// `InferredType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InferredType(token::Underscore);

/// `ImplTraitTypeOneBound`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ImplTraitTypeOneBound {
	pub impl_: token::Impl,
	pub bound: TraitBound,
}

/// `TraitObjectTypeOneBound`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitObjectTypeOneBound {
	pub dyn_:  Option<token::Dyn>,
	pub bound: TraitBound,
}

/// `ImplTraitType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ImplTraitType {
	pub impl_: token::Impl,
	pub bound: TypeParamBounds,
}

/// `TraitObjectType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitObjectType {
	pub dyn_:  Option<token::Dyn>,
	pub bound: TypeParamBounds,
}
