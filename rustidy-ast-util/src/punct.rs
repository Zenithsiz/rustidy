//! Punctuated

// Imports
use {
	either::Either,
	rustidy_format::{Format, WsFmtFn},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// Punctuated type `T`, separated by `P`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(args(ty = "FmtArgs<WT, WP>", generic = "WT: WsFmtFn", generic = "WP: WsFmtFn"))]
#[format(where_format = "where T: Format<()>, P: Format<()>")]
// TODO: Support arguments for `T` and `P`
pub struct Punctuated<T, P> {
	pub first: T,
	#[format(prefix_ws = &args.punct)]
	#[format(args = rustidy_format::VecArgs::new(&args.punct, &args.value))]
	pub rest:  Vec<PunctuatedRest<T, P>>,
}

impl<T, P> Punctuated<T, P> {
	/// Creates a punctuated from a single value
	pub const fn single(value: T) -> Self {
		Self {
			first: value,
			rest:  vec![],
		}
	}

	/// Pushes a punctuation and value onto this punctuated
	pub fn push(&mut self, punct: P, value: T) {
		self.rest.push(PunctuatedRest { punct, value });
	}

	/// Pushes a value onto this punctuated, with a default punctuated
	pub fn push_value(&mut self, value: T)
	where
		P: Default,
	{
		self.push(P::default(), value);
	}

	/// Splits this punctuated at the first value
	pub fn split_first_mut(
		&mut self,
	) -> (
		&mut T,
		impl DoubleEndedIterator<Item = (&mut P, &mut T)> + ExactSizeIterator,
	) {
		(
			&mut self.first,
			self.rest
				.iter_mut()
				.map(|PunctuatedRest { punct, value }| (punct, value)),
		)
	}

	/// Splits this punctuated at the last value
	pub fn split_last_mut(&mut self) -> (SplitLastMut<'_, T, P>, &mut T) {
		let mut rest = self.rest.iter_mut();
		match rest.next_back() {
			Some(PunctuatedRest { punct, value }) => {
				let iter = SplitLastMut {
					next_value: Some(&mut self.first),
					last_punct: Some(punct),
					rest,
				};
				(iter, value)
			},
			None => {
				let iter = SplitLastMut {
					next_value: None,
					last_punct: None,
					rest,
				};
				(iter, &mut self.first)
			},
		}
	}

	/// Returns an iterator over all elements
	pub fn iter(&self) -> impl Iterator<Item = Either<&T, &P>> {
		itertools::chain![
			[Either::Left(&self.first)],
			self.rest
				.iter()
				.flat_map(|PunctuatedRest { punct, value }| [Either::Right(punct), Either::Left(value)])
		]
	}

	/// Returns a mutable iterator over all elements
	pub fn iter_mut(&mut self) -> impl Iterator<Item = Either<&mut T, &mut P>> {
		itertools::chain![
			[Either::Left(&mut self.first)],
			self.rest
				.iter_mut()
				.flat_map(|PunctuatedRest { punct, value }| [Either::Right(punct), Either::Left(value)])
		]
	}

	/// Returns an iterator over all values
	pub fn values(&self) -> impl Iterator<Item = &T> {
		itertools::chain![
			[&self.first],
			self.rest.iter().map(|PunctuatedRest { value, .. }| value)
		]
	}

	/// Returns a mutable iterator over all values
	pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
		itertools::chain![
			[&mut self.first],
			self.rest.iter_mut().map(|PunctuatedRest { value, .. }| value)
		]
	}

	/// Returns the number of values
	pub const fn values_len(&self) -> usize {
		1 + self.rest.len()
	}

	/// Returns an iterator over all punctuation
	pub fn puncts(&self) -> impl Iterator<Item = &P> {
		self.rest.iter().map(|PunctuatedRest { punct, .. }| punct)
	}

	/// Returns a mutable iterator over all punctuation
	pub fn puncts_mut(&mut self) -> impl Iterator<Item = &mut P> {
		self.rest.iter_mut().map(|PunctuatedRest { punct, .. }| punct)
	}

	/// Returns a mutable iterator over all punctuation
	pub fn punct_mut(&mut self) -> impl Iterator<Item = &mut P> {
		self.rest.iter_mut().map(|PunctuatedRest { punct, .. }| punct)
	}
}

/// Punctuated type `T`, separated by `P` with an optional trailing `P`.
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(args(ty = "FmtArgs<WT, WP>", generic = "WT: WsFmtFn", generic = "WP: WsFmtFn"))]
#[format(where_format = "where T: Format<()>, P: Format<()>")]
pub struct PunctuatedTrailing<T, P> {
	#[format(args = *args)]
	pub punctuated: Punctuated<T, P>,
	#[format(prefix_ws = args.punct)]
	pub trailing:   Option<P>,
}

impl<T, P> PunctuatedTrailing<T, P> {
	/// Creates a punctuated trailing from a single value
	pub const fn single(value: T) -> Self {
		Self {
			punctuated: Punctuated::single(value),
			trailing:   None,
		}
	}

	/// Pushes a value onto this punctuated
	pub fn push(&mut self, punct: P, value: T) {
		self.punctuated.push(punct, value);
	}

	/// Pushes a value onto this punctuated, with a default punctuated
	pub fn push_value(&mut self, value: T)
	where
		P: Default,
	{
		self.push(P::default(), value);
	}

	/// Splits this punctuated at the last value
	pub fn split_last_mut(&mut self) -> (SplitLastMut<'_, T, P>, &mut T, &mut Option<P>) {
		let (iter, last) = self.punctuated.split_last_mut();
		(iter, last, &mut self.trailing)
	}

	/// Returns an iterator over all elements
	pub fn iter(&self) -> impl Iterator<Item = Either<&T, &P>> {
		itertools::chain![self.punctuated.iter(), self.trailing.as_ref().map(Either::Right),]
	}

	/// Returns an iterator over all elements
	pub fn iter_mut(&mut self) -> impl Iterator<Item = Either<&mut T, &mut P>> {
		itertools::chain![self.punctuated.iter_mut(), self.trailing.as_mut().map(Either::Right),]
	}

	/// Returns an iterator over all values
	pub fn values(&self) -> impl Iterator<Item = &T> {
		self.punctuated.values()
	}

	/// Returns a mutable iterator over all values
	pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.punctuated.values_mut()
	}

	/// Returns the number of values
	pub const fn values_len(&self) -> usize {
		self.punctuated.values_len()
	}

	/// Returns an iterator over all punctuation
	pub fn puncts(&self) -> impl Iterator<Item = &P> {
		itertools::chain![self.punctuated.puncts(), &self.trailing]
	}

	/// Returns a mutable iterator over all punctuation
	pub fn puncts_mut(&mut self) -> impl Iterator<Item = &mut P> {
		itertools::chain![self.punctuated.puncts_mut(), &mut self.trailing]
	}
}

/// Iterator for [`Punctuated::split_last_mut`]
pub struct SplitLastMut<'a, T, P> {
	/// Next value to yield
	next_value: Option<&'a mut T>,

	/// Last punctuated to yield once the slice is empty
	last_punct: Option<&'a mut P>,

	/// Rest of the slice
	rest: std::slice::IterMut<'a, PunctuatedRest<T, P>>,
}

impl<'a, T, P> Iterator for SplitLastMut<'a, T, P> {
	type Item = (&'a mut T, &'a mut P);

	fn next(&mut self) -> Option<Self::Item> {
		// If we don't have a next value, we're finished
		let value = self.next_value.take()?;

		// If we do, get the punctuation
		let punct = match self.rest.next() {
			// If there's still something in the slice, save the value
			// for the next iteration
			Some(PunctuatedRest { punct, value }) => {
				self.next_value = Some(value);
				punct
			},

			// Otherwise, use our last punctuation
			// Note: If we had a next value, we're guaranteed
			//       to have a last punctuation.
			None => self.last_punct.take().expect("Should exist"),
		};

		Some((value, punct))
	}
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(args(ty = "WT", generic = "WT: WsFmtFn"))]
#[format(where_format = "where T: Format<()>, P: Format<()>")]
pub struct PunctuatedRest<T, P> {
	pub punct: P,
	#[format(prefix_ws = *args)]
	pub value: T,
}

/// Formatting arguments
pub struct FmtArgs<WT, WP> {
	pub value: WT,
	pub punct: WP,
}

impl<WT, WP> FmtArgs<WT, WP> {
	pub const fn new(value: WT, punct: WP) -> Self {
		Self { value, punct }
	}
}
