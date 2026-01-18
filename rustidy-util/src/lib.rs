//! Utilities

// Modules
// TODO: Rename the `parser_` to `ast_`.
pub mod arena;
pub mod parser_pos;
pub mod parser_range;
pub mod parser_str;

// Exports
pub use self::{
	arena::{Arena, ArenaData, ArenaIdx, ArenaRef},
	parser_pos::ParserPos,
	parser_range::ParserRange,
	parser_str::ParserStr,
};
