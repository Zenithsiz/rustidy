//! Format output

// Imports
use rustidy_util::ast_str::AstStrRepr;

/// Formatting output
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[must_use = "Should not ignore format output"]
pub struct FormatOutput {
	/// Prefix whitespace length, if any
	pub prefix_ws_len: Option<usize>,

	/// Total length of this type
	pub len:           usize,

	/// Whether the type was empty
	pub is_empty:      bool,

	/// Whether the type was blank
	pub is_blank:      bool,

	/// Multi-line output
	pub multiline:     Option<FormatMultilineOutput>,
}

impl FormatOutput {
	/// Returns if this format output has any prefix whitespace
	#[must_use]
	pub const fn has_prefix_ws(&self) -> bool {
		self.prefix_ws_len.is_some()
	}

	/// Returns the length of this type, excluding the prefix whitespace, if any
	// TODO: Rename this to just `len` and `Self::len` to `total_len`?.
	#[must_use]
	pub fn len_without_prefix_ws(&self) -> usize {
		self.len - self.prefix_ws_len.unwrap_or(0)
	}

	/// Returns the non-whitespace non-multiline whitespace length of this type
	#[must_use]
	pub fn len_non_multiline_ws(&self) -> usize {
		match self.multiline {
			Some(multiline) => multiline.prefix_len + multiline.suffix_len,
			None => self.len_without_prefix_ws(),
		}
	}

	/// Joins two format outputs.
	///
	/// You must ensure that `rhs` directly follows `lhs`.
	pub const fn join(lhs: Self, rhs: Self) -> Self {
		Self {
			prefix_ws_len: self::join_prefix_ws(lhs.prefix_ws_len, rhs.prefix_ws_len, lhs.len),
			len: lhs.len + rhs.len,
			is_empty: lhs.is_empty && rhs.is_empty,
			is_blank: lhs.is_blank && rhs.is_blank,
			multiline: FormatMultilineOutput::join(lhs.multiline, rhs.multiline, lhs.len, rhs.len),
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
			is_empty: true,
			is_blank: true,
			multiline: None,
		}
	}
}


/// Formatting multi-line output
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct FormatMultilineOutput {
	/// Prefix whitespace length (before the first newline)
	pub prefix_ws_len: Option<usize>,

	/// Prefix length (before the first newline)
	pub prefix_len:    usize,

	/// Suffix length (after the last newline)
	pub suffix_len:    usize,
}

impl FormatMultilineOutput {
	/// Gets the multi-line output of a string
	#[must_use]
	#[expect(clippy::should_implement_trait, reason = "The trait semantics are different")]
	pub fn from_str(s: &str) -> Option<Self> {
		let (prefix, _) = s.split_once('\n')?;
		let (_, suffix) = s.rsplit_once('\n')?;

		Some(Self {
			prefix_ws_len: None,
			prefix_len: prefix.len(),
			suffix_len: suffix.len(),
		})
	}

	/// Joins two multiline outputs.
	#[must_use]
	pub const fn join(
		lhs: Option<Self>,
		rhs: Option<Self>,
		lhs_len: usize,
		rhs_len: usize
	) -> Option<Self> {
		match (lhs, rhs) {
			(Some(lhs), Some(rhs)) => Some(Self {
				prefix_ws_len: self::join_prefix_ws(
					lhs.prefix_ws_len,
					rhs.prefix_ws_len,
					lhs.prefix_len
				),
				prefix_len: lhs.prefix_len,
				suffix_len: rhs.suffix_len
			}),
			(Some(lhs), None) => Some(Self {
				prefix_ws_len: lhs.prefix_ws_len,
				prefix_len: lhs.prefix_len,
				suffix_len: lhs.suffix_len + rhs_len,
			}),
			(None, Some(rhs)) => Some(Self {
				prefix_ws_len: match lhs_len == 0 {
					true => rhs.prefix_ws_len,
					false => None,
				},
				prefix_len: rhs.prefix_len + lhs_len,
				suffix_len: rhs.suffix_len
			}),
			(None, None) => None,
		}
	}

	/// Gets the multi-line output of an ast string repr
	#[must_use]
	pub fn from_ast_str_repr(repr: &AstStrRepr) -> Option<Self> {
		match *repr {
			AstStrRepr::String(ref s) => Self::from_str(s),
			AstStrRepr::Static(s) => Self::from_str(s),
			AstStrRepr::Char(ch) => match ch == '\n' {
				true => Some(Self {
					prefix_ws_len: None,
					prefix_len: 0,
					suffix_len: 0,
				}),
				false => None,
			},
			AstStrRepr::Spaces {
				..
			} => None,
			AstStrRepr::Indentation {
				ref indent,
				newlines,
				depth
			} => match newlines {
				0 => None,
				_ => Some(Self {
					prefix_ws_len: None,
					prefix_len: 0,
					suffix_len: depth * indent.len(),
				})
			},
			AstStrRepr::Join {
				ref lhs,
				ref rhs
			} => Self::join(
				Self::from_ast_str_repr(lhs.repr()),
				Self::from_ast_str_repr(rhs.repr()),
				lhs.len(),
				rhs.len(),
			),
		}
	}
}

/// Joins two prefix whitespace lengths
const fn join_prefix_ws(
	lhs_prefix_ws_len: Option<usize>,
	rhs_prefix_ws_len: Option<usize>,
	lhs_len: usize,
) -> Option<usize> {
	match lhs_prefix_ws_len {
		Some(prefix_ws_len) => Some(prefix_ws_len),
		None => match lhs_len == 0 {
			true => rhs_prefix_ws_len,
			false => None,
		},
	}
}
