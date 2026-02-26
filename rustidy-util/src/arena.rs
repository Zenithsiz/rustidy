//! Arenas

// TODO: Ensure that being able to drop arenas is sound.

// Imports
use {
	core::{
		cell::UnsafeCell,
		fmt,
		hash::Hash,
		marker::PhantomData,
		mem::{self, MaybeUninit},
		ops,
		ptr,
	},
	std::hash::Hasher,
};

pub const CAPACITY: usize = 4096;

type Slot<T> = UnsafeCell<MaybeUninit<T>>;
type Row<T> = [Slot<T>; CAPACITY];

/// Arena for `T`'s Data
#[derive(Debug)]
pub struct Arena<T> {
	/// Tracks all the initialized fields.
	///
	/// It's length also corresponds to the current
	/// length of the arena.
	// TODO: Make this a bitvec?
	init: Vec<bool>,

	/// Rows for each value.
	rows: Vec<Box<Row<T>>>,
}

impl<T> Arena<T> {
	/// Creates a new, empty, arena
	#[must_use]
	pub const fn new() -> Self {
		Self { init: vec![], rows: vec![], }
	}

	/// Removes any trailing dead values
	fn clean_trailing(&mut self) {
		let dead_elements = self
			.init
			.iter()
			.rev()
			.position(|is_init| *is_init)
			.unwrap_or(self.init.len());
		self
			.init
			.truncate(self.init.len() - dead_elements);
	}

	/// Gets a slot by index
	fn slot(&self, idx: usize) -> *mut MaybeUninit<T> {
		debug_assert!(self.init[idx]);

		let row = &self.rows[idx / CAPACITY];
		row[idx % CAPACITY].get()
	}

	/// Takes a slot by index, uninitialized it.
	///
	/// # SAFETY
	/// You must ensure that no references to the slot exist.
	unsafe fn take_slot(&mut self, idx: usize) -> T {
		let slot = self.slot(idx);
		self.init[idx] = false;

		// SAFETY: No other references to `slot` exist
		unsafe { ptr::read(slot).assume_init() }
	}
}

impl<T> !Send for Arena<T> {}
impl<T> !Sync for Arena<T> {}

impl<T> Default for Arena<T> {
	fn default() -> Self {
		Self::new()
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
		T::with_arena(|arena| {
			// SAFETY: We create no other references to `arena` in this block
			let arena = unsafe { arena.as_mut_unchecked() };

			// Before inserting, remove any trailing dead values
			arena.clean_trailing();

			// Then find our slot.
			let idx = arena.init.len();
			let row = match arena.rows.get(idx / CAPACITY) {
				Some(row) => row,
				// Note: If the row we're looking for doesn't exist, then
				//       we must be at the end of the last row, so create
				//       a new one.
				None => {
					// SAFETY: We're initializing an array of uninitialized values
					let row = unsafe {
						Box::<[Slot<T>; CAPACITY]>::new_uninit()
							.assume_init()
					};
					arena.rows.push_mut(row)
				},
			};
			let slot = &row[idx % CAPACITY];

			// Finally push the new value and set it as initialized.
			// SAFETY: No other mutable references to `slot` exist, since
			//         the slot was empty.
			unsafe { slot.as_mut_unchecked().write(value) };
			arena.init.push(true);

			Self {
				inner: idx.try_into().expect("Too many indices"),
				phantom: PhantomData,
			}
		})
	}

	/// Moves out of this value
	#[must_use = "If you don't need the value, just drop `self`"]
	pub fn take(self) -> T {
		let idx = self.inner as usize;
		mem::forget(self);

		T::with_arena(|arena| {
			// SAFETY: We create no other references to `arena` in this block
			let arena = unsafe { arena.as_mut_unchecked() };

			// SAFETY: No other mutable references to the slot exist, since we take `self`
			unsafe { arena.take_slot(idx) }
		})
	}

	/// Moves out of this value if `f` returns `Ok`.
	pub fn try_take_map<F, R>(self, f: F) -> Result<R, Self>
	where
		F: FnOnce(T) -> Result<R, T>,
	{
		// TODO: Optimize this?
		match f(self.take()) {
			Ok(value) => Ok(value),
			Err(value) => Err(Self::new(value)),
		}
	}

	/// Returns a unique id for this arena index
	#[must_use]
	pub const fn id(&self) -> u32 {
		self.inner
	}
}

impl<T: ArenaData> !Send for ArenaIdx<T> {}
impl<T: ArenaData> !Sync for ArenaIdx<T> {}

// TODO: Implement clone via reference counting with copy-on-mutable access?
impl<T: ArenaData + Clone> Clone for ArenaIdx<T> {
	fn clone(&self) -> Self {
		Self::new((**self).clone())
	}
}
impl<T: ArenaData> ops::Deref for ArenaIdx<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		let idx = self.inner as usize;

		T::with_arena(|arena| {
			// SAFETY: We create no other references to `arena` in this block
			let arena = unsafe { arena.as_ref_unchecked() };

			let slot = arena.slot(idx);

			// SAFETY: No mutable reference to the value exists since we take `&self`
			unsafe { MaybeUninit::assume_init_ref(&*slot) }
		})
	}
}

impl<T: ArenaData> ops::DerefMut for ArenaIdx<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		let idx = self.inner as usize;

		T::with_arena(|arena| {
			// SAFETY: We create no other references to `arena` in this block
			let arena = unsafe { arena.as_ref_unchecked() };

			let slot = arena.slot(idx);

			// SAFETY: No other references to the value exist since we take `&mut self`
			unsafe { MaybeUninit::assume_init_mut(&mut *slot) }
		})
	}
}

impl<T: ArenaData> Drop for ArenaIdx<T> {
	fn drop(&mut self) {
		let idx = self.inner as usize;

		T::with_arena(|arena| {
			// SAFETY: We create no other references to `arena` in this block
			let arena = unsafe { arena.as_mut_unchecked() };

			// SAFETY: No other mutable references to the slot exist, since we take `&mut self`
			// TODO: Should we manually implement this to avoid moving the value onto the stack?
			let _ = unsafe { arena.take_slot(idx) };
		});
	}
}

impl<T: ArenaData + PartialEq> PartialEq for ArenaIdx<T> {
	fn eq(&self, other: &Self) -> bool {
		if self.inner == other.inner {
			return true;
		}

		(**self) == (**other)
	}
}

impl<T: ArenaData + Eq> Eq for ArenaIdx<T> {}

impl<T: ArenaData> Hash for ArenaIdx<T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.inner.hash(state);
	}
}

impl<T: ArenaData + fmt::Debug> fmt::Debug for ArenaIdx<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f
			.debug_struct("ArenaIdx")
			.field("idx", &self.inner)
			.field("inner", &**self).finish()
	}
}


impl<T: ArenaData + serde::Serialize> serde::Serialize for ArenaIdx<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		(**self).serialize(serializer)
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
	fn with_arena<O>(f: impl FnOnce(&UnsafeCell<Arena<Self>>) -> O) -> O;
}

/// Implements `ArenaData` for `$Ty`
pub macro decl_arena(
	$Ty:ty
) {
	impl ArenaData for $Ty {
		fn with_arena<O>(f: impl FnOnce(&UnsafeCell<Arena<Self>>) -> O) -> O {
			f(&ARENA)
		}
	}

	#[thread_local]
	static ARENA: UnsafeCell<Arena<$Ty>> = UnsafeCell::new(Arena::new());
}

/// Derive macro for `ArenaData`
// TODO: Can we accept just a `$I:item` and get the type from there?
pub macro ArenaData {
	derive() ($( #[$meta:meta] )* $vis:vis struct $Ty:ident $($rest:tt)*) => {
		decl_arena! { $Ty }
	},

	derive() ($( #[$meta:meta] )* $vis:vis enum $Ty:ident $($rest:tt)*) => {
		decl_arena! { $Ty }
	}
}
