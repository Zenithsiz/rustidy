//! Type with attributes

// Imports
use {
	super::attr::{InnerAttrOrDocComment, OuterAttrOrDocComment},
	crate::{Format, Parse, Parser, Print, parser::ParsableRecursive},
	core::marker::PhantomData,
};

/// A type with outer attributes
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct WithOuterAttributes<T, RecursiveParent = !> {
	pub attrs:    Vec<OuterAttrOrDocComment>,
	pub inner:    T,
	#[serde(skip)]
	pub _phantom: PhantomData<RecursiveParent>,
}

impl<T, RecursiveParent> WithOuterAttributes<T, RecursiveParent> {
	/// Creates a new value without any attributes
	pub const fn without_attributes(inner: T) -> Self {
		Self {
			attrs: vec![],
			inner,
			_phantom: PhantomData,
		}
	}

	/// Maps the inner type
	pub fn map<U>(self, f: impl FnOnce(T) -> U) -> WithOuterAttributes<U> {
		WithOuterAttributes {
			attrs:    self.attrs,
			inner:    f(self.inner),
			_phantom: PhantomData,
		}
	}
}

impl<T, RecursiveParent> From<T> for WithOuterAttributes<T, RecursiveParent> {
	fn from(inner: T) -> Self {
		Self {
			attrs: vec![],
			inner,
			_phantom: PhantomData,
		}
	}
}

impl<T, RecursiveParent, R> ParsableRecursive<R> for WithOuterAttributes<T, RecursiveParent>
where
	T: ParsableRecursive<R>,
	RecursiveParent: ParsableRecursive<R> + From<Self>,
{
	type Base = WithOuterAttributes<T::Base>;
	type Infix = T::Infix;
	type Prefix = WithOuterAttributes<T::Prefix>;
	type Suffix = T::Suffix;

	fn into_root(self, parser: &mut Parser) -> R {
		RecursiveParent::from(self).into_root(parser)
	}

	fn from_base(base: Self::Base, parser: &mut Parser) -> Self {
		Self {
			attrs:    base.attrs,
			inner:    T::from_base(base.inner, parser),
			_phantom: PhantomData,
		}
	}

	fn join_suffix(root: R, suffix: Self::Suffix, parser: &mut Parser) -> Self {
		Self {
			attrs:    vec![],
			inner:    T::join_suffix(root, suffix, parser),
			_phantom: PhantomData,
		}
	}

	fn join_prefix(prefix: Self::Prefix, root: R, parser: &mut Parser) -> Self {
		Self {
			attrs:    prefix.attrs,
			inner:    T::join_prefix(prefix.inner, root, parser),
			_phantom: PhantomData,
		}
	}

	fn join_infix(lhs: R, infix: Self::Infix, rhs: R, parser: &mut Parser) -> Self {
		Self {
			attrs:    vec![],
			inner:    T::join_infix(lhs, infix, rhs, parser),
			_phantom: PhantomData,
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
