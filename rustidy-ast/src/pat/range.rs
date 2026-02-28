//! Range patterns

// Imports
use {
	crate::expr::PathExpression,
	super::LiteralPattern,

	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `RangePattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum RangePattern {
	Exclusive(RangeExclusivePattern),
	Inclusive(RangeInclusivePattern),
	From(RangeFromPattern),
	ToExclusive(RangeToExclusivePattern),
	ToInclusive(RangeToInclusivePattern),
	Obsolete(ObsoleteRangePattern),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct RangeExclusivePattern {
	lhs:     RangePatternBound,
	#[format(prefix_ws = Whitespace::REMOVE)]
	dot_dot: ast_token::DotDot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	rhs:     RangePatternBound,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct RangeInclusivePattern {
	lhs:        RangePatternBound,
	#[format(prefix_ws = Whitespace::REMOVE)]
	dot_dot_eq: ast_token::DotDotEq,
	#[format(prefix_ws = Whitespace::REMOVE)]
	rhs:        RangePatternBound,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct RangeFromPattern {
	lhs:     RangePatternBound,
	#[format(prefix_ws = Whitespace::REMOVE)]
	dot_dot: ast_token::DotDot,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct RangeToExclusivePattern {
	dot_dot: ast_token::DotDot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	rhs:     RangePatternBound,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct RangeToInclusivePattern {
	dot_dot_eq: ast_token::DotDotEq,
	#[format(prefix_ws = Whitespace::REMOVE)]
	rhs:        RangePatternBound,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ObsoleteRangePattern {
	lhs:         RangePatternBound,
	#[format(prefix_ws = Whitespace::REMOVE)]
	dot_dot_dot: ast_token::DotDotDot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	rhs:         RangePatternBound,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum RangePatternBound {
	Literal(LiteralPattern),
	Path(PathExpression),
}
