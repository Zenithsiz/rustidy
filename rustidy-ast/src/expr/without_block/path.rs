//! Path expression

use {
	super::LiteralExpression,
	crate::{
		expr::with_block::BlockExpression,
		item::function::TypeParamBounds,
		lifetime::Lifetime,
		path::SimplePathSegment,
		token,
		ty::{Type, TypeNoBounds, TypePath, path::TypePathSegment},
		util::Parenthesized,
	},
	rustidy_ast_util::{
		AtLeast1,
		Delimited,
		Identifier,
		Longest,
		Punctuated,
		PunctuatedTrailing,
		at_least,
		delimited,
		punct,
	},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `PathExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum PathExpression {
	Normal(PathInExpression),
	Qualified(QualifiedPathInExpression),
}

/// `PathInExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathInExpression {
	pub prefix:   Option<token::PathSep>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if = self.prefix.is_some()))]
	#[format(args = punct::fmt(Whitespace::REMOVE, Whitespace::REMOVE))]
	pub segments: Punctuated<PathExprSegment, token::PathSep>,
}

/// `PathExprSegment`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathExprSegment {
	pub ident:   PathIdentSegment,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generic: Option<PathExprSegmentGenericArgs>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathExprSegmentGenericArgs {
	pub sep:     token::PathSep,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generic: GenericArgs,
}

/// `PathIdentSegment`
#[derive(PartialEq, Eq, Debug)]
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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "generic arguments")]
pub struct GenericArgs(
	#[format(args = delimited::fmt_remove((), (), ()))]
	pub  Delimited<Option<GenericArgsInner>, token::Lt, token::Gt>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericArgsInner(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub  PunctuatedTrailing<GenericArg, token::Comma>,
);

/// `GenericArg`
#[derive(PartialEq, Eq, Debug)]
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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum GenericArgsConst {
	Block(Box<BlockExpression>),
	Literal(LiteralExpression),
	NegLiteral((token::Minus, LiteralExpression)),
	Path(SimplePathSegment),
}

/// `GenericArgsBinding`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericArgsBinding {
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<Box<GenericArgs>>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:       token::Eq,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:       Box<Type>,
}

/// `GenericArgsBounds`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericArgsBounds {
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<Box<GenericArgs>>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon:    token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:       Box<TypeParamBounds>,
}

/// `TypePathFn`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathFn {
	#[format(args = delimited::fmt_remove((), (), ()))]
	pub inputs: Parenthesized<Option<TypePathFnInputs>>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ret:    Option<TypePathFnRet>,
}

/// `TypePathFnInputs`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathFnInputs(
	#[format(args = punct::fmt(Whitespace::REMOVE, Whitespace::REMOVE))] PunctuatedTrailing<Box<Type>, token::Comma>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathFnRet {
	pub arrow: token::RArrow,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Box<TypeNoBounds>,
}

/// `QualifiedPathInExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInExpression {
	pub qualified: QualifiedPathType,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = at_least::fmt_prefix_ws(Whitespace::REMOVE))]
	pub segments:  AtLeast1<QualifiedPathInExpressionSegment>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInExpressionSegment {
	sep:     token::PathSep,
	#[format(prefix_ws = Whitespace::REMOVE)]
	segment: PathExprSegment,
}

/// `QualifiedPathType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathType(
	#[format(args = delimited::fmt_remove((), (), ()))] Delimited<QualifiedPathTypeInner, token::Lt, token::Gt>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathTypeInner {
	pub ty:  Box<Type>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub as_: Option<QualifiedPathTypeAs>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathTypeAs {
	pub as_: token::As,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:  TypePath,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInTypeSegment {
	pub sep:     token::PathSep,
	pub segment: TypePathSegment,
}
