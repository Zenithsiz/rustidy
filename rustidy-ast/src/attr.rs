//! Attributes

// Module
mod with;

// Exports
pub use self::with::{BracedWithInnerAttributes, WithInnerAttributes, WithOuterAttributes};

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
	rustidy_ast_util::{RemainingBlockComment, RemainingLine},
	rustidy_format::{Format, FormatFn},
	rustidy_parse::Parse,
	rustidy_print::Print,
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
	#[format(before_with = Format::prefix_ws_remove)]
	pub not:   token::Not,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_remove)]
	#[format(and_with = Bracketed::format_remove)]
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
	#[format(before_with = Format::prefix_ws_remove)]
	#[format(and_with = Bracketed::format_remove)]
	pub open:  Bracketed<Attr>,
}

/// Outer Doc comment
#[derive(PartialEq, Eq, Debug)]
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
	pub input: Option<AttrInput>,
}

/// `AttrInput`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum AttrInput {
	#[format(before_with = Format::prefix_ws_remove)]
	DelimTokenTree(DelimTokenTree),
	#[format(before_with = Format::prefix_ws_set_single)]
	EqExpr(AttrInputEqExpr),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AttrInputEqExpr {
	pub eq:   token::Eq,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub expr: Expression,
}

/// `DelimTokenTree`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum DelimTokenTree {
	Parens(Parenthesized<DelimTokenTreeInner>),
	Brackets(Bracketed<DelimTokenTreeInner>),
	Braces(Braced<DelimTokenTreeInner>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DelimTokenTreeInner(#[parse(fatal)] pub Vec<TokenTree>);

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
pub struct TokenNonDelimited(#[parse(with_tag = "skip:Delimiters")] pub token::Token);

/// Formats the value of a `WithOuterAttributes<T, _>` if at least 1 attribute exists
pub fn format_outer_value_non_empty<T>(f: impl FormatFn<T>) -> impl FormatFn<WithOuterAttributes<T>> {
	move |with_attrs, ctx| {
		if !with_attrs.attrs.is_empty() {
			f(&mut with_attrs.inner, ctx);
		}
	}
}

/// Formats the value of a `WithInnerAttributes<T>` if at least 1 attribute exists
pub fn format_inner_value_non_empty<T>(f: impl FormatFn<T>) -> impl FormatFn<WithInnerAttributes<T>> {
	move |with_attrs, ctx| {
		if !with_attrs.attrs.is_empty() {
			f(&mut with_attrs.inner, ctx);
		}
	}
}

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
			ArrayExprRows,
		}

		let field = match ident.1.str(ctx.input()).as_str() {
			"indent" => ConfigField::Indent,
			"array_expr_rows" => ConfigField::ArrayExprRows,
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
			ConfigField::ArrayExprRows => {
				let Some(TokenTree::Token(TokenNonDelimited(Token::IntegerLiteral(literal)))) = rest.next() else {
					bail!("Expected integer literal");
				};
				let value = literal.value(ctx.input()).context("Unable to parse integer")?;
				ctx.config_mut().array_expr_rows = value.try_into().expect("`u64` didn't fit into `usize`");
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
