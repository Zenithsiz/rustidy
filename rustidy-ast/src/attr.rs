//! Attributes

// Module
mod with;

// Exports
pub use self::with::{BracedWithInnerAttributes, WithOuterAttributes};

// Imports
use {
	super::{
		expr::Expression,
		path::SimplePath,
		token,
		util::{Braced, Bracketed, Parenthesized},
	},
	crate::token::{Punctuation, Token},
	app_error::{AppError, Context, bail},
	core::fmt::Debug,
	rustidy_ast_util::{RemainingBlockComment, RemainingLine, delimited},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::{Parse, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum InnerAttrOrDocComment {
	Attr(InnerAttribute),
	DocComment(InnerDocComment),
}

/// `InnerAttribute`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an inner attribute")]
pub struct InnerAttribute {
	pub pound: token::Pound,
	#[format(prefix_ws = Whitespace::remove)]
	pub not:   token::Not,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::remove)]
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	pub attr:  Bracketed<Attr>,
}

/// Inner Doc comment
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum InnerDocComment {
	Line(InnerLineDoc),
	Block(InnerBlockDoc),
}

/// `INNER_LINE_DOC`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InnerLineDoc {
	pub prefix:  token::InnerLineDoc,
	pub comment: RemainingLine,
}

/// `INNER_BLOCK_DOC`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InnerBlockDoc {
	pub prefix:  token::InnerBlockDoc,
	pub comment: RemainingBlockComment,
}

/// Outer attribute or doc comment
#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum OuterAttrOrDocComment {
	Attr(OuterAttribute),
	DocComment(OuterDocComment),
}

/// `OuterAttribute`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct OuterAttribute {
	pub pound: token::Pound,
	#[format(prefix_ws = Whitespace::remove)]
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	pub open:  Bracketed<Attr>,
}

/// Outer Doc comment
#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum OuterDocComment {
	Line(OuterLineDoc),
	Block(OuterBlockDoc),
}

/// `OUTER_LINE_DOC`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct OuterLineDoc {
	pub prefix:  token::OuterLineDoc,
	pub comment: RemainingLine,
}

/// `OUTER_BLOCK_DOC`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct OuterBlockDoc {
	pub prefix:  token::OuterBlockDoc,
	// TODO: This should technically need whitespace before if we find `/**/**/*/`,
	//       but the reference doesn't seem to mention this, so we allow it.
	pub comment: RemainingBlockComment,
}

/// `Attr`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Attr {
	// TODO: Unsafe attribute
	pub path:  SimplePath,
	#[format(prefix_ws = Whitespace::preserve)]
	pub input: Option<AttrInput>,
}

/// `AttrInput`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum AttrInput {
	#[format(prefix_ws = Whitespace::remove)]
	DelimTokenTree(DelimTokenTree),
	#[format(prefix_ws = Whitespace::set_single)]
	EqExpr(AttrInputEqExpr),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AttrInputEqExpr {
	pub eq:   token::Eq,
	#[format(prefix_ws = Whitespace::set_single)]
	pub expr: Expression,
}

/// `DelimTokenTree`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum DelimTokenTree {
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	Parens(Parenthesized<DelimTokenTreeInner>),
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	Brackets(Bracketed<DelimTokenTreeInner>),
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	Braces(Braced<DelimTokenTreeInner>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DelimTokenTreeInner(
	#[parse(fatal)]
	#[format(args = rustidy_format::VecArgs::from_prefix_ws(Whitespace::preserve))]
	pub Vec<TokenTree>,
);

/// `TokenTree`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TokenTree {
	Tree(DelimTokenTree),
	Token(TokenNonDelimited),
}

/// `Token` except delimiters
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TokenNonDelimited(#[parse(with_tag = ParserTag::SkipDelimiters)] pub token::Token);

/// Updates the configuration based on an attribute
// TODO: We need to return the position for better error messages.
pub fn update_config(attr: &Attr, ctx: &mut rustidy_format::Context) -> Result<(), AppError> {
	// If this isn't a `rustidy::config` macro, we have nothing to update
	if !attr.path.is_str(ctx.input(), "rustidy::config") {
		return Ok(());
	}

	let Some(AttrInput::DelimTokenTree(DelimTokenTree::Parens(input))) = &attr.input else {
		bail!("Expected `rustidy::config([...])`");
	};

	let mut rest = input.value.0.iter();
	while let Some(tt) = rest.next() {
		let TokenTree::Token(TokenNonDelimited(Token::IdentOrKeyword(ident))) = tt else {
			bail!("Expected an identifier");
		};

		enum ConfigField {
			Indent,
			ArrayExprCols,
			MaxArrayExprLen,
			MaxChainLen,
		}

		let field = match ident.1.str(ctx.input()).as_str() {
			"indent" => ConfigField::Indent,
			"array_expr_cols" => ConfigField::ArrayExprCols,
			"max_array_expr_len" => ConfigField::MaxArrayExprLen,
			"max_chain_len" => ConfigField::MaxChainLen,
			ident => bail!("Unknown configuration: {ident:?}"),
		};

		let Some(TokenTree::Token(TokenNonDelimited(Token::Punctuation(Punctuation::Eq(_))))) = rest.next() else {
			bail!("Expected `=`");
		};

		match field {
			ConfigField::Indent => {
				let Some(TokenTree::Token(TokenNonDelimited(Token::StringLiteral(literal)))) = rest.next() else {
					bail!("Expected string literal");
				};
				ctx.config_mut().indent = literal.contents(ctx.input()).into();
			},
			// TODO: Should we allow resetting it to `None` again?
			ConfigField::ArrayExprCols => {
				let Some(TokenTree::Token(TokenNonDelimited(Token::IntegerLiteral(literal)))) = rest.next() else {
					bail!("Expected integer literal");
				};
				let value = literal.value(ctx.input()).context("Unable to parse integer")?;
				ctx.config_mut().array_expr_cols = Some(value.try_into().expect("`u64` didn't fit into `usize`"));
			},
			ConfigField::MaxArrayExprLen => {
				let Some(TokenTree::Token(TokenNonDelimited(Token::IntegerLiteral(literal)))) = rest.next() else {
					bail!("Expected integer literal");
				};
				let value = literal.value(ctx.input()).context("Unable to parse integer")?;
				ctx.config_mut().max_array_expr_len = value.try_into().expect("`u64` didn't fit into `usize`");
			},
			ConfigField::MaxChainLen => {
				let Some(TokenTree::Token(TokenNonDelimited(Token::IntegerLiteral(literal)))) = rest.next() else {
					bail!("Expected integer literal");
				};
				let value = literal.value(ctx.input()).context("Unable to parse integer")?;
				ctx.config_mut().max_chain_len = value.try_into().expect("`u64` didn't fit into `usize`");
			},
		}

		match rest.next() {
			Some(TokenTree::Token(TokenNonDelimited(Token::Punctuation(Punctuation::Comma(_))))) => (),
			Some(_) => bail!("Expected `,`"),
			None => break,
		}
	}

	Ok(())
}
