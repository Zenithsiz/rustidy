//! Block expression

// Imports
use crate::{
	Format,
	ast::{attr::InnerAttrOrDocComment, delimited::Braced, expr::ExpressionWithoutBlock, stmt::Statement},
	parser::Parse,
	print::Print,
};

/// `BlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a block expression")]
#[parse(skip_if_tag = "skip:BlockExpression")]
pub struct BlockExpression(pub Braced<BlockExpressionInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BlockExpressionInner {
	attrs: Vec<InnerAttrOrDocComment>,
	stmts: Statements,
}

/// `Statements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Statements {
	stmts:         Vec<Statement>,
	trailing_expr: Option<ExpressionWithoutBlock>,
}
