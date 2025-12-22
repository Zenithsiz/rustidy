//! Format configuration

/// Format config
#[derive(Clone, Debug)]
pub struct Config {
	/// Indentation string
	pub indent:             String,
	pub empty_line_spacing: EmptyLineSpacing,
}

/// Empty line spacing.
///
/// Keeps at least `min` empty lines in between consecutive things,
/// and at most `max` (inclusive).
// TODO: Should we allow this being different for items and statements?
#[derive(Clone, Debug)]
pub struct EmptyLineSpacing {
	pub min: usize,
	pub max: usize,
}


impl Default for Config {
	fn default() -> Self {
		Self {
			indent:             "\t".to_owned(),
			empty_line_spacing: EmptyLineSpacing { min: 1, max: 2 },
		}
	}
}
