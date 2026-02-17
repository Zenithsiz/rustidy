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

// Imports
use {
	self::{attr::InnerAttrOrDocComment, item::Items, shebang::Shebang},
	core::fmt::Debug,
	rustidy_format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{AstStr, Whitespace, ast_str::AstStrRepr},
};

/// `Crate`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
#[parse(name = "a crate")]
pub struct Crate(pub CrateInner);

impl Format<()> for Crate {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		_args: &mut (),
	) -> FormatOutput {
		let mut inner_ctx = ctx.sub_context();
		for attr in &self.0.inner_attrs {
			if let Some(attr) = attr.try_as_attr_ref() &&
				let Err(err) = attr::update_config(&attr.attr.value, &mut inner_ctx)
			{
				tracing::warn!("Malformed `#![rustidy::config(...)]` attribute: {err:?}");
			}
		}

		// TODO: This also needs to set `FormatTag::AfterNewline` for `items`.
		self.0.format(&mut inner_ctx, prefix_ws, &mut ())
	}
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct CrateInner {
	pub shebang:               Option<Shebang>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = rustidy_format::vec::args_prefix_ws(Whitespace::CUR_INDENT))]
	pub inner_attrs:           Vec<InnerAttrOrDocComment>,
	#[format(prefix_ws = match self.inner_attrs.is_empty() {
		true => Whitespace::REMOVE,
		false => Whitespace::CUR_INDENT,
	})]
	pub items:                 Items,
	#[format(prefix_ws = Whitespace::indent(0, self.shebang.is_none() && self.inner_attrs.is_empty() && self.items.0.is_empty()))]
	#[format(whitespace)]
	pub suffix_ws:             Whitespace,
	#[format(prefix_ws = Whitespace::PRESERVE)]
	// TODO: This should be a `before_with`
	#[format(with = Self::format_trailing_line_comment)]
	pub trailing_line_comment: Option<TrailingLineComment>,
}

impl CrateInner {
	fn format_trailing_line_comment(
		trailing: &mut Option<TrailingLineComment>,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		(): &mut (),
	) -> FormatOutput {
		let Some(trailing) = trailing else {
			return FormatOutput::default();
		};

		let mut s = ctx.str(&trailing.0).into_owned();

		// Add the newline at the end of the trailing comment if it didn't have one already
		if !s.ends_with('\n') {
			s.push('\n');
			trailing.0.replace(ctx.input(), AstStrRepr::Dynamic(s));
		}

		trailing.format(ctx, prefix_ws, &mut ())
	}
}

/// Trailing line comment
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(error(name = NoComment, fmt = "Expected `//` (except `///` or `//!`)"))]
pub struct TrailingLineComment(
	#[parse(try_update_with = Self::parse)]
	pub AstStr,
);

impl TrailingLineComment {
	fn parse(s: &mut &str) -> Result<(), TrailingLineCommentError> {
		let is_doc_comment = (s.starts_with("///") && !s.starts_with("////")) || s.starts_with("//!");
		match s.starts_with("//") && !is_doc_comment {
			true => {
				*s = &s[s.len()..];
				Ok(())
			},
			false => Err(TrailingLineCommentError::NoComment),
		}
	}
}
