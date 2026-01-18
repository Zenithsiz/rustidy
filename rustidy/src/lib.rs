//! Rust-tidy formatter

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
	push_mut,
	anonymous_lifetime_in_impl_trait,
	exact_size_is_empty,
	coverage_attribute,
	is_ascii_octdigit,
	trim_prefix_suffix
)]

// Modules
pub mod ast;
pub mod format;

// Exports
pub use self::format::{Format, FormatRef};

// Imports
use {
	app_error::{AppError, Context, app_error},
	rustidy_parse::{ParseError, Parser},
	std::path::Path,
};

/// Parses a file
pub fn parse(file_path: &Path, parser: &mut Parser) -> Result<ast::Crate, AppError> {
	// TODO: Once we have more things in arenas, we can probably remove this
	stacker::grow(16 * 1024 * 1024, || {
		parser
			.parse::<ast::Crate>()
			.map_err(|err| {
				if let Some(pos) = err.pos() {
					parser.set_pos(pos);
				}
				parser.reverse_whitespace();

				err.to_app_error(parser)
					.with_context(|| self::parser_error_ctx(file_path, parser))
			})
			.and_then(|ast| match parser.is_finished() {
				true => Ok(ast),
				false => Err(app_error!("Unexpected tokens at the end of file")
					.with_context(|| self::parser_error_ctx(file_path, parser))),
			})
			.context("Unable to parse ast")
	})
}

fn parser_error_ctx(file_path: &Path, parser: &Parser) -> String {
	let line = parser.cur_line();
	let loc = parser.cur_loc();

	let ident = line[..loc.column]
		.chars()
		.map(|ch| match ch.is_whitespace() {
			true => ch,
			false => ' ',
		})
		.collect::<String>();

	format!("Error at {}:{loc}:\n{line}\n{ident}^", file_path.display())
}
