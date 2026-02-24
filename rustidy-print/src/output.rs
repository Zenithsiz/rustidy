//! Print output

// Imports
use arcstr::Substr;

/// Print output
#[derive(Debug)]
pub enum PrintOutput {
	/// Empty string
	Empty,

	/// Input range
	Input(Substr),

	/// String
	String(String),
}

impl PrintOutput {
	/// Mutates this output into a string
	pub fn make_string(&mut self) -> &mut String {
		match self {
			Self::Empty => *self = Self::String(String::new()),
			Self::Input(s) => {
				let s = s.as_str().to_owned();
				*self = Self::String(s);
			},
			Self::String(_) => (),
		}

		match self {
			Self::String(s) => s,
			_ => unreachable!()
		}
	}

	/// Gets the output as a string
	#[must_use]
	pub fn as_str(&self) -> &str {
		match self {
			Self::Empty => "",
			Self::Input(s) => s,
			Self::String(s) => s,
		}
	}
}
