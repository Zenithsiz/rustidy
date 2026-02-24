//! Format output

/// Formatting output
#[derive(Clone, Copy, Debug)]
#[must_use = "Should not ignore format output"]
pub struct FormatOutput {
	/// Prefix whitespace length, if any
	pub prefix_ws_len: Option<usize>,

	/// Total length of this type
	pub len:           usize,

	/// Number of newlines in the input
	pub newlines:      usize,

	/// Whether the type was empty
	pub is_empty:      bool,

	/// Whether the type was blank
	pub is_blank:      bool,
}

impl FormatOutput {
	/// Returns if this format output has any prefix whitespace
	#[must_use]
	pub const fn has_prefix_ws(&self) -> bool {
		self.prefix_ws_len.is_some()
	}

	/// Returns if this format output has any newlines
	#[must_use]
	pub const fn has_newlines(&self) -> bool {
		self.newlines != 0
	}

	/// Returns the length of this type, excluding the prefix whitespace, if any
	// TODO: Rename this to just `len` and `Self::len` to `total_len`?.
	#[must_use]
	pub fn len_without_prefix_ws(&self) -> usize {
		self.len - self.prefix_ws_len.unwrap_or(0)
	}

	/// Joins two format outputs.
	///
	/// You must ensure that `rhs` directly follows `lhs`.
	pub const fn join(lhs: Self, rhs: Self) -> Self {
		Self {
			prefix_ws_len: match lhs.prefix_ws_len {
				Some(prefix_ws_len) => Some(prefix_ws_len),
				None => match lhs.len == 0 {
					true => rhs.prefix_ws_len,
					false => None,
				},
			},
			len: lhs.len + rhs.len,
			newlines: lhs.newlines + rhs.newlines,
			is_empty: lhs.is_empty && rhs.is_empty,
			is_blank: lhs.is_blank && rhs.is_blank,
		}
	}

	/// Appends a format output to this one.
	///
	/// See [`join`](Self::join) for details.
	pub const fn append(&mut self, other: Self) {
		*self = Self::join(*self, other);
	}

	/// Appends this format output to `output`.
	///
	/// See [`join`](Self::join) for details.
	pub const fn append_to(self, output: &mut Self) {
		output.append(self);
	}
}

impl<const N: usize> From<[Self; N]> for FormatOutput {
	fn from(outputs: [Self; N]) -> Self {
		outputs.into_iter().collect()
	}
}

impl FromIterator<Self> for FormatOutput {
	fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
		iter
			.into_iter()
			.fold(Self::default(), Self::join)
	}
}

impl Default for FormatOutput {
	fn default() -> Self {
		Self {
			prefix_ws_len: None,
			len: 0,
			newlines: 0,
			is_empty: true,
			is_blank: true,
		}
	}
}
