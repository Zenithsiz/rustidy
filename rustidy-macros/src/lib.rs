//! Macros for [`rustidy`](../rustidy/index.html)

// Features
#![feature(proc_macro_def_site, extend_one, decl_macro, macro_derive, if_let_guard, yeet_expr)]
// Lints
#![expect(clippy::needless_continue, reason = "Macro-generated code")]

// Modules
mod format;
mod parse;
mod parse_error;
mod parse_recursive;
mod print;
mod util;

#[proc_macro_derive(ParseError, attributes(error, parse_error))]
pub fn derive_parse_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	parse_error::derive(input).unwrap_or_else(|err| panic!("{err:?}"))
}

#[proc_macro_derive(Parse, attributes(parse))]
pub fn derive_parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	parse::derive(input).unwrap_or_else(|err| panic!("{err:?}"))
}

#[proc_macro_derive(ParseRecursive, attributes(parse_recursive))]
pub fn derive_parser_recursive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	parse_recursive::derive(input).unwrap_or_else(|err| panic!("{err:?}"))
}

#[proc_macro_derive(Format, attributes(format))]
pub fn derive_format(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	format::derive(input).unwrap_or_else(|err| panic!("{err:?}"))
}

#[proc_macro_derive(Print, attributes())]
pub fn derive_print(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	print::derive(input).unwrap_or_else(|err| panic!("{err:?}"))
}
