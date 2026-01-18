//! Ast position

/// Ast position
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(derive_more::From)]
#[serde(transparent)]
pub struct AstPos(pub usize);

impl AstPos {
	/// Creates an ast position from a usize
	// TODO: Should we allow this?
	#[must_use]
	pub const fn from_usize(pos: usize) -> Self {
		Self(pos)
	}

	/// Returns the index corresponding to this position
	// TODO: Should we allow this?
	#[must_use]
	pub const fn to_usize(self) -> usize {
		self.0
	}
}
