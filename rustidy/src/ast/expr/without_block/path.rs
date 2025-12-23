//! Path expression

use {
	super::LiteralExpression,
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			at_least::AtLeast1,
			delimited::Parenthesized,
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
	prefix:   Option<token::PathSep>,
	segments: Punctuated<PathExprSegment, token::PathSep>,
}

/// `PathExprSegment`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathExprSegment {
	ident:   PathIdentSegment,
	generic: Option<PathExprSegmentGenericArgs>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathExprSegmentGenericArgs {
	sep:     token::PathSep,
	generic: GenericArgs,
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
pub struct GenericArgs {
	open:  token::Lt,
	#[parse(fatal)]
	inner: Option<GenericArgsInner>,
	close: token::Gt,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericArgsInner {
	args: PunctuatedTrailing<GenericArg, token::Comma>,
}

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
	ident:    Identifier,
	generics: Option<Box<GenericArgs>>,
	eq:       token::Eq,
	#[parse(fatal)]
	ty:       Box<Type>,
}

/// `GenericArgsBounds`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericArgsBounds {
	ident:    Identifier,
	generics: Option<Box<GenericArgs>>,
	colon:    token::Colon,
	#[parse(fatal)]
	ty:       Box<TypeParamBounds>,
}

/// `TypePathFn`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathFn {
	inputs: Parenthesized<Option<TypePathFnInputs>>,
	ret:    Option<TypePathFnRet>,
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
	arrow: token::RArrow,
	ty:    Box<TypeNoBounds>,
}

/// `QualifiedPathInExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInExpression {
	qualified: QualifiedPathType,
	segments:  AtLeast1<(token::PathSep, PathExprSegment)>,
}

/// `QualifiedPathType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathType {
	open:  token::Lt,
	#[parse(fatal)]
	ty:    Box<Type>,
	as_:   Option<QualifiedPathTypeAs>,
	close: token::Gt,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathTypeAs {
	as_: token::As,
	ty:  TypePath,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInTypeSegment {
	sep:     token::PathSep,
	segment: TypePathSegment,
}
