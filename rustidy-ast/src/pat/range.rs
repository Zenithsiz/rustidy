//! Range patterns

// Imports
use {
	super::LiteralPattern,
	crate::{expr::PathExpression, token},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `RangePattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum RangePattern {
	Exclusive(RangeExclusivePattern),
	Inclusive(RangeInclusivePattern),
	From(RangeFromPattern),
	ToExclusive(RangeToExclusivePattern),
	ToInclusive(RangeToInclusivePattern),
	Obsolete(ObsoleteRangePattern),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeExclusivePattern {
	lhs:     RangePatternBound,
	#[format(prefix_ws = Whitespace::remove)]
	dot_dot: token::DotDot,
	#[format(prefix_ws = Whitespace::remove)]
	rhs:     RangePatternBound,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeInclusivePattern {
	lhs:        RangePatternBound,
	#[format(prefix_ws = Whitespace::remove)]
	dot_dot_eq: token::DotDotEq,
	#[format(prefix_ws = Whitespace::remove)]
	rhs:        RangePatternBound,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeFromPattern {
	lhs:     RangePatternBound,
	#[format(prefix_ws = Whitespace::remove)]
	dot_dot: token::DotDot,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeToExclusivePattern {
	dot_dot: token::DotDot,
	#[format(prefix_ws = Whitespace::remove)]
	rhs:     RangePatternBound,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeToInclusivePattern {
	dot_dot_eq: token::DotDotEq,
	#[format(prefix_ws = Whitespace::remove)]
	rhs:        RangePatternBound,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ObsoleteRangePattern {
	lhs:         RangePatternBound,
	#[format(prefix_ws = Whitespace::remove)]
	dot_dot_dot: token::DotDotDot,
	#[format(prefix_ws = Whitespace::remove)]
	rhs:         RangePatternBound,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum RangePatternBound {
	Literal(LiteralPattern),
	Path(PathExpression),
}
