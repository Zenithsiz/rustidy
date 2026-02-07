//! Type with attributes

// Imports
use {
	super::{InnerAttrOrDocComment, OuterAttrOrDocComment},
	crate::{
		attr::{Attr, AttrInput, DelimTokenTree, TokenNonDelimited, TokenTree},
		token::{Punctuation, Token},
	},
	app_error::{AppError, bail},
	rustidy_format::Format,
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
		for attr in &mut self.attrs {
			attr.prefix_ws_set_cur_indent(ctx);
			attr.format(ctx);
		}

		let mut value_ctx = ctx.sub_context();
		for attr in &mut self.attrs {
			if let Some(attr) = attr.try_as_attr_ref() &&
				let Err(err) = self::update_config(&attr.open.value, &mut value_ctx)
			{
				tracing::warn!("Malformed `#[rustidy::config(...)]` attribute: {err:?}");
			}
		}

		self.inner.format(&mut value_ctx);
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
#[format(with = Self::format)]
pub struct WithInnerAttributes<T> {
	pub attrs: Vec<InnerAttrOrDocComment>,
	pub inner: T,
}

impl<T: Format> WithInnerAttributes<T> {
	fn format(&mut self, ctx: &mut rustidy_format::Context) {
		// TODO: Ideally we'd also format the parent of this to avoid the very first whitespace
		//       not being affected by this context.
		let mut attr_value_ctx = ctx.sub_context();

		for attr in &mut self.attrs {
			if let Some(attr) = attr.try_as_attr_ref() &&
				let Err(err) = self::update_config(&attr.attr.value, &mut attr_value_ctx)
			{
				tracing::warn!("Malformed `#![rustidy::config(...)]` attribute: {err:?}");
			}
		}

		for attr in &mut self.attrs {
			attr.prefix_ws_set_cur_indent(&mut attr_value_ctx);
			attr.format(&mut attr_value_ctx);
		}

		self.inner.format(&mut attr_value_ctx);
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
