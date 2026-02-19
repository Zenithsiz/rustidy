//! Macros for [`rustidy`](../rustidy/index.html)

// Features
#![feature(
	proc_macro_def_site,
	extend_one,
	decl_macro,
	macro_derive,
	if_let_guard,
	yeet_expr,
	try_find,
	try_blocks,
	super_let,
	exact_size_is_empty
)]

// Lints
#![expect(clippy::needless_continue, reason = "Macro-generated code")]

// Modules
mod format;
mod formattable;
mod parse;
mod parse_error;
mod parse_recursive;
mod print;
mod util;

// Imports
use {app_error::{AppError, app_error}, core::panic::UnwindSafe};

#[proc_macro_derive(ParseError, attributes(error, parse_error))]
pub fn derive_parse_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	self::try_derive(input, parse_error::derive)
}

#[proc_macro_derive(Parse, attributes(parse))]
pub fn derive_parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	self::try_derive(input, parse::derive)
}

#[proc_macro_derive(ParseRecursive, attributes(parse_recursive))]
pub fn derive_parser_recursive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	self::try_derive(input, parse_recursive::derive)
}

#[proc_macro_derive(Formattable, attributes(formattable))]
pub fn derive_formattable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	self::try_derive(input, formattable::derive)
}

#[proc_macro_derive(Format, attributes(format))]
pub fn derive_format(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	self::try_derive(input, format::derive)
}

#[proc_macro_derive(Print, attributes())]
pub fn derive_print(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	self::try_derive(input, print::derive)
}

fn try_derive(input: proc_macro::TokenStream, f: impl FnOnce(proc_macro::TokenStream) -> Result<proc_macro::TokenStream, AppError> + UnwindSafe,) -> proc_macro::TokenStream {
	std::panic::catch_unwind(move || f(input))
		.map_err(|payload| app_error!("Derive macro panicked: {payload:?}"))
		.flatten()
		.unwrap_or_else(|err| {
			let err = err.to_string();
			quote::quote! {
				compile_error! { #err }
			}
				.into()
		})
}
