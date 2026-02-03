//! Method call expression

use {
	super::{ExpressionWithoutBlockInner, path::PathExprSegment},
	crate::{
		expr::{Expression, ExpressionInner},
		token,
		util::Parenthesized,
	},
	rustidy_ast_util::{PunctuatedTrailing, punct},
	rustidy_format::Format,
	rustidy_parse::{Parse, ParseRecursive},
	rustidy_print::Print,
};

/// `CallExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct CallExpression {
	pub expr:   Expression,
	#[format(before_with = Format::prefix_ws_remove)]
	#[format(and_with = Parenthesized::format_remove)]
	pub params: Parenthesized<Option<CallParams>>,
}

/// `MethodCallExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct MethodCallExpression {
	pub expr:    Expression,
	#[format(before_with = Format::prefix_ws_remove)]
	pub dot:     token::Dot,
	#[format(before_with = Format::prefix_ws_remove)]
	pub segment: PathExprSegment,
	#[format(before_with = Format::prefix_ws_remove)]
	#[format(and_with = Parenthesized::format_remove)]
	pub params:  Parenthesized<Option<CallParams>>,
}

/// `CallParams`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct CallParams(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
	pub  PunctuatedTrailing<Expression, token::Comma>,
);
