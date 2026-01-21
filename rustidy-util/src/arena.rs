//! Arenas

// Imports
use {
	core::{fmt, hash::Hash, marker::PhantomData, mem, ops},
	std::{hash::Hasher, sync::Mutex},
};

/// Arena for `T`'s Data
#[derive(Debug)]
pub struct Arena<T: ?Sized + ArenaData> {
	// TODO: Track the borrows/dropped separately to not wrap each
	//       data within an `ArenaSlot`?
	data: Mutex<Vec<ArenaSlot<T::Data>>>,
}

impl<T: ?Sized + ArenaData> Arena<T> {
	/// Creates a new, empty, arena
	#[must_use]
	pub const fn new() -> Self {
		Self {
			data: Mutex::new(vec![]),
		}
	}

	/// Pushes a value onto the arena, returning it's index
	pub fn push(&self, value: T::Data) -> ArenaIdx<T> {
		let mut data = self.data.lock().expect("Poisoned");
		let idx = data.len();
		data.push(ArenaSlot::Alive(value));
		drop(data);

		ArenaIdx {
			inner:   idx.try_into().expect("Too many indices"),
			phantom: PhantomData,
		}
	}

	/// Borrows a value at an index
	pub fn get(&self, idx: &ArenaIdx<T>) -> ArenaRef<'_, T> {
		let idx = idx.inner as usize;
		let value = self
			.data
			.lock()
			.expect("Poisoned")
			.get_mut(idx)
			.expect("Invalid arena index")
			.borrow()
			.expect("Attempted to borrow arena value twice");

		ArenaRef {
			value: Some(value),
			arena: self,
			idx,
		}
	}

	/// Takes a value in the arena if `f` returns `Ok`.
	pub fn try_take_map<F, R>(&self, idx: ArenaIdx<T>, f: F) -> Result<R, ArenaIdx<T>>
	where
		F: FnOnce(T::Data) -> Result<R, T::Data>,
	{
		let inner_idx = idx.inner as usize;
		let value = self
			.data
			.lock()
			.expect("Poisoned")
			.get_mut(inner_idx)
			.expect("Invalid arena index")
			.borrow()
			.expect("Attempted to borrow arena value twice");

		let (output, slot) = match f(value) {
			Ok(output) => {
				mem::forget(idx);
				(Ok(output), ArenaSlot::Empty)
			},
			Err(value) => (Err(idx), ArenaSlot::Alive(value)),
		};

		*self
			.data
			.lock()
			.expect("Poisoned")
			.get_mut(inner_idx)
			.expect("Invalid arena index") = slot;

		output
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

	/// Drops an arena index
	fn drop(&self, idx: u32) {
		let idx = idx as usize;

		let mut data = self.data.lock().expect("Poisoned");
		let value = mem::replace(&mut data[idx], ArenaSlot::Empty);
		assert!(value.is_alive(), "Attempted to drop a non-alive pack");

		// TODO: Track the first/last dropped value to make this cheaper?
		if data[idx + 1..].iter().all(ArenaSlot::is_empty) {
			let backwards_len = data[..idx]
				.iter()
				.rev()
				.position(|slot| !slot.is_empty())
				.unwrap_or(idx);
			let len = idx - backwards_len;
			data.truncate(len);
		}

		drop(data);
		drop(value);
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
}

/// Arena reference
pub struct ArenaRef<'a, T: ?Sized + ArenaData> {
	value: Option<T::Data>,
	arena: &'a Arena<T>,
	idx:   usize,
}

impl<T: ?Sized + ArenaData> ops::Deref for ArenaRef<'_, T> {
	type Target = T::Data;

	fn deref(&self) -> &Self::Target {
		self.value.as_ref().expect("Value should exist")
	}
}
impl<T: ?Sized + ArenaData> ops::DerefMut for ArenaRef<'_, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.value.as_mut().expect("Value should exist")
	}
}

impl<T: ?Sized + ArenaData> Drop for ArenaRef<'_, T> {
	fn drop(&mut self) {
		let mut data = self.arena.data.lock().expect("Poisoned");
		let value = data.get_mut(self.idx).expect("Arena was truncated during borrow");
		assert!(value.is_borrowed(), "Borrowed value wasn't borrowed");
		*value = ArenaSlot::Alive(self.value.take().expect("Value should exist"));
		drop(data);
	}
}

/// Arena index for `T`
pub struct ArenaIdx<T: ?Sized + ArenaData> {
	inner:   u32,
	phantom: PhantomData<T>,
}

impl<T: ?Sized + ArenaData> ArenaIdx<T> {
	/// Returns a unique id for this arena index
	#[must_use]
	pub const fn id(&self) -> u32 {
		self.inner
	}
}

impl<T: ?Sized + ArenaData> Drop for ArenaIdx<T> {
	fn drop(&mut self) {
		T::ARENA.drop(self.inner);
	}
}

impl<T: ?Sized + ArenaData> PartialEq for ArenaIdx<T> {
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner
	}
}

impl<T: ?Sized + ArenaData> Eq for ArenaIdx<T> {}

impl<T: ?Sized + ArenaData> Hash for ArenaIdx<T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.inner.hash(state);
	}
}

impl<T: ?Sized + ArenaData> fmt::Debug for ArenaIdx<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("ArenaIdx").field(&self.inner).finish()
	}
}


impl<T: ?Sized + ArenaData<Data: serde::Serialize>> serde::Serialize for ArenaIdx<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		T::ARENA.get(self).serialize(serializer)
	}
}

impl<'de, T: ?Sized + ArenaData<Data: serde::Deserialize<'de>>> serde::Deserialize<'de> for ArenaIdx<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let value = T::Data::deserialize(deserializer)?;
		Ok(T::ARENA.push(value))
	}
}
/// Arena data
pub trait ArenaData: 'static {
	type Data: 'static;
	const ARENA: &'static Arena<Self>;
}
