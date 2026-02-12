//! Tuple indexing expression

use {
	super::{ExpressionWithoutBlockInner, TupleIndex},
	crate::{
		expr::{Expression, ExpressionInner},
		token,
	},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::ParseRecursive,
	rustidy_print::Print, rustidy_util::Whitespace,
};

/// `TupleIndexingExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct TupleIndexingExpression {
	pub expr:  Expression,
	#[format(prefix_ws = Whitespace::remove)]
	pub dot:   token::Dot,
	#[format(prefix_ws = Whitespace::remove)]
	pub ident: TupleIndex,
}
