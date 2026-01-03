//! Literal expression

// Modules
pub mod byte;
pub mod char;
pub mod escape;
pub mod float;
pub mod int;
pub mod raw_string;
pub mod string;
pub mod suffix;

// Exports
pub use self::{
	byte::ByteLiteral,
	char::CharLiteral,
	escape::{AsciiEscape, ByteEscape, QuoteEscape, StringContinue, UnicodeEscape},
	float::FloatLiteral,
	int::IntegerLiteral,
	raw_string::RawStringLiteral,
	string::StringLiteral,
	suffix::{Suffix, SuffixNoE},
};

// Imports
use crate::{Format, Parse, Print, ast::token};

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LiteralExpression {
	Float(FloatLiteral),

	Char(CharLiteral),
	String(StringLiteral),
	RawString(RawStringLiteral),
	Byte(ByteLiteral),
	ByteString(!),
	RawByteString(!),
	CString(!),
	RawCString(!),
	Integer(IntegerLiteral),
	True(token::True),
	False(token::False),
}

/// `TUPLE_INDEX`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleIndex(pub IntegerLiteral);
