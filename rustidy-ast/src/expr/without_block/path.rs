//! Path expression

use {
	crate::{
		expr::with_block::BlockExpression,
		item::function::TypeParamBounds,
		path::SimplePathSegment,
		ty::{Type, TypeNoBounds, TypePath, path::TypePathSegment},
		util::Parenthesized,
	},
	super::LiteralExpression,
	ast_literal::Identifier,
	ast_literal::Lifetime,
	ast_util::{
		AtLeast1,
		Delimited,
		Longest,
		Punctuated,
		PunctuatedTrailing,
		at_least,
		delimited,
		punct,
	},
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `PathExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum PathExpression {
	Normal(PathInExpression),
	Qualified(QualifiedPathInExpression),
}

/// `PathInExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct PathInExpression {
	pub prefix:   Option<ast_token::PathSep>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if_ = self.prefix.is_some()))]
	#[format(args = punct::fmt(Whitespace::REMOVE, Whitespace::REMOVE))]
	pub segments: Punctuated<PathExprSegment, ast_token::PathSep>,
}

/// `PathExprSegment`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct PathExprSegment {
	pub ident:   PathIdentSegment,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generic: Option<PathExprSegmentGenericArgs>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct PathExprSegmentGenericArgs {
	pub sep:     ast_token::PathSep,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generic: GenericArgs,
}

/// `PathIdentSegment`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum PathIdentSegment {
	Super(ast_token::Super),
	SelfLower(ast_token::SelfLower),
	SelfUpper(ast_token::SelfUpper),
	Crate(ast_token::Crate),
	DollarCrate(ast_token::DollarCrate),
	Ident(Identifier),
}

/// `GenericArgs`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "generic arguments")]
pub struct GenericArgs(
	#[format(args = delimited::FmtRemove)]
	pub Delimited<Option<GenericArgsInner>, ast_token::Lt, ast_token::Gt>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct GenericArgsInner(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub PunctuatedTrailing<GenericArg, ast_token::Comma>,
);

/// `GenericArg`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
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
#[derive(Parse, Formattable, Format, Print)]
pub enum GenericArgsConst {
	Block(Box<BlockExpression>),
	Literal(LiteralExpression),
	NegLiteral((ast_token::Minus, LiteralExpression)),
	Path(SimplePathSegment),
}

/// `GenericArgsBinding`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct GenericArgsBinding {
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<Box<GenericArgs>>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:       ast_token::Eq,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:       Box<Type>,
}

/// `GenericArgsBounds`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct GenericArgsBounds {
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<Box<GenericArgs>>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon:    ast_token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:       Box<TypeParamBounds>,
}

/// `TypePathFn`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TypePathFn {
	#[format(args = delimited::FmtRemove)]
	pub inputs: Parenthesized<Option<TypePathFnInputs>>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ret:    Option<TypePathFnRet>,
}

/// `TypePathFnInputs`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TypePathFnInputs(
	#[format(args = punct::fmt(Whitespace::REMOVE, Whitespace::REMOVE))]
	PunctuatedTrailing<Box<Type>, ast_token::Comma>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TypePathFnRet {
	pub arrow: ast_token::RArrow,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Box<TypeNoBounds>,
}

/// `QualifiedPathInExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct QualifiedPathInExpression {
	pub qualified: QualifiedPathType,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = at_least::fmt_prefix_ws(Whitespace::REMOVE))]
	pub segments:  AtLeast1<QualifiedPathInExpressionSegment>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct QualifiedPathInExpressionSegment {
	sep:     ast_token::PathSep,
	#[format(prefix_ws = Whitespace::REMOVE)]
	segment: PathExprSegment,
}

/// `QualifiedPathType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct QualifiedPathType(
	#[format(args = delimited::FmtRemove)]
	Delimited<QualifiedPathTypeInner, ast_token::Lt, ast_token::Gt>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct QualifiedPathTypeInner {
	pub ty:  Box<Type>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub as_: Option<QualifiedPathTypeAs>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct QualifiedPathTypeAs {
	pub as_: ast_token::As,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:  TypePath,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct QualifiedPathInTypeSegment {
	pub sep:     ast_token::PathSep,
	pub segment: TypePathSegment,
}
