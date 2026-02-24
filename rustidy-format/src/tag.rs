//! Formatter tags

// Imports
use core::mem;

/// Formatter tag
// TODO: These should be separated by tags that need scopes,
//       and tags that are pushed/popped.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum FormatTag {
	InsideChain,

	// Note: This attribute only works because every time
	//       we apply it, there's always whitespace directly
	//       after to remove it, otherwise it would stay for
	//       too long and be applied when it's no longer relevant.
	// TODO: Ideally, we'd assign some "position" to this, but
	//       during formatting, we no longer necessarily have
	//       the input ranges.
	AfterNewline,
}

/// Formatter tags
#[derive(Clone, Copy, Debug)]
pub struct FormatTags {
	pub inside_chain:  bool,
	pub after_newline: bool,
}

impl FormatTags {
	/// Creates new, empty, tags
	#[must_use]
	pub const fn new() -> Self {
		Self { inside_chain: false, after_newline: false, }
	}

	/// Adds a tag.
	///
	/// Returns if the tag was present
	pub const fn add(&mut self, tag: FormatTag) -> bool {
		self.set(tag, true)
	}

	/// Removes a tag.
	///
	/// Returns if the tag was present
	pub const fn remove(&mut self, tag: FormatTag) -> bool {
		self.set(tag, false)
	}

	/// Sets whether a tag is present or not.
	///
	/// Returns if the tag was present.
	pub const fn set(&mut self, tag: FormatTag, present: bool) -> bool {
		match tag {
			FormatTag::InsideChain => mem::replace(&mut self.inside_chain, present),
			FormatTag::AfterNewline => mem::replace(&mut self.after_newline, present),
		}
	}

	/// Returns if a tag exists
	#[must_use]
	pub const fn contains(&self, tag: FormatTag) -> bool {
		match tag {
			FormatTag::InsideChain => self.inside_chain,
			FormatTag::AfterNewline => self.after_newline,
		}
	}
}

impl Default for FormatTags {
	fn default() -> Self {
		Self::new()
	}
}
