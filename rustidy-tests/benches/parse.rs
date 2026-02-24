//! Parse benches

// Features
#![feature(test)]

// Lints
#![expect(unused_crate_dependencies, reason = "They're used in other tests")]

// Imports
use {
	rustidy_ast::{Crate, expr::Expression},
	rustidy_parse::{Parse, ParseError, Parser, ParserError},
	rustidy_util::{AstPos, Whitespace},
	test::Bencher,
};
extern crate test;

#[bench]
fn parse_whitespace_empty(bencher: &mut Bencher) {
	self::parse::<Whitespace>(bencher, "");
}

#[bench]
fn parse_crate_empty(bencher: &mut Bencher) {
	self::parse::<Crate>(bencher, "");
}

#[bench]
fn parse_crate_hello_world(bencher: &mut Bencher) {
	self::parse::<Crate>(bencher, r#"fn main() { println!("Hello, world!"); }"#);
}

#[bench]
fn parse_expr_path(bencher: &mut Bencher) {
	self::parse::<Expression>(bencher, "a");
}

#[bench]
fn parse_expr_addition(bencher: &mut Bencher) {
	self::parse::<Expression>(bencher, "1 + 1");
}


fn parse<T: Parse>(bencher: &mut Bencher, input: &str) {
	let mut parser = Parser::new(input);
	bencher
		.iter(|| {
			parser.set_pos(AstPos(0));
			let _: T = parser
				.parse::<T>()
				.unwrap_or_else(|err| self::on_err(&parser, &err));
		});
}

#[cold]
fn on_err<T: Parse>(parser: &Parser, err: &ParserError<T>) -> ! {
	panic!("Unable to parse {:?}: {:?}", parser.input(), err.to_app_error(parser));
}
