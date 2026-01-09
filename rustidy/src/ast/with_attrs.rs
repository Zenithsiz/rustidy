//! Type with attributes

// Imports
use {
	super::attr::{InnerAttrOrDocComment, OuterAttrOrDocComment},
	crate::{Format, Parse, Parser, Print, parser::ParsableRecursive},
};

/// A type with outer attributes
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
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
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct WithInnerAttributes<T> {
	pub attrs: Vec<InnerAttrOrDocComment>,
	pub inner: T,
}
