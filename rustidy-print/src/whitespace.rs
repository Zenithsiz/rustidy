//! Whitespace impls

// Imports
use {
	crate::Print,
	rustidy_util::{
		Whitespace,
		whitespace::{BlockComment, Comment, LineComment, PureWhitespace, WhitespaceInner},
	},
};

impl Print for Whitespace {
	fn print(&self, f: &mut crate::PrintFmt) {
		self.0.print(f);
	}
}

impl Print for WhitespaceInner {
	fn print(&self, f: &mut crate::PrintFmt) {
		self.first.print(f);
		for (comment, pure) in &self.rest {
			comment.print(f);
			pure.print(f);
		}
	}
}

impl Print for Comment {
	fn print(&self, f: &mut crate::PrintFmt) {
		match self {
			Self::Line(comment) => comment.print(f),
			Self::Block(comment) => comment.print(f),
		}
	}
}

impl Print for BlockComment {
	fn print(&self, f: &mut crate::PrintFmt) {
		self.0.print(f);
	}
}

impl Print for LineComment {
	fn print(&self, f: &mut crate::PrintFmt) {
		self.0.print(f);
	}
}

impl Print for PureWhitespace {
	fn print(&self, f: &mut crate::PrintFmt) {
		self.0.print(f);
	}
}
