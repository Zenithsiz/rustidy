//! Integer literal

// Imports
use {
	super::SuffixNoE,
	crate::{Format, Parse, ParserStr, Print, ast::whitespace::Whitespace},
};


/// `INTEGER_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an integer literal")]
pub struct IntegerLiteral {
	#[format(whitespace)]
	pub ws:     Whitespace,
	pub inner:  IntegerLiteralInner,
	pub suffix: Option<SuffixNoE>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum IntegerLiteralInner {
	Decimal(DecLiteral),
	Binary(BinLiteral),
	Octal(OctLiteral),
	Hex(HexLiteral),
}

/// `DEC_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = StartDigit, fmt = "Expected 0-9"))]
pub struct DecLiteral(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl DecLiteral {
	fn parse(s: &mut &str) -> Result<(), DecLiteralError> {
		*s = s
			.strip_prefix(|ch: char| ch.is_ascii_digit())
			.ok_or(DecLiteralError::StartDigit)?;
		*s = s.trim_start_matches(|ch: char| ch.is_ascii_digit() || ch == '_');

		Ok(())
	}
}

/// `BIN_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = Start0B, fmt = "Expected `0b`"))]
#[parse(error(name = Digit, fmt = "Expected 0 or 1"))]
pub struct BinLiteral(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl BinLiteral {
	fn parse(s: &mut &str) -> Result<(), BinLiteralError> {
		*s = s.strip_prefix("0b").ok_or(BinLiteralError::Start0B)?;
		*s = s.trim_start_matches('_');
		*s = s
			.strip_prefix(|ch| matches!(ch, '0' | '1'))
			.ok_or(BinLiteralError::Digit)?;
		*s = s.trim_start_matches(['0', '1', '_']);

		Ok(())
	}
}

/// `OCT_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = Start0O, fmt = "Expected `0o`"))]
#[parse(error(name = Digit, fmt = "Expected 0-7"))]
pub struct OctLiteral(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl OctLiteral {
	fn parse(s: &mut &str) -> Result<(), OctLiteralError> {
		*s = s.strip_prefix("0o").ok_or(OctLiteralError::Start0O)?;
		*s = s.trim_start_matches('_');
		*s = s
			.strip_prefix(|ch: char| ch.is_ascii_octdigit())
			.ok_or(OctLiteralError::Digit)?;
		*s = s.trim_start_matches(|ch: char| ch.is_ascii_octdigit() || matches!(ch, '_'));

		Ok(())
	}
}

/// `HEX_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = Start0X, fmt = "Expected `0x`"))]
#[parse(error(name = Digit, fmt = "Expected 0-9 or a-f"))]
pub struct HexLiteral(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl HexLiteral {
	fn parse(s: &mut &str) -> Result<(), HexLiteralError> {
		*s = s.strip_prefix("0x").ok_or(HexLiteralError::Start0X)?;
		*s = s.trim_start_matches('_');
		*s = s
			.strip_prefix(|ch: char| ch.is_ascii_hexdigit())
			.ok_or(HexLiteralError::Digit)?;
		*s = s.trim_start_matches(|ch: char| ch.is_ascii_hexdigit() || matches!(ch, '_'));

		Ok(())
	}
}
