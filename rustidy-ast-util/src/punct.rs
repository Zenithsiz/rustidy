//! Punctuated

// Imports
use {
	either::Either,
	rustidy_format::{Format, Formattable, WhitespaceConfig},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// Punctuated type `T`, separated by `P`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "FmtArgs<TA, PA>", generic = "TA: Clone", generic = "PA: Clone"))]
#[format(where_format = "where T: Format<WhitespaceConfig, TA>, P: Format<WhitespaceConfig, PA>")]
pub struct Punctuated<T, P> {
	#[format(args = args.value_args.clone())]
	pub first: T,
	#[format(prefix_ws(if = output.has_prefix_ws(), expr = args.punct_prefix_ws))]
	#[format(args = rustidy_format::vec::args(args.punct_prefix_ws, args))]
	pub rest:  Vec<PunctuatedRest<T, P>>,
}

impl<T, P> Punctuated<T, P> {
	/// Creates a punctuated from a single value
	pub const fn single(value: T) -> Self {
		Self {
			first: value,
			rest: vec![],
		}
	}

	/// Pushes a punctuation and value onto this punctuated
	pub fn push(&mut self, punct: P, value: T) {
		self
			.rest
			.push(PunctuatedRest {
				punct,
				value
			});
	}

	/// Pushes a value onto this punctuated, with a default punctuated
	pub fn push_value(&mut self, value: T)
	where
		P: Default,
	{
		self.push(P::default(), value);
	}

	/// Splits this punctuated at the first value
	pub fn split_first_mut(&mut self,) -> (&mut T, impl DoubleEndedIterator<Item = (&mut P, &mut T)> + ExactSizeIterator,) {
		(&mut self.first, self
			.rest
			.iter_mut()
			.map(|PunctuatedRest {
				punct,
				value
			}| (punct, value)),)
	}

	/// Splits this punctuated at the last value
	pub fn split_last_mut(&mut self) -> (SplitLastMut<'_, T, P>, &mut T) {
		let mut rest = self.rest.iter_mut();
		match rest.next_back() {
			Some(PunctuatedRest {
				punct,
				value
			}) => {
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
		self
			.rest
			.iter()
			.map(|PunctuatedRest {
				punct,
				..
			}| punct)
	}

	/// Returns a mutable iterator over all punctuation
	pub fn puncts_mut(&mut self) -> impl Iterator<Item = &mut P> {
		self
			.rest
			.iter_mut()
			.map(|PunctuatedRest {
				punct,
				..
			}| punct)
	}

	/// Returns a mutable iterator over all punctuation
	pub fn punct_mut(&mut self) -> impl Iterator<Item = &mut P> {
		self
			.rest
			.iter_mut()
			.map(|PunctuatedRest {
				punct,
				..
			}| punct)
	}
}

/// Punctuated type `T`, separated by `P` with an optional trailing `P`.
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "FmtArgs<TA, PA>", generic = "TA: Clone", generic = "PA: Clone"))]
#[format(where_format = "where T: Format<WhitespaceConfig, TA>, P: Format<WhitespaceConfig, PA>")]
pub struct PunctuatedTrailing<T, P> {
	#[format(args = args.clone())]
	pub punctuated: Punctuated<T, P>,
	#[format(prefix_ws(if = output.has_prefix_ws(), expr = args.punct_prefix_ws))]
	#[format(args = args.punct_args)]
	pub trailing:   Option<P>,
}

impl<T, P> PunctuatedTrailing<T, P> {
	/// Creates a punctuated trailing from a single value
	pub const fn single(value: T) -> Self {
		Self {
			punctuated: Punctuated::single(value),
			trailing: None,
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
	rest:       std::slice::IterMut<'a, PunctuatedRest<T, P>>,
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
			Some(PunctuatedRest {
				punct,
				value
			}) => {
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
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "FmtArgs<TA, PA>", generic = "TA", generic = "PA"))]
#[format(where_format = "where T: Format<WhitespaceConfig, TA>, P: Format<WhitespaceConfig, PA>")]
pub struct PunctuatedRest<T, P> {
	#[format(args = args.punct_args)]
	pub punct: P,
	#[format(prefix_ws = args.value_prefix_ws)]
	#[format(args = args.value_args)]
	pub value: T,
}

/// Formatting arguments
#[derive(Clone, Copy, Debug)]
pub struct FmtArgs<TA, PA> {
	pub value_prefix_ws: WhitespaceConfig,
	pub punct_prefix_ws: WhitespaceConfig,

	pub value_args:      TA,
	pub punct_args:      PA,
}

#[must_use]
pub const fn fmt_with<TA, PA>(value: WhitespaceConfig, punct: WhitespaceConfig, value_args: TA, punct_args: PA,) -> FmtArgs<TA, PA> {
	FmtArgs {
		value_prefix_ws: value,
		punct_prefix_ws: punct,
		value_args,
		punct_args,
	}
}

#[must_use]
pub const fn fmt(value: WhitespaceConfig, punct: WhitespaceConfig) -> FmtArgs<(), ()> {
	self::fmt_with(value, punct, (), ())
}
