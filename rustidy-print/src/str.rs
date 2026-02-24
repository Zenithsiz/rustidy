//! String impls

// Imports
use {crate::PrintOutput, arcstr::Substr, rustidy_util::AstStr};

/// Writes a string onto an output
pub fn write(s: &AstStr, output: &mut PrintOutput) {
	// If the string is empty, we can just skip it
	if s.is_empty() {
		return;
	}

	match s.input() {
		// If the input wasn't modified, check what we have in the output
		Some(input) if s.repr().is_str(input) => match output {
			// If the output is empty, just set it to the input range
			PrintOutput::Empty => *output = PrintOutput::Input(Substr::clone(input)),

			// If we already had a range, and this one is contiguously after,
			// extend it
			PrintOutput::Input(s) if s.range().end == input.range().start => {
				let s = s
					.parent()
					.substr(s.range().start..input.range().end);
				*output = PrintOutput::Input(s);
			},

			// Otherwise, just write it as a string.
			_ => s.write(output.make_string()),
		},

		_ => s.write(output.make_string()),
	}
}
