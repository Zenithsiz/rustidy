//! Syntax tree

// Features
#![recursion_limit = "512"]
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
	str_as_str,
	thread_local,
	type_changing_struct_update,
	try_blocks
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

// Imports
use {
	self::{attr::InnerAttrOrDocComment, item::Items, shebang::Shebang},
	core::fmt::Debug,
	rustidy_format::{Format, FormatOutput, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Crate`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
#[parse(name = "a crate")]
pub struct Crate {
	pub shebang:     Option<Shebang>,
	pub inner_attrs: Vec<InnerAttrOrDocComment>,
	pub items:       Items,
	pub suffix_ws:   Whitespace,
}

impl Format<(), ()> for Crate {
	fn format(&mut self, ctx: &mut rustidy_format::Context, _prefix_ws: (), _args: ()) -> FormatOutput {
		let mut ctx = ctx.sub_context();
		for attr in &self.inner_attrs {
			if let Some(attr) = attr.try_as_attr_ref() && let Err(err) = attr::update_config(&attr.attr.value, &mut ctx) {
				tracing::warn!("Malformed `#![rustidy::config(...)]` attribute: {err:?}");
			}
		}

		let mut output = FormatOutput::default();

		self
			.shebang
			.format(&mut ctx, (), ())
			.append_to(&mut output);

		self
			.inner_attrs
			.format(&mut ctx, Whitespace::REMOVE, rustidy_format::vec::args_prefix_ws(Whitespace::INDENT),)
			.append_to(&mut output);

		// TODO: We need to set `FormatTag::AfterNewline` for `items`.
		self
			.items
			.format(&mut ctx, match self.inner_attrs.is_empty() {
				true => Whitespace::REMOVE,
				false => Whitespace::INDENT,
			}, (),)
			.append_to(&mut output);

		self
			.suffix_ws
			.format(&mut ctx, Whitespace::indent(output.is_empty), ())
			.append_to(&mut output);

		output
	}
}
