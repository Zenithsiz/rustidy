//! Continue

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{lifetime::LifetimeOrLabel, token},
};

/// `ContinueExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ContinueExpression {
	pub continue_: token::Continue,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub label:     Option<LifetimeOrLabel>,
}
