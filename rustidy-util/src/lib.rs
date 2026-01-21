//! Utilities

// Features
#![feature(iter_advance_by)]

// Modules
pub mod arena;
pub mod ast_pos;
pub mod ast_range;
pub mod ast_str;
pub mod config;
pub mod replacement;

// Exports
pub use self::{
	arena::{Arena, ArenaData, ArenaIdx, ArenaRef},
	ast_pos::AstPos,
	ast_range::AstRange,
	ast_str::AstStr,
	config::Config,
	replacement::{Replacement, Replacements},
};
