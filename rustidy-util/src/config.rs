//! Formatter configuration

// Imports
use std::sync::Arc;

/// Formatter configuration
#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
	/// Indentation string
	pub indent:             Arc<str>,
	pub empty_line_spacing: EmptyLineSpacing,
	pub max_use_tree_len:   usize,
	pub array_expr_cols:    Option<usize>,
}

/// Empty line spacing.
///
/// Keeps at least `min` empty lines in between consecutive things,
/// and at most `max` (inclusive).
// TODO: Should we allow this being different for items and statements?
#[derive(Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmptyLineSpacing {
	pub min: usize,
	pub max: usize,
}


impl Default for Config {
	fn default() -> Self {
		Self {
			indent:             "\t".into(),
			empty_line_spacing: EmptyLineSpacing { min: 0, max: 2 },
			max_use_tree_len:   75,
			array_expr_cols:    None,
		}
	}
}
