//! Continue

// Imports
use {
	crate::ast::{lifetime::LifetimeOrLabel, token},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `ContinueExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ContinueExpression {
	pub continue_: token::Continue,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub label:     Option<LifetimeOrLabel>,
}
