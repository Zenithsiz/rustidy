//! Tuple indexing expression

use {
	super::{ExpressionWithoutBlockInner, literal::TupleIndex},
	crate::{
		Format,
		ParseRecursive,
		Print,
		ast::{
			expr::{Expression, ExpressionInner},
			token,
		},
	},
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
	#[format(and_with = Format::prefix_ws_remove)]
	pub dot:   token::Dot,
	#[format(and_with = Format::prefix_ws_remove)]
	pub ident: TupleIndex,
}
