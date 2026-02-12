//! Formatter tags

/// Formatter tag
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
