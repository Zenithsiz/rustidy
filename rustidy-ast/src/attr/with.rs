//! Type with attributes

// Imports
use {
	super::{InnerAttrOrDocComment, OuterAttrOrDocComment},
	crate::{
		attr::{InnerDocComment, OuterDocComment},
		util::Braced,
	},
	rustidy_format::{Format, FormatOutput, FormatTag, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::{ParsableRecursive, Parse, Parser},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// A type with outer attributes
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
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

impl<T: Format<WhitespaceConfig, ()>> Format<WhitespaceConfig, ()> for WithOuterAttributes<T> {
	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: WhitespaceConfig, _args: ()) -> FormatOutput {
		self.format(ctx, prefix_ws, FmtArgs { inner_args: () })
	}
}

impl<A, T: Format<WhitespaceConfig, A>> Format<WhitespaceConfig, FmtArgs<A>> for WithOuterAttributes<T> {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtArgs<A>,
	) -> FormatOutput {
		let mut output = FormatOutput::default();

		let mut is_after_newline = false;
		let mut has_prefix_ws = true;
		for attr in &mut self.attrs {
			ctx.with_tag_if(is_after_newline, FormatTag::AfterNewline, |ctx| match has_prefix_ws {
				true => attr.format(ctx, prefix_ws, ()),
				false => attr.format(ctx, Whitespace::CUR_INDENT, ()),
			})
			.append_to(&mut output);

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
				true => self.inner.format(ctx, prefix_ws, args.inner_args),
				// TODO: The user should be able to choose this
				false => self.inner.format(ctx, Whitespace::CUR_INDENT, args.inner_args),
			}
			.append_to(&mut output);
		});

		output
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
#[derive(Parse, Formattable, Print)]
pub struct BracedWithInnerAttributes<T>(Braced<WithInnerAttributes<T>>);

impl<T: Format<WhitespaceConfig, ()>> Format<WhitespaceConfig, ()> for BracedWithInnerAttributes<T> {
	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: WhitespaceConfig, _args: ()) -> FormatOutput {
		self.format(ctx, prefix_ws, FmtArgs { inner_args: () })
	}
}

impl<A, T: Format<WhitespaceConfig, A>> Format<WhitespaceConfig, FmtArgs<A>> for BracedWithInnerAttributes<T> {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtArgs<A>,
	) -> FormatOutput {
		let mut output = FormatOutput::default();

		self.0.prefix.format(ctx, prefix_ws, ()).append_to(&mut output);

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
					attr.format(ctx, Whitespace::CUR_INDENT, ()).append_to(&mut output);
				});

				is_after_newline = matches!(attr, InnerAttrOrDocComment::DocComment(InnerDocComment::Line(_)));
			}

			ctx.with_tag_if(is_after_newline, FormatTag::AfterNewline, |ctx| {
				let value_output = self.0.value.inner.format(ctx, Whitespace::CUR_INDENT, args.inner_args);
				value_output.append_to(&mut output);

				let remove_if_pure = self.0.value.attrs.is_empty() && value_output.is_empty;
				let prefix_ws = Whitespace::indent(-1, remove_if_pure);
				self.0.suffix.format(ctx, prefix_ws, ()).append_to(&mut output);
			});
		});

		output
	}
}

/// A type with inner attributes
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
struct WithInnerAttributes<T> {
	pub attrs: Vec<InnerAttrOrDocComment>,
	pub inner: T,
}

/// Formatting arguments
#[derive(Clone, Copy, Debug)]
pub struct FmtArgs<A> {
	pub inner_args: A,
}

#[must_use]
pub const fn fmt<A>(inner_args: A) -> FmtArgs<A> {
	FmtArgs { inner_args }
}
