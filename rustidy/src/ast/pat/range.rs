//! Range patterns

// Imports
use {
	super::LiteralPattern,
	crate::ast::{expr::PathExpression, token},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
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
	#[format(and_with = Format::prefix_ws_remove)]
	dot_dot: token::DotDot,
	#[format(and_with = Format::prefix_ws_remove)]
	rhs:     RangePatternBound,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeInclusivePattern {
	lhs:        RangePatternBound,
	#[format(and_with = Format::prefix_ws_remove)]
	dot_dot_eq: token::DotDotEq,
	#[format(and_with = Format::prefix_ws_remove)]
	rhs:        RangePatternBound,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeFromPattern {
	lhs:     RangePatternBound,
	#[format(and_with = Format::prefix_ws_remove)]
	dot_dot: token::DotDot,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeToExclusivePattern {
	dot_dot: token::DotDot,
	#[format(and_with = Format::prefix_ws_remove)]
	rhs:     RangePatternBound,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeToInclusivePattern {
	dot_dot_eq: token::DotDotEq,
	#[format(and_with = Format::prefix_ws_remove)]
	rhs:        RangePatternBound,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ObsoleteRangePattern {
	lhs:         RangePatternBound,
	#[format(and_with = Format::prefix_ws_remove)]
	dot_dot_dot: token::DotDotDot,
	#[format(and_with = Format::prefix_ws_remove)]
	rhs:         RangePatternBound,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum RangePatternBound {
	Literal(LiteralPattern),
	Path(PathExpression),
}
