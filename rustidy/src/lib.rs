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
	anonymous_lifetime_in_impl_trait,
	exact_size_is_empty,
	coverage_attribute,
	is_ascii_octdigit,
	trim_prefix_suffix,
	if_let_guard
)]

// Imports
use {
	app_error::{AppError, app_error},
	rustidy_ast::Crate,
	rustidy_format::{Format, FormatOutput},
	rustidy_parse::{ParseError, Parser},
	rustidy_util::Config,
	std::path::Path,
};

/// Formats the crate `crate_`.
pub fn format(input: &str, config: &Config, crate_: &mut Crate) -> FormatOutput {
	let mut ctx = rustidy_format::Context::new(input, config);
	crate_.format(&mut ctx, (), ())
}

/// Parses the input `input`.
///
/// `file` is only used for error reporting and does not have to exist.
pub fn parse(input: &str, file: &Path) -> Result<rustidy_ast::Crate, AppError> {
	let mut parser = Parser::new(input);
	parser
		.parse::<rustidy_ast::Crate>()
		.map_err(|err| {
			if let Some(pos) = err.pos() {
				parser.set_pos(pos);
			}
			parser.reverse_whitespace();

			err.to_app_error(&parser)
				.with_context(|| self::parser_error_ctx(file, &parser))
		})
		.and_then(|ast| match parser.is_finished() {
			true => Ok(ast),
			false => Err(app_error!("Unexpected tokens at the end of file")
				.with_context(|| self::parser_error_ctx(file, &parser))),
		})
}

/// Creates context for a parser error.
fn parser_error_ctx(file: &Path, parser: &Parser) -> String {
	let line = parser.cur_line();
	let line_indent = line.find(|ch: char| !ch.is_ascii_whitespace()).unwrap_or(line.len());

	let loc = parser.cur_loc();

	let indent = line[..loc.column][line_indent..]
		.chars()
		.map(|ch| match ch.is_whitespace() {
			true => ch,
			false => ' ',
		})
		.collect::<String>();

	format!(
		"Error at {}:{loc}:\n    |\n{:>3} | {}\n    | {indent}^",
		file.display(),
		loc.line + 1,
		&line[line_indent..]
	)
}
