//! Format benches

// Features
#![feature(test)]

// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other tests")]

// Imports
use {
	rustidy_ast::{Crate, expr::Expression, item::UseDeclaration},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::{Parse, ParseError, Parser, ParserError},
	rustidy_util::{Config, Whitespace},
	test::Bencher,
};
extern crate test;

#[bench]
fn format_crate_empty(bencher: &mut Bencher) {
	self::format::<Crate>(bencher, "");
}

#[bench]
fn format_expr_chain(bencher: &mut Bencher) {
	self::format_with::<Expression, _, _>(bencher, "a.b.c", Whitespace::PRESERVE, ());
}

#[bench]
fn format_expr_path(bencher: &mut Bencher) {
	self::format_with::<Expression, _, _>(bencher, "a", Whitespace::PRESERVE, ());
}

#[bench]
fn format_use_decl_simple(bencher: &mut Bencher) {
	self::format_with::<UseDeclaration, _, _>(bencher, "use a;", Whitespace::PRESERVE, ());
}

#[bench]
fn format_use_decl_group(bencher: &mut Bencher) {
	self::format_with::<UseDeclaration, _, _>(bencher, "use {a, b, c};", Whitespace::PRESERVE, ());
}

fn format<T: Parse + Format<(), ()>>(bencher: &mut Bencher, input: &str) {
	self::format_with::<T, _, _>(bencher, input, (), ());
}

fn format_with<T, PrefixWs, Args>(
	bencher: &mut Bencher,
	input: &str,
	prefix_ws: PrefixWs,
	args: Args
)
where
	T: Parse + Format<PrefixWs, Args>,
	PrefixWs: Copy,
	Args: Copy
{
	let mut parser = Parser::new(input);
	let mut value: T = parser
		.parse::<T>()
		.unwrap_or_else(|err| self::on_err(&parser, &err));

	let config = Config::default();
	let mut ctx = rustidy_format::Context::new(input, &config);
	bencher
		.iter(|| {
			value.format(&mut ctx, prefix_ws, args)
		});
}

#[cold]
fn on_err<T: Parse>(parser: &Parser, err: &ParserError<T>) -> ! {
	panic!("Unable to parse {:?}: {:?}", parser.input(), err.to_app_error(parser));
}
