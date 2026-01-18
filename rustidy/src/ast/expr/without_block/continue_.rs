//! Continue

// Imports
use {
	crate::{
		Format,
		Print,
		ast::{lifetime::LifetimeOrLabel, token},
	},
	rustidy_parse::Parse,
};

/// `ContinueExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ContinueExpression {
	pub continue_: token::Continue,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub label:     Option<LifetimeOrLabel>,
}
