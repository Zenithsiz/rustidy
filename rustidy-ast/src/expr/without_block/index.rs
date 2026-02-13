//! Index

// Imports
use {
	super::ExpressionWithoutBlockInner,
	crate::{
		expr::{Expression, ExpressionInner},
		util::Bracketed,
	},
	rustidy_ast_util::delimited,
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::ParseRecursive,
	rustidy_print::Print,
	rustidy_util::Whitespace,
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
	#[format(prefix_ws = Whitespace::remove)]
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	pub index: Bracketed<Expression>,
}
