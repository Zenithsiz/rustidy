//! Parser tags

/// Parser tag
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ParserTag {
	// TODO: Move all the `Skip...` tags into their own enum
	SkipAssignmentExpression,
	SkipBlockExpression,
	SkipCompoundAssignmentExpression,
	SkipDelimiters,
	SkipExpressionWithoutBlock,
	SkipLazyBooleanExpression,
	SkipOptionalTrailingBlockExpression,
	SkipRangeExpr,
	SkipRangeFromExpr,
	SkipRangeInclusiveExpr,
	SkipStructExpression,
	SkipTokenCrate,
	SkipTokenDollar,
	SkipTokenPlus,
	SkipTokenQuestion,
	SkipTokenStar,
	SkipWhitespace,
}
