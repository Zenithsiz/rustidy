//! Arenas

// Imports
use {
	crate::ParserStr,
	core::{fmt, hash::Hash, marker::PhantomData},
	std::hash::Hasher,
};

/// Arenas
#[derive(Debug)]
pub struct Arenas {
	parser_str: Arena<ParserStr>,
}

impl Arenas {
	/// Creates all arenas as empty
	#[must_use]
	pub const fn new() -> Self {
		Self {
			parser_str: Arena::new(),
		}
	}

	/// Returns the arena for `T`
	#[must_use]
	pub fn get<T: WithArena>(&self) -> &Arena<T> {
		T::get_arena(self)
	}

	/// Returns the arena for `T` mutably
	#[must_use]
	pub fn get_mut<T: WithArena>(&mut self) -> &mut Arena<T> {
		T::get_arena_mut(self)
	}
}

impl Default for Arenas {
	fn default() -> Self {
		Self::new()
	}
}

/// Arena for `T`'s Data
#[derive(Debug)]
pub struct Arena<T: ?Sized + ArenaData> {
	data: Vec<T::Data>,
}

impl<T: ArenaData> Arena<T> {
	/// Creates a new, empty, arena
	#[must_use]
	const fn new() -> Self {
		Self { data: vec![] }
	}

	/// Pushes a value onto the arena, returning it's index
	pub fn push(&mut self, value: T::Data) -> ArenaIdx<T> {
		let idx = self.data.len();
		self.data.push(value);
		ArenaIdx {
			inner:   idx.try_into().expect("Too many indices"),
			phantom: PhantomData,
		}
	}

	/// Returns the value at an index
	#[must_use]
	pub fn get(&self, idx: ArenaIdx<T>) -> &T::Data {
		&self.data[idx.inner as usize]
	}

	/// Returns the value at an index mutably
	#[must_use]
	pub fn get_mut(&mut self, idx: ArenaIdx<T>) -> &mut T::Data {
		&mut self.data[idx.inner as usize]
	}

	/// Returns the number of values in this arena
	#[must_use]
	pub const fn len(&self) -> usize {
		self.data.len()
	}

	/// Returns if the arena is empty
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	/// Truncates the arena to hold `len` elements, returning the rest
	pub fn truncate(&mut self, len: usize) -> impl Iterator<Item = T::Data> {
		self.data.drain(len..)
	}

	/// Extends the arena with elements
	pub fn extend(&mut self, values: impl IntoIterator<Item = T::Data>) {
		self.data.extend(values);
	}
}

impl<T: ArenaData> Default for Arena<T> {
	fn default() -> Self {
		Self::new()
	}
}

/// Arena index for `T`
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ArenaIdx<T> {
	inner:   u32,
	phantom: PhantomData<T>,
}

impl<T> PartialEq for ArenaIdx<T> {
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner
	}
}

impl<T> Eq for ArenaIdx<T> {}

impl<T> Clone for ArenaIdx<T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Copy for ArenaIdx<T> {}

impl<T> Hash for ArenaIdx<T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.inner.hash(state);
	}
}

impl<T> fmt::Debug for ArenaIdx<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("ArenaIdx").field(&self.inner).finish()
	}
}

/// Arena data
pub trait ArenaData {
	type Data;
}

// TODO: This should be sealed
pub trait WithArena: ArenaData {
	fn get_arena(arenas: &Arenas) -> &Arena<Self>;
	fn get_arena_mut(arenas: &mut Arenas) -> &mut Arena<Self>;
}

macro impl_with_arena( $($Ty:ty => $field:ident),* $(,)? ) {
	$(
		impl WithArena for $Ty {
			fn get_arena(arenas: &Arenas) -> &Arena<Self> {
				&arenas.$field
			}

			fn get_arena_mut(arenas: &mut Arenas) -> &mut Arena<Self> {
				&mut arenas.$field
			}
		}
	)*
}

impl_with_arena! {
	ParserStr => parser_str,
}
