//! Index

// Imports
use {
	crate::{expr::{Expression, ExpressionInner}, util::Bracketed},
	super::ExpressionWithoutBlockInner,
	ast_util::delimited,
	format::{Format, Formattable, WhitespaceFormat},
	parse::ParseRecursive,
	print::Print,
	util::Whitespace,
};

/// `IndexExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct IndexExpression {
	pub expr:  Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtRemove)]
	pub index: Bracketed<Expression>,
}
