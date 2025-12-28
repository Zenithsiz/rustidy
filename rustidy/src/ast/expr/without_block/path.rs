//! Path expression

use {
	super::LiteralExpression,
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			at_least::AtLeast1,
			delimited::{Delimited, Parenthesized},
			expr::with_block::BlockExpression,
			ident::Identifier,
			item::function::TypeParamBounds,
			lifetime::Lifetime,
			longest::Longest,
			path::SimplePathSegment,
			punct::{Punctuated, PunctuatedTrailing},
			token,
			ty::{Type, TypeNoBounds, TypePath, path::TypePathSegment},
		},
	},
};

/// `PathExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum PathExpression {
	Normal(PathInExpression),
	Qualified(QualifiedPathInExpression),
}

/// `PathInExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathInExpression {
	pub prefix:   Option<token::PathSep>,
	pub segments: Punctuated<PathExprSegment, token::PathSep>,
}

/// `PathExprSegment`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathExprSegment {
	pub ident:   PathIdentSegment,
	pub generic: Option<PathExprSegmentGenericArgs>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathExprSegmentGenericArgs {
	pub sep:     token::PathSep,
	pub generic: GenericArgs,
}

/// `PathIdentSegment`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum PathIdentSegment {
	Super(token::Super),
	SelfLower(token::SelfLower),
	SelfUpper(token::SelfUpper),
	Crate(token::Crate),
	DollarCrate(token::DollarCrate),
	Ident(Identifier),
}

/// `GenericArgs`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "generic arguments")]
pub struct GenericArgs(pub Delimited<Option<GenericArgsInner>, token::Lt, token::Gt>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericArgsInner(pub PunctuatedTrailing<GenericArg, token::Comma>);

/// `GenericArg`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum GenericArg {
	Lifetime(Lifetime),

	Binding(GenericArgsBinding),
	Bounds(GenericArgsBounds),

	// TODO: Unmerge these with some attribute
	TypeOrConst(Longest<Box<Type>, GenericArgsConst>),
}

/// `GenericArgsConst`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum GenericArgsConst {
	Block(Box<BlockExpression>),
	Literal(LiteralExpression),
	NegLiteral((token::Minus, LiteralExpression)),
	Path(SimplePathSegment),
}

/// `GenericArgsBinding`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericArgsBinding {
	pub ident:    Identifier,
	pub generics: Option<Box<GenericArgs>>,
	pub eq:       token::Eq,
	#[parse(fatal)]
	pub ty:       Box<Type>,
}

/// `GenericArgsBounds`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericArgsBounds {
	pub ident:    Identifier,
	pub generics: Option<Box<GenericArgs>>,
	pub colon:    token::Colon,
	#[parse(fatal)]
	pub ty:       Box<TypeParamBounds>,
}

/// `TypePathFn`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathFn {
	pub inputs: Parenthesized<Option<TypePathFnInputs>>,
	pub ret:    Option<TypePathFnRet>,
}

/// `TypePathFnInputs`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathFnInputs(PunctuatedTrailing<Box<Type>, token::Comma>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathFnRet {
	pub arrow: token::RArrow,
	pub ty:    Box<TypeNoBounds>,
}

/// `QualifiedPathInExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInExpression {
	pub qualified: QualifiedPathType,
	pub segments:  AtLeast1<(token::PathSep, PathExprSegment)>,
}

/// `QualifiedPathType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathType(Delimited<QualifiedPathTypeInner, token::Lt, token::Gt>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathTypeInner {
	pub ty:  Box<Type>,
	pub as_: Option<QualifiedPathTypeAs>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathTypeAs {
	pub as_: token::As,
	pub ty:  TypePath,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInTypeSegment {
	pub sep:     token::PathSep,
	pub segment: TypePathSegment,
}
