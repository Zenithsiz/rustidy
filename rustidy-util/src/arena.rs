//! Arenas

// Imports
use {
	core::{fmt, hash::Hash, marker::PhantomData, mem, ops},
	std::{hash::Hasher, sync::Mutex},
};

/// Arena for `T`'s Data
#[derive(Debug)]
pub struct Arena<T> {
	// TODO: Track the borrows/dropped separately to not wrap each
	//       data within an `ArenaSlot`?
	data: Mutex<Vec<ArenaSlot<T>>>,
}

impl<T> Arena<T> {
	/// Creates a new, empty, arena
	#[must_use]
	pub const fn new() -> Self {
		Self {
			data: Mutex::new(vec![]),
		}
	}

	/// Returns the number of values in this arena
	#[must_use]
	pub fn len(&self) -> usize {
		self.data.lock().expect("Poisoned").len()
	}

	/// Returns if the arena is empty
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.data.lock().expect("Poisoned").is_empty()
	}
}

impl<T: ArenaData> Default for Arena<T> {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug)]
#[derive(strum::EnumIs)]
enum ArenaSlot<T> {
	Alive(T),
	Borrowed,
	Empty,
}

impl<T> ArenaSlot<T> {
	/// Borrows the value in this arena slot.
	fn borrow(&mut self) -> Option<T> {
		match mem::replace(self, Self::Borrowed) {
			Self::Alive(value) => Some(value),
			Self::Borrowed => None,
			Self::Empty => {
				*self = Self::Empty;
				None
			},
		}
	}

	/// Takes the value in this arena slot.
	fn take(&mut self) -> Option<T> {
		match mem::replace(self, Self::Borrowed) {
			Self::Alive(value) => Some(value),
			Self::Borrowed => {
				*self = Self::Borrowed;
				None
			},
			Self::Empty => None,
		}
	}
}

/// Arena reference
pub struct ArenaRef<'a, T: ArenaData> {
	value:   Option<T>,
	idx:     usize,
	phantom: PhantomData<&'a T>,
}

impl<T: ArenaData> ops::Deref for ArenaRef<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.value.as_ref().expect("Value should exist")
	}
}

impl<T: ArenaData + fmt::Debug> fmt::Debug for ArenaRef<'_, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T: ArenaData> Drop for ArenaRef<'_, T> {
	fn drop(&mut self) {
		let mut data = T::ARENA.data.lock().expect("Poisoned");
		let value = data.get_mut(self.idx).expect("Arena was truncated during borrow");
		assert!(value.is_borrowed(), "Borrowed value wasn't borrowed");
		*value = ArenaSlot::Alive(self.value.take().expect("Value should exist"));
		drop(data);
	}
}

/// Arena mutable reference
pub struct ArenaRefMut<'a, T: ArenaData> {
	value:   Option<T>,
	idx:     usize,
	phantom: PhantomData<&'a mut T>,
}

impl<T: ArenaData> ops::Deref for ArenaRefMut<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.value.as_ref().expect("Value should exist")
	}
}

impl<T: ArenaData> ops::DerefMut for ArenaRefMut<'_, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.value.as_mut().expect("Value should exist")
	}
}

impl<T: ArenaData + fmt::Debug> fmt::Debug for ArenaRefMut<'_, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T: ArenaData> Drop for ArenaRefMut<'_, T> {
	fn drop(&mut self) {
		let mut data = T::ARENA.data.lock().expect("Poisoned");
		let value = data.get_mut(self.idx).expect("Arena was truncated during borrow");
		assert!(value.is_borrowed(), "Borrowed value wasn't borrowed");
		*value = ArenaSlot::Alive(self.value.take().expect("Value should exist"));
		drop(data);
	}
}

/// Arena index for `T`
pub struct ArenaIdx<T: ArenaData> {
	inner:   u32,
	phantom: PhantomData<T>,
}

impl<T: ArenaData> ArenaIdx<T> {
	/// Creates a new value in the arena
	pub fn new(value: T) -> Self {
		let mut data = T::ARENA.data.lock().expect("Poisoned");

		// Pop all dead slots at the end
		while data.pop_if(|slot| slot.is_empty()).is_some() {}

		// Then push the new value
		let idx = data.len();
		data.push(ArenaSlot::Alive(value));
		drop(data);

		Self {
			inner:   idx.try_into().expect("Too many indices"),
			phantom: PhantomData,
		}
	}

	/// Borrows this value
	pub fn get(&self) -> ArenaRef<'_, T> {
		let idx = self.inner as usize;
		let value = T::ARENA
			.data
			.lock()
			.expect("Poisoned")
			.get_mut(idx)
			.expect("Invalid arena index")
			.borrow()
			.expect("Attempted to borrow arena value twice");

		ArenaRef {
			value: Some(value),
			idx,
			phantom: PhantomData,
		}
	}

	/// Borrows this value mutably
	pub fn get_mut(&mut self) -> ArenaRefMut<'_, T> {
		let idx = self.inner as usize;
		let value = T::ARENA
			.data
			.lock()
			.expect("Poisoned")
			.get_mut(idx)
			.expect("Invalid arena index")
			.borrow()
			.expect("Attempted to borrow arena value twice");

		ArenaRefMut {
			value: Some(value),
			idx,
			phantom: PhantomData,
		}
	}

	/// Moves out of this value
	pub fn take(self) -> T {
		let inner_idx = self.inner as usize;
		mem::forget(self);
		T::ARENA
			.data
			.lock()
			.expect("Poisoned")
			.get_mut(inner_idx)
			.expect("Invalid arena index")
			.take()
			.expect("Attempted to borrow arena value twice")
	}

	/// Moves out of this value if `f` returns `Ok`.
	pub fn try_take_map<F, R>(self, f: F) -> Result<R, Self>
	where
		F: FnOnce(T) -> Result<R, T>,
	{
		let inner_idx = self.inner as usize;
		let value = T::ARENA
			.data
			.lock()
			.expect("Poisoned")
			.get_mut(inner_idx)
			.expect("Invalid arena index")
			.borrow()
			.expect("Attempted to borrow arena value twice");

		let (output, slot) = match f(value) {
			Ok(output) => {
				mem::forget(self);
				(Ok(output), ArenaSlot::Empty)
			},
			Err(value) => (Err(self), ArenaSlot::Alive(value)),
		};

		*T::ARENA
			.data
			.lock()
			.expect("Poisoned")
			.get_mut(inner_idx)
			.expect("Invalid arena index") = slot;

		output
	}

	/// Returns a unique id for this arena index
	#[must_use]
	pub const fn id(&self) -> u32 {
		self.inner
	}
}

impl<T: ArenaData> Drop for ArenaIdx<T> {
	fn drop(&mut self) {
		let inner_idx = self.inner as usize;

		let mut data = T::ARENA.data.lock().expect("Poisoned");
		let value = mem::replace(&mut data[inner_idx], ArenaSlot::Empty);
		drop(data);

		assert!(value.is_alive(), "Attempted to drop a non-alive pack");
	}
}

impl<T: ArenaData> PartialEq for ArenaIdx<T> {
	fn eq(&self, other: &Self) -> bool {
		// TODO: Should we do deep equality if this returns false?
		self.inner == other.inner
	}
}

impl<T: ArenaData> Eq for ArenaIdx<T> {}

impl<T: ArenaData> Hash for ArenaIdx<T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.inner.hash(state);
	}
}

impl<T: ArenaData> fmt::Debug for ArenaIdx<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("ArenaIdx").field(&self.inner).finish()
	}
}


impl<T: ArenaData + serde::Serialize> serde::Serialize for ArenaIdx<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.get().serialize(serializer)
	}
}

impl<'de, T: ArenaData + serde::Deserialize<'de>> serde::Deserialize<'de> for ArenaIdx<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let value = T::deserialize(deserializer)?;
		Ok(Self::new(value))
	}
}
/// Arena data
pub trait ArenaData: Sized + 'static {
	const ARENA: &'static Arena<Self>;
}
