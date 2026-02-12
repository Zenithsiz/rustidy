//! Owned or Borrowed

// Imports
use core::{
	borrow::{Borrow, BorrowMut},
	ops::{Deref, DerefMut},
};

/// Owned or Borrowed
pub enum Oob<'a, T: ?Sized + ToOwned> {
	Borrowed(&'a mut T),
	Owned(T::Owned),
}

impl<T: ?Sized + ToOwned> Deref for Oob<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Borrowed(value) => value,
			Self::Owned(value) => value.borrow(),
		}
	}
}

impl<T: ?Sized + ToOwned<Owned: BorrowMut<T>>> DerefMut for Oob<'_, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Borrowed(value) => value,
			Self::Owned(value) => value.borrow_mut(),
		}
	}
}
