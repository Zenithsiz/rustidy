//! Literal expression

// Modules
pub mod byte;
pub mod byte_string;
pub mod c_string;
pub mod char;
pub mod escape;
pub mod float;
pub mod int;
pub mod raw_byte_string;
pub mod raw_c_string;
pub mod raw_string;
pub mod string;
pub mod suffix;

// Exports
pub use self::{
	byte::ByteLiteral,
	byte_string::ByteStringLiteral,
	c_string::CStringLiteral,
	char::CharLiteral,
	escape::{
		AsciiEscape,
		ByteEscape,
		NonNulByteEscape,
		NonNulUnicodeEscape,
		QuoteEscape,
		StringContinue,
		UnicodeEscape,
	},
	float::FloatLiteral,
	int::IntegerLiteral,
	raw_byte_string::RawByteStringLiteral,
	raw_c_string::RawCStringLiteral,
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
	ByteString(ByteStringLiteral),
	RawByteString(RawByteStringLiteral),
	CString(CStringLiteral),
	RawCString(RawCStringLiteral),
	Integer(IntegerLiteral),
	True(token::True),
	False(token::False),
}

/// `TUPLE_INDEX`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleIndex(pub IntegerLiteral);
