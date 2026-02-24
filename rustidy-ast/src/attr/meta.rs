//! `MetaItem` attributes

// Imports
use {
	crate::{expr::Expression, path::SimplePath, token, util::Parenthesized},
	rustidy_ast_util::{Longest, PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::{ParsableFrom, Parse},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `MetaItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum MetaItem {
	EqExpr(MetaItemEqExpr),
	Seq(MetaItemSeq),
	Path(MetaItemPath),
}

impl MetaItem {
	/// Returns the path of this meta item
	#[must_use]
	pub const fn path(&self) -> &SimplePath {
		match self {
			Self::EqExpr(meta) => &meta.path,
			Self::Seq(meta) => &meta.path,
			Self::Path(meta) => &meta.0,
		}
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MetaItemPath(SimplePath);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MetaItemEqExpr {
	pub path: SimplePath,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:   token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MetaItemSeq {
	pub path: SimplePath,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(
		args = delimited::fmt_remove_or_indent_if_non_blank(
			100,
			MetaSeqFmt { inner: Whitespace::SINGLE },
			MetaSeqFmt { inner: Whitespace::INDENT },
		)
	)]
	pub seq:  Parenthesized<Option<MetaSeq>>
}

/// `MetaSeq`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "MetaSeqFmt"))]
pub struct MetaSeq(
	#[format(args = punct::fmt(args.inner, Whitespace::REMOVE))]
	pub PunctuatedTrailing<MetaItemInner, token::Comma>,
);

#[derive(Clone, Copy, Debug)]
struct MetaSeqFmt {
	inner: WhitespaceConfig,
}

/// `MetaItemInner`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(from = Longest::<Expression, MetaItem>)]
pub enum MetaItemInner {
	Meta(Box<MetaItem>),
	Expr(Expression),
}

impl ParsableFrom<Longest<Expression, MetaItem>> for MetaItemInner {
	fn from_parsable(value: Longest<Expression, MetaItem>) -> Self {
		match value {
			Longest::Left(expr) => Self::Expr(expr),
			Longest::Right(meta) => Self::Meta(Box::new(meta)),
		}
	}
}
