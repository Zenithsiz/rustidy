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
	continue_: token::Continue,
	label:     Option<LifetimeOrLabel>,
}
