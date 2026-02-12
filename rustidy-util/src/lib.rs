//! Utilities

// Features
#![feature(iter_advance_by, decl_macro, macro_metavar_expr_concat, if_let_guard)]

// Modules
pub mod arena;
pub mod ast_pos;
pub mod ast_range;
pub mod ast_str;
pub mod config;
pub mod whitespace;

// Exports
pub use self::{
	arena::{Arena, ArenaData, ArenaIdx, ArenaRef},
	ast_pos::AstPos,
	ast_range::AstRange,
	ast_str::AstStr,
	config::Config,
	whitespace::Whitespace,
};

/// Returns if a string is blank
#[must_use]
pub fn is_str_blank(s: &str) -> bool {
	s.chars().all(|ch| ch.is_ascii_whitespace())
}
