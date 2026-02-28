//! Attributes

// Module
pub mod with;
pub mod meta;

// Exports
pub use self::with::{BracedWithInnerAttributes, WithOuterAttributes};

// Imports
use {
	super::{
		expr::Expression,
		path::SimplePath,
		util::{Braced, Bracketed, Parenthesized},
	},
	self::meta::MetaItem,
	app_error::{AppError, Context, bail},
	core::fmt::Debug,
	rustidy_ast_literal::token,
	rustidy_ast_util::{Longest, RemainingBlockComment, RemainingLine, delimited},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::{ParsableFrom, Parse, ParserTag},
	rustidy_print::Print,
	rustidy_util::{Config, Whitespace},
};

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum InnerAttrOrDocComment {
	Attr(InnerAttribute),
	DocComment(InnerDocComment),
}

/// `InnerAttribute`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "an inner attribute")]
pub struct InnerAttribute {
	pub pound: token::Pound,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub not:   token::Not,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtRemove)]
	pub attr:  Bracketed<AttrOrMetaItem>,
}

/// Inner Doc comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum InnerDocComment {
	Line(InnerLineDoc),
	Block(InnerBlockDoc),
}

/// `INNER_LINE_DOC`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct InnerLineDoc {
	pub prefix:  token::InnerLineDoc,
	#[format(prefix_ws = ())]
	pub comment: RemainingLine,
}

/// `INNER_BLOCK_DOC`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct InnerBlockDoc {
	pub prefix:  token::InnerBlockDoc,
	pub comment: RemainingBlockComment,
}

/// Outer attribute or doc comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum OuterAttrOrDocComment {
	Attr(OuterAttribute),
	DocComment(OuterDocComment),
}

/// `OuterAttribute`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct OuterAttribute {
	pub pound: token::Pound,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtRemove)]
	pub open:  Bracketed<AttrOrMetaItem>,
}

/// Outer Doc comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum OuterDocComment {
	Line(OuterLineDoc),
	Block(OuterBlockDoc),
}

/// `OUTER_LINE_DOC`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct OuterLineDoc {
	pub prefix:  token::OuterLineDoc,
	#[format(prefix_ws = ())]
	pub comment: RemainingLine,
}

/// `OUTER_BLOCK_DOC`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct OuterBlockDoc {
	pub prefix:  token::OuterBlockDoc,
	// TODO: This should technically need whitespace before if we find `/**/**/*/`,
	//       but the reference doesn't seem to mention this, so we allow it.
	pub comment: RemainingBlockComment,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(from = Longest::<Attr, MetaItem>)]
pub enum AttrOrMetaItem {
	Meta(MetaItem),
	Attr(Attr),
}

impl ParsableFrom<Longest<Attr, MetaItem>> for AttrOrMetaItem {
	fn from_parsable(value: Longest<Attr, MetaItem>) -> Self {
		match value {
			Longest::Left(attr) => Self::Attr(attr),
			Longest::Right(meta) => Self::Meta(meta),
		}
	}
}

/// `Attr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Attr {
	// TODO: Unsafe attribute
	pub path:  SimplePath,
	#[format(prefix_ws = Whitespace::PRESERVE)]
	pub input: Option<AttrInput>,
}

/// `AttrInput`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum AttrInput {
	#[format(prefix_ws = Whitespace::REMOVE)]
	DelimTokenTree(DelimTokenTree),
	#[format(prefix_ws = Whitespace::SINGLE)]
	EqExpr(AttrInputEqExpr),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct AttrInputEqExpr {
	pub eq:   token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}

/// `DelimTokenTree`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum DelimTokenTree {
	Parens(DelimTokenTreeParens),
	Brackets(DelimTokenTreeBrackets),
	Braces(DelimTokenTreeBraces),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum DelimTokenTreeParens {
	#[format(args = delimited::fmt_preserve())]
	Tokens(Parenthesized<DelimTokenTreeInner>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum DelimTokenTreeBrackets {
	#[format(args = delimited::fmt_preserve())]
	Tokens(Bracketed<DelimTokenTreeInner>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum DelimTokenTreeBraces {
	#[format(args = delimited::fmt_preserve())]
	Tokens(Braced<DelimTokenTreeInner>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct DelimTokenTreeInner(
	#[parse(fatal)]
	#[format(args = rustidy_format::vec::args_prefix_ws(Whitespace::PRESERVE))]
	pub Vec<TokenTree>,
);

/// `TokenTree`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum TokenTree {
	Tree(DelimTokenTree),
	Token(TokenNonDelimited),
}

/// `Token` except delimiters
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TokenNonDelimited(#[parse(with_tag = ParserTag::SkipDelimiters)] pub token::Token);

/// Updates the configuration based on an attribute
// TODO: We need to return the position for better error messages.
pub fn update_from_attr(attr: &AttrOrMetaItem, ctx: &mut rustidy_format::Context) -> Result<(), AppError> {
	let meta = match attr {
		AttrOrMetaItem::Meta(meta) => match meta.path().starts_with("rustidy") {
			true => meta,
			false => return Ok(()),
		},
		AttrOrMetaItem::Attr(attr) => match attr.path.starts_with("rustidy") {
			true => bail!("`#[rustidy]` attributes must be meta items"),
			false => return Ok(()),
		},
	};

	match meta.path().as_str().as_str() {
		"rustidy::config" => self::update_config(meta, ctx)?,
		"rustidy::skip" => ctx.config_mut().skip = true,
		path => bail!("Unknown `#[rustidy]` attribute: {path:?}"),
	}

	Ok(())
}

/// Parses a `#[rustidy::config]` attribute
fn update_config(meta: &MetaItem, ctx: &mut rustidy_format::Context) -> Result<(), AppError> {
	let MetaItem::Seq(meta) = meta else { bail!("Expected `rustidy::config([...])`") };

	let Some(configs) = &meta.seq.value else { return Ok(()) };

	for config in configs.0.values() {
		let config = try {
			config.try_as_meta_ref()?.try_as_eq_expr_ref()?
		};
		let Some(config) = config else {
			bail!("Expected `rustidy::config(<config-name> = <value>)`")
		};

		macro str() {
			config
				.expr
				.as_string_literal()
				.context("Expected a string literal")?
				.contents()
		}
		macro int() {
			config
				.expr
				.as_integer_literal()
				.context("Expected an integer literal")?
				.value()
				.context("Unable to parse integer")?
				.try_into()
				.expect("`u64` didn't fit into `usize`")
		}

		macro fields(
			$( $field:ident = $value:expr ),* $(,)?
		) {
			let Config {
				$( $field, )*

				// Note: Skip is controlled by `rustidy::skip`.
				skip: _,
			} = ctx.config_mut();

			match config.path.as_str().as_str() {
				$(
					stringify!($field) => *$field = $value,
				)*
				ident => bail!("Unknown configuration: {ident:?}"),
			}
		}

		// TODO: Should we allow resetting `Option` types?
		fields! {
			indent = str!().into(),
			min_empty_lines = int!(),
			max_empty_lines = int!(),
			max_use_tree_len = int!(),
			array_expr_cols = Some(int!()),
			max_array_expr_len = int!(),
			max_chain_len = int!(),
			max_inline_tuple_struct_len = int!(),
		}
	};

	Ok(())
}
