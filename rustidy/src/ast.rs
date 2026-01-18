//! Syntax tree

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
	crate::{Format, format},
	core::fmt::Debug,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `Crate`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a crate")]
#[format(and_with = Self::format_first_inner_attr_or_item)]
pub struct Crate {
	pub shebang:               Option<Shebang>,
	#[format(and_with = format::format_vec_each_with_all(Format::prefix_ws_set_cur_indent))]
	pub inner_attrs:           Vec<InnerAttrOrDocComment>,
	#[format(and_with = format::format_vec_each_with_all(Format::prefix_ws_set_cur_indent))]
	pub items:                 Vec<Item>,
	#[format(and_with = whitespace::set_indent(0, self.shebang.is_none() && self.inner_attrs.is_empty() && self.items.is_empty()))]
	pub suffix_ws:             Whitespace,
	#[format(and_with = Self::format_trailing_line_comment)]
	pub trailing_line_comment: Option<TrailingLineComment>,
}

impl Crate {
	fn format_first_inner_attr_or_item(&mut self, ctx: &mut format::Context) {
		if let Some(attr) = self.inner_attrs.first_mut() {
			attr.prefix_ws_remove(ctx);
		} else if let Some(item) = self.items.first_mut() {
			item.prefix_ws_remove(ctx);
		}
	}

	fn format_trailing_line_comment(trailing: &mut Option<TrailingLineComment>, ctx: &mut format::Context) {
		let Some(trailing) = trailing else { return };

		let mut s = ctx.str(&trailing.0).to_owned();

		// Add the newline at the end of the trailing comment if it didn't have one already
		if !s.ends_with('\n') {
			s.push('\n');
			ctx.replace(&trailing.0, s);
		}
	}
}
