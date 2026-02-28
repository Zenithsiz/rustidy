//! Ast literals

// Features
#![feature(
	never_type,
	coverage_attribute,
	yeet_expr,
	anonymous_lifetime_in_impl_trait,
	decl_macro,
	is_ascii_octdigit,
	trim_prefix_suffix,
	string_remove_matches
)]

// Modules
pub mod byte;
pub mod byte_string;
pub mod c_string;
pub mod char;
pub mod escape;
pub mod float;
pub mod int;
pub mod raw_byte_string;
pub mod token;
pub mod lifetime;
pub mod raw_c_string;
pub mod raw_string;
pub mod string;
pub mod suffix;
pub mod ident;

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
	ident::{Identifier, IdentifierOrKeyword, NonKeywordIdentifier, RawIdentifier},
	int::IntegerLiteral,
	lifetime::{Lifetime, LifetimeOrLabel, LifetimeToken},
	raw_byte_string::RawByteStringLiteral,
	raw_c_string::RawCStringLiteral,
	raw_string::RawStringLiteral,
	string::StringLiteral,
	suffix::{Suffix, SuffixNoE},
	token::{Punctuation, Token},
};

// Imports
use {
	format::{Format, Formattable},
	parse::Parse,
	print::Print,
};

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
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
	True(ast_tokens::True),
	False(ast_tokens::False),
}
