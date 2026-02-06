//! Syntax tree

// Features
#![cfg_attr(doc, recursion_limit = "512")]
#![feature(
	never_type,
	decl_macro,
	macro_metavar_expr,
	macro_metavar_expr_concat,
	yeet_expr,
	pattern,
	unwrap_infallible,
	substr_range,
	try_trait_v2,
	iter_array_chunks,
	try_trait_v2_residual,
	associated_type_defaults,
	macro_derive,
	debug_closure_helpers,
	trait_alias,
	anonymous_lifetime_in_impl_trait,
	exact_size_is_empty,
	coverage_attribute,
	is_ascii_octdigit,
	trim_prefix_suffix,
	if_let_guard,
	str_as_str
)]

// Modules
pub mod attr;
pub mod expr;
pub mod item;
pub mod lifetime;
pub mod pat;
pub mod path;
pub mod shebang;
pub mod stmt;
pub mod token;
pub mod ty;
pub mod util;
pub mod vis;
pub mod with_attrs;

// Imports
use {
	self::{attr::InnerAttrOrDocComment, item::Items, shebang::Shebang},
	core::fmt::Debug,
	rustidy_ast_util::{
		Whitespace,
		whitespace::{self, TrailingLineComment},
	},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{AstStr, ast_str::AstStrRepr},
};

/// `Crate`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a crate")]
#[format(and_with = Self::format_first_inner_attr_or_item)]
pub struct Crate {
	pub shebang:               Option<Shebang>,
	#[format(and_with = rustidy_format::format_vec_each_with_all(Format::prefix_ws_set_cur_indent))]
	pub inner_attrs:           Vec<InnerAttrOrDocComment>,
	pub items:                 Items,
	#[format(and_with = whitespace::set_indent(0, self.shebang.is_none() && self.inner_attrs.is_empty() && self.items.0.is_empty()))]
	pub suffix_ws:             Whitespace,
	#[format(and_with = Self::format_trailing_line_comment)]
	pub trailing_line_comment: Option<TrailingLineComment>,
}

impl Crate {
	fn format_first_inner_attr_or_item(&mut self, ctx: &mut rustidy_format::Context) {
		if let Some(attr) = self.inner_attrs.first_mut() {
			attr.prefix_ws_remove(ctx);
		} else if let Some(item) = self.items.0.first_mut() {
			item.prefix_ws_remove(ctx);
		}
	}

	fn format_trailing_line_comment(trailing: &mut Option<TrailingLineComment>, ctx: &mut rustidy_format::Context) {
		let Some(trailing) = trailing else { return };

		let mut s = ctx.str(&trailing.0).into_owned();

		// Add the newline at the end of the trailing comment if it didn't have one already
		if !s.ends_with('\n') {
			s.push('\n');
			trailing.0 = AstStr::new(AstStrRepr::Dynamic(s));
		}
	}
}
