//! Attributes

// Module
pub mod with;
pub mod meta;

// Exports
pub use self::with::{BracedWithInnerAttributes, WithOuterAttributes};

// Imports
use {
	self::meta::MetaItem,
	super::{
		expr::Expression,
		path::SimplePath,
		token,
		util::{Braced, Bracketed, Parenthesized},
	},
	app_error::{AppError, Context, bail},
	core::fmt::Debug,
	rustidy_ast_util::{Longest, RemainingBlockComment, RemainingLine, delimited},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::{Parse, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
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
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct AttrOrMetaItem(pub Longest<Attr, MetaItem>);

impl AttrOrMetaItem {
	#[must_use]
	pub const fn try_as_meta(&self) -> Option<&MetaItem> {
		self.0.try_as_right_ref()
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
	#[format(args = delimited::fmt_preserve())]
	Parens(Parenthesized<DelimTokenTreeInner>),
	#[format(args = delimited::fmt_preserve())]
	Brackets(Bracketed<DelimTokenTreeInner>),
	#[format(args = delimited::fmt_preserve())]
	Braces(Braced<DelimTokenTreeInner>),
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
pub struct TokenNonDelimited(#[parse(with_tag = ParserTag::SkipDelimiters)]
pub token::Token);

/// Updates the configuration based on an attribute
// TODO: We need to return the position for better error messages.
pub fn update_config(attr: &AttrOrMetaItem, ctx: &mut rustidy_format::Context) -> Result<(), AppError> {
	let meta = attr
		.try_as_meta()
		.context("Attribute was not a meta item")?;

	// If this isn't a `rustidy::config` macro, we have nothing to update
	if !meta
		.path()
		.is_str(ctx.input(), "rustidy::config") {
		return Ok(());
	}

	let MetaItem::Seq(meta) = meta else {
		bail!("Expected `rustidy::config([...])`");
	};

	let Some(configs) = &meta.seq.value else {
		return Ok(())
	};

	for config in configs.0.values() {
		let config = try {
			config.try_as_item()?.try_as_eq_expr_ref()?
		};
		let Some(config) = config else {
			bail!("Expected `rustidy::config(<config-name> = <value>)`");
		};

		macro str() {
			config
				.expr
				.as_string_literal()
				.context("Expected a string literal")?
				.contents(ctx.input())
		}
		macro int() {
			config
				.expr
				.as_integer_literal()
				.context("Expected an integer literal")?
				.value(ctx.input())
				.context("Unable to parse integer")?
				.try_into()
				.expect("`u64` didn't fit into `usize`")
		}

		macro set_arc_str(
			$field:ident
		) {
			ctx.config_mut().$field = str!().into()
		}

		macro set_int(
			$field:ident
		) {
			ctx.config_mut().$field = int!()
		}

		macro set_opt_int(
			$field:ident
		) {
			ctx.config_mut().$field = Some(int!())
		}

		match config.path.as_str(ctx.input()).as_str() {
			"indent" => set_arc_str!(indent),
			// TODO: Should we allow resetting these to `None` again?
			"array_expr_cols" => set_opt_int!(array_expr_cols),
			"max_array_expr_len" => set_int!(max_array_expr_len),
			"max_chain_len" => set_int!(max_chain_len),
			ident => bail!("Unknown configuration: {ident:?}"),
		}
	};

	Ok(())
}
