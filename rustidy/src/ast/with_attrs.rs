//! Type with attributes

// Imports
use {
	super::attr::OuterAttrOrDocComment,
	crate::{Format, Parse, Print, parser::ParsableRecursive},
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
	R: From<Self>,
{
	type Base = WithOuterAttributes<T::Base>;
	type Infix = T::Infix;
	type Prefix = WithOuterAttributes<T::Prefix>;
	type Suffix = T::Suffix;

	fn into_root(self) -> R {
		self.into()
	}

	fn from_base(base: Self::Base) -> Self {
		Self {
			attrs: base.attrs,
			inner: T::from_base(base.inner),
		}
	}

	fn join_suffix(root: R, suffix: Self::Suffix) -> Self {
		Self {
			attrs: vec![],
			inner: T::join_suffix(root, suffix),
		}
	}

	fn join_prefix(prefix: Self::Prefix, root: R) -> Self {
		Self {
			attrs: prefix.attrs,
			inner: T::join_prefix(prefix.inner, root),
		}
	}

	fn join_infix(lhs: R, infix: Self::Infix, rhs: R) -> Self {
		Self {
			attrs: vec![],
			inner: T::join_infix(lhs, infix, rhs),
		}
	}
}
