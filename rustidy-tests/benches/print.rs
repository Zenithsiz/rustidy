//! Print benches

// Features
#![feature(test)]

// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other tests")]

// Imports
use {
	rustidy_ast::{Crate, expr::Expression},
	rustidy_parse::{Parse, ParseError, Parser, ParserError},
	rustidy_print::Print,
	test::Bencher,
};
extern crate test;

#[bench]
fn print_crate_empty(bencher: &mut Bencher) {
	self::print::<Crate>(bencher, "");
}

#[bench]
fn print_crate_hello_world(bencher: &mut Bencher) {
	self::print::<Crate>(
		bencher,
		r#"fn main() { println!("Hello, World!"); }"#
	);
}

#[bench]
fn print_crate_large(bencher: &mut Bencher) {
	let input = r#"fn main() { println!("Hello, World!"); }"#
		.repeat(100);
	self::print::<Crate>(bencher, &input);
}

#[bench]
fn print_expr_path(bencher: &mut Bencher) {
	self::print::<Expression>(bencher, "a");
}

fn print<T>(bencher: &mut Bencher, input: &str,)
where
	T: Parse + Print,
{
	let mut parser = Parser::new(input);
	let value: T = parser
		.parse::<T>()
		.unwrap_or_else(|err| self::on_err(&parser, &err));

	bencher.iter(|| {
		let mut f = rustidy_print::PrintFmt::new();
		value.print(&mut f);
	});
}

#[cold]
fn on_err<T: Parse>(parser: &Parser, err: &ParserError<T>) -> ! {
	panic!("Unable to parse {:?}: {:?}", parser.input(), err.to_app_error(parser));
}
