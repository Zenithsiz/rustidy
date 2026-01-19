//! Field expression

use {
	super::ExpressionWithoutBlockInner,
	crate::ast::{
		expr::{Expression, ExpressionInner},
		ident::Identifier,
		token,
	},
	rustidy_format::Format,
	rustidy_parse::ParseRecursive,
	rustidy_print::Print,
};

/// `FieldExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct FieldExpression {
	pub expr:  Expression,
	#[format(and_with = Format::prefix_ws_remove)]
	pub dot:   token::Dot,
	#[format(and_with = Format::prefix_ws_remove)]
	pub ident: Identifier,
}
