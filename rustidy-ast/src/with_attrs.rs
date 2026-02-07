//! Type with attributes

// Imports
use {
	super::attr::{InnerAttrOrDocComment, OuterAttrOrDocComment},
	crate::{
		attr::{Attr, AttrInput, DelimTokenTree, TokenNonDelimited, TokenTree},
		token::{Punctuation, Token},
	},
	app_error::{AppError, bail},
	rustidy_format::{Format, FormatFn},
	rustidy_parse::{ParsableRecursive, Parse, Parser},
	rustidy_print::Print,
};

/// A type with outer attributes
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(with = Self::format)]
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

impl<T: Format> WithOuterAttributes<T> {
	fn format(&mut self, ctx: &mut rustidy_format::Context) {
		let mut ctx = ctx.sub_context();

		for attr in &mut self.attrs {
			attr.prefix_ws_set_cur_indent(&mut ctx);
			attr.format(&mut ctx);

			if let Some(attr) = attr.try_as_attr_ref() &&
				let Err(err) = self::update_config(&attr.open.value, &mut ctx)
			{
				tracing::warn!("Malformed `#[rustidy::config(...)]` attribute: {err:?}");
			}
		}

		self.inner.format(&mut ctx);
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

/// A type with inner attributes
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct WithInnerAttributes<T> {
	#[format(and_with = rustidy_format::format_vec_each_with(Format::prefix_ws_set_cur_indent))]
	pub attrs: Vec<InnerAttrOrDocComment>,
	pub inner: T,
}

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
fn update_config(attr: &Attr, ctx: &mut rustidy_format::Context) -> Result<(), AppError> {
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
			Ident,
		}

		let field = match ident.1.str(ctx.input()).as_str() {
			"ident" => ConfigField::Ident,
			ident => bail!("Unknown configuration: {ident:?}"),
		};

		let Some(TokenTree::Token(TokenNonDelimited(Token::Punctuation(Punctuation::Eq(_))))) = rest.next() else {
			bail!("Expected `=`");
		};

		match field {
			ConfigField::Ident => {
				let Some(TokenTree::Token(TokenNonDelimited(Token::StringLiteral(literal)))) = rest.next() else {
					bail!("Expected integer literal");
				};
				ctx.config_mut().indent = literal.contents(ctx.input()).into();
			},
		}
	}

	Ok(())
}
