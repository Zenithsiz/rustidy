//! Index

// Imports
use {
	super::ExpressionWithoutBlockInner,
	crate::ast::{
		delimited::Bracketed,
		expr::{Expression, ExpressionInner},
	},
	rustidy_format::Format,
	rustidy_parse::ParseRecursive,
	rustidy_print::Print,
};

/// `IndexExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct IndexExpression {
	pub expr:  Expression,
	#[format(and_with = Format::prefix_ws_remove)]
	#[format(and_with = Bracketed::format_remove)]
	pub index: Bracketed<Expression>,
}
