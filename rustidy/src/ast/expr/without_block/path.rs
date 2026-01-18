//! Path expression

use {
	super::LiteralExpression,
	crate::{
		Format,
		Print,
		ast::{
			at_least::{self, AtLeast1},
			delimited::{Delimited, Parenthesized},
			expr::with_block::BlockExpression,
			ident::Identifier,
			item::function::TypeParamBounds,
			lifetime::Lifetime,
			longest::Longest,
			path::SimplePathSegment,
			punct::{self, Punctuated, PunctuatedTrailing},
			token,
			ty::{Type, TypeNoBounds, TypePath, path::TypePathSegment},
		},
	},
	rustidy_parse::Parse,
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
	#[format(and_with(expr = Format::prefix_ws_remove, if = self.prefix.is_some()))]
	#[format(and_with = punct::format(Format::prefix_ws_remove, Format::prefix_ws_remove))]
	pub segments: Punctuated<PathExprSegment, token::PathSep>,
}

/// `PathExprSegment`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathExprSegment {
	pub ident:   PathIdentSegment,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generic: Option<PathExprSegmentGenericArgs>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathExprSegmentGenericArgs {
	pub sep:     token::PathSep,
	#[format(and_with = Format::prefix_ws_remove)]
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
	#[format(and_with = Delimited::format_remove)] pub Delimited<Option<GenericArgsInner>, token::Lt, token::Gt>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericArgsInner(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
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
	#[format(and_with = Format::prefix_ws_remove)]
	pub generics: Option<Box<GenericArgs>>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub eq:       token::Eq,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:       Box<Type>,
}

/// `GenericArgsBounds`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericArgsBounds {
	pub ident:    Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generics: Option<Box<GenericArgs>>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon:    token::Colon,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:       Box<TypeParamBounds>,
}

/// `TypePathFn`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathFn {
	#[format(and_with = Parenthesized::format_remove)]
	pub inputs: Parenthesized<Option<TypePathFnInputs>>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ret:    Option<TypePathFnRet>,
}

/// `TypePathFnInputs`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathFnInputs(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_remove, Format::prefix_ws_remove))]
	PunctuatedTrailing<Box<Type>, token::Comma>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathFnRet {
	pub arrow: token::RArrow,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:    Box<TypeNoBounds>,
}

/// `QualifiedPathInExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInExpression {
	pub qualified: QualifiedPathType,
	#[format(and_with = Format::prefix_ws_remove)]
	#[format(and_with = at_least::format(Format::prefix_ws_remove))]
	pub segments:  AtLeast1<QualifiedPathInExpressionSegment>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInExpressionSegment {
	sep:     token::PathSep,
	#[format(and_with = Format::prefix_ws_remove)]
	segment: PathExprSegment,
}

/// `QualifiedPathType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathType(
	#[format(and_with = Delimited::format_remove)] Delimited<QualifiedPathTypeInner, token::Lt, token::Gt>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathTypeInner {
	pub ty:  Box<Type>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub as_: Option<QualifiedPathTypeAs>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathTypeAs {
	pub as_: token::As,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:  TypePath,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInTypeSegment {
	pub sep:     token::PathSep,
	pub segment: TypePathSegment,
}
