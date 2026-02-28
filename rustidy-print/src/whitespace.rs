//! Whitespace impls

// Imports
use {
	crate::Print,
	util::{
		Whitespace,
		whitespace::{BlockComment, Comment, LineComment, PureWhitespace, WhitespaceInner},
	},
};

impl Print for Whitespace {
	fn print(&self, f: &mut crate::PrintFmt) {
		self.0.print(f);
	}

	fn print_non_ws(&self, _f: &mut crate::PrintFmt) {
		// Note: Don't print any whitespace
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

	fn print_non_ws(&self, _f: &mut crate::PrintFmt) {}
}

impl Print for Comment {
	fn print(&self, f: &mut crate::PrintFmt) {
		match self {
			Self::Line(comment) => comment.print(f),
			Self::Block(comment) => comment.print(f),
		}
	}

	fn print_non_ws(&self, _f: &mut crate::PrintFmt) {}
}

impl Print for BlockComment {
	fn print(&self, f: &mut crate::PrintFmt) {
		self.0.print(f);
	}

	fn print_non_ws(&self, _f: &mut crate::PrintFmt) {}
}

impl Print for LineComment {
	fn print(&self, f: &mut crate::PrintFmt) {
		self.0.print(f);
	}

	fn print_non_ws(&self, _f: &mut crate::PrintFmt) {}
}

impl Print for PureWhitespace {
	fn print(&self, f: &mut crate::PrintFmt) {
		self.0.print(f);
	}

	fn print_non_ws(&self, _f: &mut crate::PrintFmt) {}
}
