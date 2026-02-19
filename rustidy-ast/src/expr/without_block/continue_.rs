//! Continue

// Imports
use {
	crate::{lifetime::LifetimeOrLabel, token},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ContinueExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ContinueExpression {
	pub continue_: token::Continue,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub label:     Option<LifetimeOrLabel>,
}
