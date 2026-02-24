//! Type with attributes

// Imports
use {
	crate::{attr::{InnerDocComment, OuterDocComment}, util::Braced},
	super::{InnerAttrOrDocComment, OuterAttrOrDocComment},
	rustidy_ast_util::delimited,
	rustidy_format::{
		Format,
		FormatOutput,
		FormatTag,
		Formattable,
		WhitespaceConfig,
		WhitespaceFormat,
	},
	rustidy_parse::{ParsableRecursive, Parse, Parser},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// A type with outer attributes
#[derive(PartialEq, Eq, Clone, Debug)]
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
		WithOuterAttributes { attrs: self.attrs, inner: f(self.inner), }
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
			if is_after_newline {
				ctx.add_tag(FormatTag::AfterNewline);
			}

			match has_prefix_ws {
				true => ctx.format(attr, prefix_ws),
				false => ctx.format(attr, args.prefix_ws),
			}.append_to(&mut output);

			is_after_newline = matches!(attr, OuterAttrOrDocComment::DocComment(OuterDocComment::Line(_)));
			has_prefix_ws = false;
		}

		let mut value_ctx = ctx.sub_context();
		for attr in &mut self.attrs {
			if let Some(attr) = attr.try_as_attr_ref() && let Err(err) = super::update_from_attr(&attr.open.value, &mut value_ctx) {
				tracing::warn!("Malformed `#[rustidy::config(...)]` attribute: {err:?}");
			}
		}


		if is_after_newline {
			value_ctx.add_tag(FormatTag::AfterNewline);
		}

		match has_prefix_ws {
			true => value_ctx
				.format_with(&mut self.inner, prefix_ws, args.inner_args),
			false => value_ctx
				.format_with(&mut self.inner, args.prefix_ws, args.inner_args),
		}.append_to(&mut output);

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
	T: ParsableRecursive<R> {
	type Base = WithOuterAttributes<T::Base>;
	type Infix = T::Infix;
	type Prefix = T::Prefix;
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
			attrs: vec![],
			inner: T::join_prefix(prefix, root, parser),
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
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
pub struct BracedWithInnerAttributes<T>(Braced<WithInnerAttributes<T>>);

impl<T: Format<WhitespaceConfig, ()>> Format<WhitespaceConfig, ()> for BracedWithInnerAttributes<T> {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		_args: ()
	) -> FormatOutput {
		self.format(
			ctx,
			prefix_ws,
			FmtArgs { prefix_ws: Whitespace::INDENT, inner_args: () }
		)
	}
}

impl<T: Format<WhitespaceConfig, A>, A: Clone> Format<WhitespaceConfig, FmtArgs<A>> for BracedWithInnerAttributes<T> {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtArgs<A>,
	) -> FormatOutput {
		let mut ctx = ctx.sub_context();
		for attr in &self.0.value.attrs {
			if let Some(attr) = attr.try_as_attr_ref() && let Err(err) = super::update_from_attr(&attr.attr.value, &mut ctx) {
				tracing::warn!("Malformed `#![rustidy::config(...)]` attribute: {err:?}");
			}
		}

		ctx.format_with(
			&mut self.0,
			prefix_ws,
			delimited::fmt_indent_if_non_blank_with_value(args)
		)

	}
}

/// A type with inner attributes
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
struct WithInnerAttributes<T> {
	pub attrs: Vec<InnerAttrOrDocComment>,
	pub inner: T,
}

impl<T: Format<WhitespaceConfig, A>, A> Format<WhitespaceConfig, FmtArgs<A>> for WithInnerAttributes<T> {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtArgs<A>
	) -> FormatOutput {
		let mut output = FormatOutput::default();

		let mut is_after_newline = false;
		let mut has_prefix_ws = true;
		for attr in &mut self.attrs {
			if is_after_newline {
				ctx.add_tag(FormatTag::AfterNewline);
			}

			let prefix_ws = match has_prefix_ws {
				true => prefix_ws,
				false => args.prefix_ws,
			};

			ctx
				.format(attr, prefix_ws)
				.append_to(&mut output);

			is_after_newline = matches!(attr, InnerAttrOrDocComment::DocComment(InnerDocComment::Line(_)));
			has_prefix_ws = false;
		}

		// Note: `self.inner` might be empty, so we add the tag
		//       for the suffix in `BracedWithInnerAttributes`.
		if is_after_newline {
			ctx.add_tag(FormatTag::AfterNewline);
		}

		let prefix_ws = match has_prefix_ws {
			true => prefix_ws,
			false => args.prefix_ws,
		};
		ctx
			.format_with(&mut self.inner, prefix_ws, args.inner_args)
			.append_to(&mut output);

		output
	}
}

/// Formatting arguments
#[derive(Clone, Copy, Debug)]
pub struct FmtArgs<A> {
	pub prefix_ws:  WhitespaceConfig,
	pub inner_args: A,
}

#[must_use]
pub const fn fmt(prefix_ws: WhitespaceConfig) -> FmtArgs<()> {
	self::fmt_with(prefix_ws, ())
}

#[must_use]
pub const fn fmt_with<A>(prefix_ws: WhitespaceConfig, inner_args: A) -> FmtArgs<A> {
	FmtArgs { prefix_ws, inner_args }
}
