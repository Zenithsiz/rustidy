//! Syntax tree

// Lints
#![expect(clippy::large_stack_frames, reason = "We'll make everything smaller eventually")]

// Modules
pub mod at_least;
pub mod attr;
pub mod delimited;
pub mod expr;
pub mod ident;
pub mod item;
pub mod lifetime;
pub mod line;
pub mod longest;
pub mod pat;
pub mod path;
pub mod punct;
pub mod shebang;
pub mod stmt;
pub mod token;
pub mod ty;
pub mod vis;
pub mod whitespace;
pub mod with_attrs;

// Imports
use {
	self::{
		attr::InnerAttrOrDocComment,
		item::Item,
		shebang::Shebang,
		whitespace::{TrailingLineComment, Whitespace},
	},
	crate::{Format, parser::Parse, print::Print},
	core::fmt::Debug,
};

/// `Crate`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a crate")]
pub struct Crate {
	pub shebang:               Option<Shebang>,
	pub inner_attrs:           Vec<InnerAttrOrDocComment>,
	pub items:                 Vec<Item>,
	#[format(whitespace)]
	pub suffix_ws:             Whitespace,
	pub trailing_line_comment: Option<TrailingLineComment>,
}
