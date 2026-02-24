//! Utilities

// Features
#![feature(
	iter_advance_by,
	decl_macro,
	macro_metavar_expr_concat,
	if_let_guard,
	macro_derive,
	thread_local,
	negative_impls,
	unsafe_cell_access
)]

// Modules
pub mod arena;
pub mod ast_pos;
pub mod ast_range;
pub mod ast_str;
pub mod config;
pub mod oob;
pub mod whitespace;

// Exports
pub use self::{
	arena::{Arena, ArenaData, ArenaIdx, decl_arena},
	ast_pos::AstPos,
	ast_range::AstRange,
	ast_str::AstStr,
	config::Config,
	oob::Oob,
	whitespace::Whitespace,
};

// Imports
use core::iter;

/// Returns if a string is blank
#[must_use]
pub fn is_str_blank(s: &str) -> bool {
	s.chars().all(|ch| ch.is_ascii_whitespace())
}

/// Counts the number of newlines in a string
#[must_use]
pub fn str_count_newlines(s: &str) -> usize {
	s.chars().filter(|&ch| ch == '\n').count()
}

#[extend::ext(name = StrPopFirst)]
pub impl &str {
	fn pop_first(&mut self) -> Option<char> {
		let mut chars = self.chars();
		let ch = chars.next()?;
		*self = chars.as_str();

		Some(ch)
	}
}

#[extend::ext(name = StrChunk)]
pub impl str {
	fn chunk(&self, len: usize) -> impl Iterator<Item = &str> {
		let mut s = self;
		iter::from_fn(
			move || match s.is_empty() {
				true => None,
				false => {
					let (cur, next) = s
						.split_at_checked(len)
						.unwrap_or_else(|| (s, &s[s.len()..]));
					s = next;
					Some(cur)
				}
			}
		)
	}
}
