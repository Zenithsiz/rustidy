//! Arenas

// Imports
use {
	crate::{ParserStr, ast::whitespace::Whitespace},
	core::{cell::RefCell, fmt, hash::Hash, marker::PhantomData},
	std::hash::Hasher,
};

/// Arenas
#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Arenas {
	parser_str: Arena<ParserStr>,
	whitespace: Arena<Whitespace>,
}

impl Arenas {
	/// Creates all arenas as empty
	#[must_use]
	pub const fn new() -> Self {
		Self {
			parser_str: Arena::new(),
			whitespace: Arena::new(),
		}
	}

	/// Returns the arena for `T`
	#[must_use]
	pub fn get<T: ?Sized + WithArena>(&self) -> &Arena<T> {
		T::get_arena(self)
	}

	/// Creates a checkpoint on all arenas
	pub fn checkpoint(&self) -> ArenasCheckpoint {
		ArenasCheckpoint {
			parser_str: self.get::<ParserStr>().checkpoint(),
			whitespace: self.get::<Whitespace>().checkpoint(),
		}
	}

	/// Undoes a checkpoint on all arenas
	pub fn undo_checkpoint(&self, checkpoint: ArenasCheckpoint) {
		let ArenasCheckpoint { parser_str, whitespace } = checkpoint;
		self.get::<ParserStr>().undo_checkpoint(parser_str);
		self.get::<Whitespace>().undo_checkpoint(whitespace);
	}

	/// Stashes a checkpoint on all arenas
	pub fn stash_checkpoint(&self, checkpoint: ArenasCheckpoint) -> ArenasCheckpointStash {
		ArenasCheckpointStash {
			parser_str: self.get::<ParserStr>().stash_checkpoint(checkpoint.parser_str),
			whitespace: self.get::<Whitespace>().stash_checkpoint(checkpoint.whitespace),
		}
	}

	/// Applies a checkpoint stash on all arenas
	pub fn apply_checkpoint_stash(&self, stash: ArenasCheckpointStash) {
		let ArenasCheckpointStash { parser_str, whitespace } = stash;
		self.get::<ParserStr>().apply_checkpoint_stash(parser_str);
		self.get::<Whitespace>().apply_checkpoint_stash(whitespace);
	}
}

impl Default for Arenas {
	fn default() -> Self {
		Self::new()
	}
}

/// Arenas checkpoint
#[derive(Clone, Copy, Debug)]
pub struct ArenasCheckpoint {
	parser_str: ArenaCheckpoint,
	whitespace: ArenaCheckpoint,
}

/// Arenas checkpoint stash
#[derive(Clone, Debug)]
pub struct ArenasCheckpointStash {
	parser_str: ArenaCheckpointStash<ParserStr>,
	whitespace: ArenaCheckpointStash<Whitespace>,
}

/// Arena for `T`'s Data
#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound(serialize = "T::Data: serde::Serialize"))]
#[serde(bound(deserialize = "T::Data: serde::Deserialize<'de>"))]
pub struct Arena<T: ?Sized + ArenaData> {
	// TODO: Track the borrows separately to not wrap each
	//       data within an `Option`?
	data: RefCell<Vec<Option<T::Data>>>,
}

impl<T: ?Sized + ArenaData> Arena<T> {
	/// Creates a new, empty, arena
	#[must_use]
	const fn new() -> Self {
		Self {
			data: RefCell::new(vec![]),
		}
	}

	/// Pushes a value onto the arena, returning it's index
	pub fn push(&self, value: T::Data) -> ArenaIdx<T> {
		let mut data = self.data.borrow_mut();
		let idx = data.len();
		data.push(Some(value));
		ArenaIdx {
			inner:   idx.try_into().expect("Too many indices"),
			phantom: PhantomData,
		}
	}

	/// Uses the value at an index
	pub fn with_value<R>(&self, idx: ArenaIdx<T>, f: impl FnOnce(&mut T::Data) -> R) -> R {
		let mut value = self
			.data
			.borrow_mut()
			.get_mut(idx.inner as usize)
			.expect("Invalid arena index")
			.take()
			.expect("Attempted to borrow arena value twice");

		let output = f(&mut value);

		*self
			.data
			.borrow_mut()
			.get_mut(idx.inner as usize)
			.expect("Arena was truncated during borrow") = Some(value);

		output
	}

	/// Returns the number of values in this arena
	#[must_use]
	pub fn len(&self) -> usize {
		self.data.borrow().len()
	}

	/// Returns if the arena is empty
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.data.borrow().is_empty()
	}

	/// Creates a checkpoint
	pub fn checkpoint(&self) -> ArenaCheckpoint {
		ArenaCheckpoint { len: self.len() }
	}

	/// Undoes a checkpoint
	pub fn undo_checkpoint(&self, checkpoint: ArenaCheckpoint) {
		let mut data = self.data.borrow_mut();
		for (idx, data) in data.iter().enumerate().skip(checkpoint.len) {
			if data.is_none() {
				Self::panic_on_borrowed_checkpoint_stash(idx);
			}
		}

		data.truncate(checkpoint.len);
	}

	/// Stashes a checkpoint
	pub fn stash_checkpoint(&self, checkpoint: ArenaCheckpoint) -> ArenaCheckpointStash<T> {
		ArenaCheckpointStash {
			values: self
				.data
				.borrow_mut()
				.drain(checkpoint.len..)
				.enumerate()
				.map(|(idx, data)| match data {
					Some(data) => data,
					None => Self::panic_on_borrowed_checkpoint_stash(checkpoint.len + idx),
				})
				.collect(),
		}
	}

	/// Applies a checkpoint stash
	pub fn apply_checkpoint_stash(&self, stash: ArenaCheckpointStash<T>) {
		self.data.borrow_mut().extend(stash.values.into_iter().map(Some));
	}

	#[cold]
	fn panic_on_borrowed_checkpoint_stash(idx: usize) -> ! {
		panic!(
			"Attempted to stash checkpoint with a borrowed value at index {idx} in arena {:?}",
			std::any::type_name::<T>(),
		);
	}
}

impl<T: ArenaData> Default for Arena<T> {
	fn default() -> Self {
		Self::new()
	}
}

/// Arena checkpoint
#[derive(Clone, Copy, Debug)]
pub struct ArenaCheckpoint {
	len: usize,
}

/// Arena checkpoint stash
#[derive(Clone, Debug)]
pub struct ArenaCheckpointStash<T: ?Sized + ArenaData> {
	// TODO: Ideally here we wouldn't allocate and maybe just move
	//       the new ranges somewhere else temporarily.
	values: Vec<T::Data>,
}

/// Arena index for `T`
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ArenaIdx<T: ?Sized> {
	inner:   u32,
	phantom: PhantomData<T>,
}

impl<T: ?Sized> PartialEq for ArenaIdx<T> {
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner
	}
}

impl<T: ?Sized> Eq for ArenaIdx<T> {}

impl<T: ?Sized> Clone for ArenaIdx<T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T: ?Sized> Copy for ArenaIdx<T> {}

impl<T: ?Sized> Hash for ArenaIdx<T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.inner.hash(state);
	}
}

impl<T: ?Sized> fmt::Debug for ArenaIdx<T> {
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
}

macro impl_with_arena( $($Ty:ty => $field:ident),* $(,)? ) {
	$(
		impl WithArena for $Ty {
			fn get_arena(arenas: &Arenas) -> &Arena<Self> {
				&arenas.$field
			}
		}
	)*
}

impl_with_arena! {
	ParserStr => parser_str,
	Whitespace => whitespace,
}
