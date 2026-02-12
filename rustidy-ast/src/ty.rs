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
		lifetime::Lifetime,
		token,
		util::Parenthesized,
	},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{Arena, ArenaData, ArenaIdx, Whitespace},
};

/// `Type`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[expect(clippy::use_self, reason = "`Parse` derive macro doesn't support `Self`")]
pub struct Type(pub ArenaIdx<Type>);

impl ArenaData for Type {
	type Data = TypeInner;

	const ARENA: &'static Arena<Self> = &TYPE_ARENA;
}

static TYPE_ARENA: Arena<Type> = Arena::new();

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a type")]
pub enum TypeInner {
	ImplTrait(ImplTraitType),
	TraitObject(TraitObjectType),
	#[parse(peek = MacroInvocation)]
	NoBounds(TypeNoBounds),
}

/// `TypeNoBounds`
#[derive(PartialEq, Eq, Debug)]
#[derive(derive_more::From)]
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
	#[format(prefix_ws = Whitespace::remove)]
	pub lifetime: Option<Lifetime>,
	#[format(prefix_ws = match self.lifetime.is_some() {
		true => Whitespace::set_single,
		false => Whitespace::remove,
	})]
	pub mut_:     Option<token::Mut>,
	#[format(prefix_ws = match self.lifetime.is_some() || self.mut_.is_some() {
		true => Whitespace::set_single,
		false => Whitespace::remove,
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
	#[format(prefix_ws = Whitespace::set_single)]
	pub bound: TraitBound,
}

/// `TraitObjectTypeOneBound`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitObjectTypeOneBound {
	pub dyn_:  Option<token::Dyn>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.dyn_.is_some()))]
	pub bound: TraitBound,
}

/// `ImplTraitType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ImplTraitType {
	pub impl_: token::Impl,
	#[format(prefix_ws = Whitespace::set_single)]
	pub bound: TypeParamBounds,
}

/// `TraitObjectType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitObjectType {
	pub dyn_:  Option<token::Dyn>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.dyn_.is_some()))]
	pub bound: TypeParamBounds,
}
