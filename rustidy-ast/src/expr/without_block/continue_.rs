//! Continue

// Imports
use {
	ast_literal::LifetimeOrLabel,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `ContinueExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ContinueExpression {
	pub continue_: ast_token::Continue,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub label:     Option<LifetimeOrLabel>,
}
