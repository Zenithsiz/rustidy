//! Type with attributes

// Imports
use {
	super::{InnerAttrOrDocComment, OuterAttrOrDocComment},
	crate::{
		attr::{InnerDocComment, OuterDocComment},
		util::Braced,
	},
	rustidy_format::{Format, FormatTag, Formattable, WhitespaceFormat, WsFmtFn},
	rustidy_parse::{ParsableRecursive, Parse, Parser},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// A type with outer attributes
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(with = Self::format)]
#[format(where_format = "where T: Format<()>")]
pub struct WithOuterAttributes<T> {
	pub attrs: Vec<OuterAttrOrDocComment>,
	pub inner: T,
}

impl<T> WithOuterAttributes<T> {
	/// Creates a new value without any attributes
	pub const fn without_attributes(inner: T) -> Self {
		Self { attrs: vec![], inner }
	}

	/// Maps the inner type
	pub fn map<U>(self, f: impl FnOnce(T) -> U) -> WithOuterAttributes<U> {
		WithOuterAttributes {
			attrs: self.attrs,
			inner: f(self.inner),
		}
	}
}

impl<T> WithOuterAttributes<T> {
	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: &impl WsFmtFn, args: &mut ())
	where
		T: Format<()>,
	{
		let mut is_after_newline = false;
		let mut has_prefix_ws = true;
		for attr in &mut self.attrs {
			ctx.with_tag_if(is_after_newline, FormatTag::AfterNewline, |ctx| match has_prefix_ws {
				true => attr.format(ctx, prefix_ws, &mut ()),
				false => attr.format(ctx, &Whitespace::set_cur_indent, &mut ()),
			});

			is_after_newline = matches!(attr, OuterAttrOrDocComment::DocComment(OuterDocComment::Line(_)));
			has_prefix_ws = false;
		}

		let mut value_ctx = ctx.sub_context();
		for attr in &mut self.attrs {
			if let Some(attr) = attr.try_as_attr_ref() &&
				let Err(err) = super::update_config(&attr.open.value, &mut value_ctx)
			{
				tracing::warn!("Malformed `#[rustidy::config(...)]` attribute: {err:?}");
			}
		}


		value_ctx.with_tag_if(is_after_newline, FormatTag::AfterNewline, |ctx| {
			match has_prefix_ws {
				true => self.inner.format(ctx, prefix_ws, args),
				// TODO: The user should be able to choose this
				false => self.inner.format(ctx, &Whitespace::set_cur_indent, args),
			}
		});
	}
}

impl<T> From<T> for WithOuterAttributes<T> {
	fn from(inner: T) -> Self {
		Self { attrs: vec![], inner }
	}
}

impl<T, R> ParsableRecursive<R> for WithOuterAttributes<T>
where
	T: ParsableRecursive<R>,
{
	type Base = WithOuterAttributes<T::Base>;
	type Infix = T::Infix;
	type Prefix = WithOuterAttributes<T::Prefix>;
	type Suffix = T::Suffix;

	fn from_base(base: Self::Base, parser: &mut Parser) -> Self {
		Self {
			attrs: base.attrs,
			inner: T::from_base(base.inner, parser),
		}
	}

	fn join_suffix(root: R, suffix: Self::Suffix, parser: &mut Parser) -> Self {
		Self {
			attrs: vec![],
			inner: T::join_suffix(root, suffix, parser),
		}
	}

	fn join_prefix(prefix: Self::Prefix, root: R, parser: &mut Parser) -> Self {
		Self {
			attrs: prefix.attrs,
			inner: T::join_prefix(prefix.inner, root, parser),
		}
	}

	fn join_infix(lhs: R, infix: Self::Infix, rhs: R, parser: &mut Parser) -> Self {
		Self {
			attrs: vec![],
			inner: T::join_infix(lhs, infix, rhs, parser),
		}
	}
}

/// A braced type with inner attributes.
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
// TODO: Remove once rustc realizes that `Braced<WithInnerAttributes<T>>: Format => T: Format`
#[format(with_where_format = "where T: Format<()>")]
#[format(with = Self::format)]
pub struct BracedWithInnerAttributes<T>(Braced<WithInnerAttributes<T>>);

impl<T> BracedWithInnerAttributes<T> {
	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: &impl WsFmtFn, args: &mut ())
	where
		T: Format<()>,
	{
		self.0.prefix.format(ctx, prefix_ws, &mut ());

		let mut ctx = ctx.sub_context();
		for attr in &self.0.value.attrs {
			if let Some(attr) = attr.try_as_attr_ref() &&
				let Err(err) = super::update_config(&attr.attr.value, &mut ctx)
			{
				tracing::warn!("Malformed `#![rustidy::config(...)]` attribute: {err:?}");
			}
		}

		ctx.with_indent(|ctx| {
			let mut is_after_newline = false;
			for attr in &mut self.0.value.attrs {
				ctx.with_tag_if(is_after_newline, FormatTag::AfterNewline, |ctx| {
					attr.format(ctx, &Whitespace::set_cur_indent, &mut ());
				});

				is_after_newline = matches!(attr, InnerAttrOrDocComment::DocComment(InnerDocComment::Line(_)));
			}

			ctx.with_tag_if(is_after_newline, FormatTag::AfterNewline, |ctx| {
				self.0.value.inner.format(ctx, &Whitespace::set_cur_indent, args);
				let is_value_blank = self.0.value.is_blank(ctx, true);

				self.0.suffix.format(
					ctx,
					&|ws: &mut Whitespace, ctx| {
						ws.set_indent(ctx, -1, is_value_blank);
					},
					&mut (),
				);
			});
		});
	}
}

/// A type with inner attributes
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(with = |_, _, _, (): &mut ()| panic!("This type shouldn't be formatted manually"))]
#[format(where_format = "where T: Format<()>")]
struct WithInnerAttributes<T> {
	pub attrs: Vec<InnerAttrOrDocComment>,
	pub inner: T,
}
