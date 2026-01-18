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
	crate::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `Type`
#[derive(PartialEq, Eq, Debug)]
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
#[derive(PartialEq, Eq, Debug)]
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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ParenthesizedPath(#[format(and_with = Parenthesized::format_single_if_non_blank)] Parenthesized<Box<Type>>);

/// `NeverType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct NeverType(token::Not);

/// `ReferenceType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a reference type")]
pub struct ReferenceType {
	pub ref_:     token::AndTy,
	#[format(and_with = Format::prefix_ws_remove)]
	pub lifetime: Option<Lifetime>,
	#[format(and_with = match self.lifetime.is_some() {
		true => Format::prefix_ws_set_single,
		false => Format::prefix_ws_remove,
	})]
	pub mut_:     Option<token::Mut>,
	#[format(and_with = match self.lifetime.is_some() || self.mut_.is_some() {
		true => Format::prefix_ws_set_single,
		false => Format::prefix_ws_remove,
	})]
	pub ty:       Box<TypeNoBounds>,
}

/// `InferredType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InferredType(token::Underscore);

/// `ImplTraitTypeOneBound`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ImplTraitTypeOneBound {
	pub impl_: token::Impl,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub bound: TraitBound,
}

/// `TraitObjectTypeOneBound`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitObjectTypeOneBound {
	pub dyn_:  Option<token::Dyn>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.dyn_.is_some()))]
	pub bound: TraitBound,
}

/// `ImplTraitType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ImplTraitType {
	pub impl_: token::Impl,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub bound: TypeParamBounds,
}

/// `TraitObjectType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitObjectType {
	pub dyn_:  Option<token::Dyn>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.dyn_.is_some()))]
	pub bound: TypeParamBounds,
}
