//! String replacements

// Imports
use crate::Config;

/// String replacement
#[derive(Debug, Clone)]
#[derive(derive_more::From)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Replacement {
	#[from]
	Static(&'static str),
	Indentation {
		newlines: usize,
		depth:    usize,
	},

	// Note: Not `#[from]` to make it clear this is expensive
	Dynamic(String),
}

impl Replacement {
	/// Returns if this replacement is equal to `s`
	#[must_use]
	pub fn is(&self, config: &Config, s: &str) -> bool {
		match *self {
			Self::Static(replacement) => replacement == s,
			Self::Indentation { newlines, depth } => {
				let mut chars = s.chars();
				for _ in 0..newlines {
					if chars.next() != Some('\n') {
						return false;
					}
				}

				for _ in 0..depth {
					if !chars.as_str().starts_with(&config.indent) {
						return false;
					}
					_ = chars.advance_by(config.indent.len());
				}

				chars.next().is_none()
			},
			Self::Dynamic(ref replacement) => replacement == s,
		}
	}

	/// Returns if this replacement is blank
	#[must_use]
	pub fn is_blank(&self, _config: &Config) -> bool {
		match *self {
			Self::Static(s) => crate::is_str_blank(s),
			Self::Indentation { .. } => true,
			Self::Dynamic(ref s) => crate::is_str_blank(s),
		}
	}

	/// Writes this replacement onto a string
	pub fn write(&self, config: &Config, output: &mut String) {
		match *self {
			Self::Static(replacement) => *output += replacement,
			Self::Indentation { newlines, depth } => {
				for _ in 0..newlines {
					output.push('\n');
				}
				for _ in 0..depth {
					output.push_str(&config.indent);
				}
			},
			Self::Dynamic(ref replacement) => *output += replacement,
		}
	}

	/// Returns the length of this replacement
	#[must_use]
	pub const fn len(&self) -> usize {
		match *self {
			Self::Static(s) => s.len(),
			Self::Indentation { newlines, depth } => newlines + depth,
			Self::Dynamic(ref s) => s.len(),
		}
	}

	/// Returns if this replacement is empty
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.len() == 0
	}
}
