//! Integer literal

// Imports
use {
	super::SuffixNoE,
	app_error::{AppError, Context},
	rustidy_format::{Format, Formattable},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{AstStr, Whitespace},
	std::borrow::Cow,
};

/// `INTEGER_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "an integer literal")]
pub struct IntegerLiteral {
	pub ws:     Whitespace,
	#[format(prefix_ws = ())]
	pub inner:  IntegerLiteralInner,
	#[format(prefix_ws = ())]
	pub suffix: Option<SuffixNoE>,
}

impl IntegerLiteral {
	/// Returns the value of this integer literal
	pub fn value(&self, input: &str) -> Result<u64, AppError> {
		let (radix, prefix_len, mut digits) = match &self.inner {
			IntegerLiteralInner::Decimal(dec) => (10, 0, dec.0.str(input)),
			IntegerLiteralInner::Binary(bin) => (2, 2, bin.0.str(input)),
			IntegerLiteralInner::Octal(oct) => (8, 2, oct.0.str(input)),
			IntegerLiteralInner::Hex(hex) => (16, 2, hex.0.str(input)),
		};
		if digits.contains('_') {
			Cow::to_mut(&mut digits).remove_matches('_');
		}
		let digits = &digits[prefix_len..];


		u64::from_str_radix(digits, radix)
			.with_context(|| format!("Unable to parse {digits:?} as a base {radix} number"))
	}
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(no_prefix_ws)]
pub enum IntegerLiteralInner {
	Decimal(DecLiteral),
	Binary(BinLiteral),
	Octal(OctLiteral),
	Hex(HexLiteral),
}

/// `DEC_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(error(name = StartDigit, fmt = "Expected 0-9"))]
#[format(no_prefix_ws)]
pub struct DecLiteral(#[parse(try_update_with = Self::parse)]
#[format(str)]
pub AstStr,);

impl DecLiteral {
	fn parse(s: &mut &str) -> Result<(), DecLiteralError> {
		*s = s
			.strip_prefix(|ch: char| ch.is_ascii_digit())
			.ok_or(DecLiteralError::StartDigit)?;
		*s = s
			.trim_start_matches(|ch: char| ch.is_ascii_digit() || ch == '_');

		Ok(())
	}
}

/// `BIN_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(error(name = Start0B, fmt = "Expected `0b`"))]
#[parse(error(name = Digit, fmt = "Expected 0 or 1"))]
#[format(no_prefix_ws)]
pub struct BinLiteral(#[parse(try_update_with = Self::parse)]
#[format(str)]
pub AstStr,);

impl BinLiteral {
	fn parse(s: &mut &str) -> Result<(), BinLiteralError> {
		*s = s
			.strip_prefix("0b")
			.ok_or(BinLiteralError::Start0B)?;
		*s = s.trim_start_matches('_');
		*s = s
			.strip_prefix(|ch| matches!(ch, '0' | '1'))
			.ok_or(BinLiteralError::Digit)?;
		*s = s.trim_start_matches(['0', '1', '_']);

		Ok(())
	}
}

/// `OCT_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(error(name = Start0O, fmt = "Expected `0o`"))]
#[parse(error(name = Digit, fmt = "Expected 0-7"))]
#[format(no_prefix_ws)]
pub struct OctLiteral(#[parse(try_update_with = Self::parse)]
#[format(str)]
pub AstStr,);

impl OctLiteral {
	fn parse(s: &mut &str) -> Result<(), OctLiteralError> {
		*s = s
			.strip_prefix("0o")
			.ok_or(OctLiteralError::Start0O)?;
		*s = s.trim_start_matches('_');
		*s = s
			.strip_prefix(|ch: char| ch.is_ascii_octdigit())
			.ok_or(OctLiteralError::Digit)?;
		*s = s
			.trim_start_matches(|ch: char| ch.is_ascii_octdigit() || matches!(ch, '_'));

		Ok(())
	}
}

/// `HEX_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(error(name = Start0X, fmt = "Expected `0x`"))]
#[parse(error(name = Digit, fmt = "Expected 0-9 or a-f"))]
#[format(no_prefix_ws)]
pub struct HexLiteral(#[parse(try_update_with = Self::parse)]
#[format(str)]
pub AstStr,);

impl HexLiteral {
	fn parse(s: &mut &str) -> Result<(), HexLiteralError> {
		*s = s
			.strip_prefix("0x")
			.ok_or(HexLiteralError::Start0X)?;
		*s = s.trim_start_matches('_');
		*s = s
			.strip_prefix(|ch: char| ch.is_ascii_hexdigit())
			.ok_or(HexLiteralError::Digit)?;
		*s = s
			.trim_start_matches(|ch: char| ch.is_ascii_hexdigit() || matches!(ch, '_'));

		Ok(())
	}
}
