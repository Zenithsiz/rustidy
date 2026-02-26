//! Punctuated

// Imports
use {
	core::mem,
	either::Either,
	rustidy_format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// Punctuated type `T`, separated by `P`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "FmtArgs<TA, PA>", generic = "TA: Clone", generic = "PA: Clone"))]
#[format(where_format = "where T: Format<WhitespaceConfig, TA>, P: Format<WhitespaceConfig, PA>")]
pub struct Punctuated<T, P> {
	#[format(args = args.value_args.clone())]
	pub first: T,
	#[format(prefix_ws(if_ = output.has_prefix_ws(), expr = args.punct_prefix_ws))]
	#[format(args = rustidy_format::vec::args(args.punct_prefix_ws, args))]
	pub rest:  Vec<PunctuatedRest<T, P>>,
}

impl<T, P> Punctuated<T, P> {
	/// Creates a punctuated from a single value
	pub const fn single(value: T) -> Self {
		Self { first: value, rest: vec![], }
	}

	/// Pushes a punctuation and value at the front of this punctuated
	pub fn push_front(&mut self, value: T, punct: P) {
		let first = mem::replace(&mut self.first, value);
		self
			.rest
			.push(PunctuatedRest { punct, value: first });
	}

	/// Pushes a value onto the front of this punctuated, with a default punctuated
	pub fn push_front_value(&mut self, value: T)
	where
		P: Default,
	{
		self.push_front(value, P::default());
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
	pub fn split_first_mut(&mut self,) -> (&mut T, impl DoubleEndedIterator<Item = (&mut P, &mut T)> + ExactSizeIterator,) {
		(&mut self.first, self
			.rest
			.iter_mut()
			.map(|PunctuatedRest { punct, value }| (punct, value)),)
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
				let iter = SplitLastMut { next_value: None, last_punct: None, rest, };
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

	/// Gets a value by index mutably
	pub fn value_mut(&mut self, idx: usize) -> Option<&mut T> {
		match idx.checked_sub(1) {
			Some(idx) => self
				.rest
				.get_mut(idx)
				.map(|rest| &mut rest.value),
			None => Some(&mut self.first),
		}
	}

	/// Gets a punctuation by index mutably
	pub fn punct_mut(&mut self, idx: usize) -> Option<&mut P> {
		self
			.rest
			.get_mut(idx)
			.map(|rest| &mut rest.punct)
	}

	/// Returns an iterator over all punctuation
	pub fn puncts(&self) -> impl Iterator<Item = &P> {
		self
			.rest
			.iter()
			.map(|PunctuatedRest { punct, .. }| punct)
	}

	/// Returns a mutable iterator over all punctuation
	pub fn puncts_mut(&mut self) -> impl Iterator<Item = &mut P> {
		self
			.rest
			.iter_mut()
			.map(|PunctuatedRest { punct, .. }| punct)
	}
}

/// Punctuated type `T`, separated by `P` with an optional trailing `P`.
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "FmtArgs<TA, PA>", generic = "TA: Clone", generic = "PA: Clone"))]
#[format(where_format = "where T: Format<WhitespaceConfig, TA>, P: Format<WhitespaceConfig, PA>")]
pub struct PunctuatedTrailing<T, P> {
	#[format(args = args.clone())]
	pub punctuated: Punctuated<T, P>,
	#[format(prefix_ws(if_ = output.has_prefix_ws(), expr = args.punct_prefix_ws))]
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

	/// Extends this container with another punctuated trailing
	pub fn extend_from_punctuated_trailing(&mut self, other: Self)
	where
		P: Default
	{
		self.punctuated.rest.push(PunctuatedRest {
			punct: self.trailing.take().unwrap_or_default(),
			value: other.punctuated.first
		});

		self
			.punctuated
			.rest
			.extend(other.punctuated.rest);

		self.trailing = other.trailing;
	}

	/// Pushes a value onto this the front of this punctuated, with a default punctuated
	pub fn push_front(&mut self, value: T, punct: P) {
		self.punctuated.push_front(value, punct);
	}

	/// Pushes a value onto this the front of this punctuated, with a default punctuated
	pub fn push_front_value(&mut self, value: T)
	where
		P: Default,
	{
		self.push_front(value, P::default());
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

	/// Uses this punctuated as a `Vec<(T, Option<P>)>`
	fn with_values_vec(&mut self, f: impl FnOnce(&mut Vec<(T, Option<P>)>))
	where
		P: Default
	{
		replace_with::replace_with_or_abort(self, |this| {
			let mut values = vec![];

			let mut next_value = Some(this.punctuated.first);
			let mut rest = this.punctuated.rest.into_iter();
			while let Some(cur_value) = next_value.take() {
				match rest.next() {
					Some(PunctuatedRest { punct, value }) => {
						next_value = Some(value);
						values.push((cur_value, Some(punct)));
					},
					None => {
						values.push((cur_value, this.trailing));
						break;
					},
				}
			}

			f(&mut values);

			let mut values = values.into_iter();
			let (first, mut next_punct) = values
				.next()
				.expect("Should have at least one element");

			let mut punctuated = Punctuated { first, rest: vec![], };
			for (value, punct) in values {
				let cur_punct = next_punct.unwrap_or_default();
				next_punct = punct;
				punctuated
					.rest
					.push(PunctuatedRest { punct: cur_punct, value });
			}


			Self { punctuated, trailing: next_punct }
		});
	}

	/// Sorts the values in this punctuated by a key
	pub fn sort_values_by_key(
		&mut self,
		mut f: impl for<'a> FnMut<(&'a T, Option<&'a P>), Output: Ord>
	)
	where
		P: Default
	{
		self.with_values_vec(|values| values.sort_by(
			|(lhs_value, lhs_punct), (rhs_value, rhs_punct)| {
				let lhs = f(lhs_value, lhs_punct.as_ref());
				let rhs = f(rhs_value, rhs_punct.as_ref());

				lhs.cmp(&rhs)
			}
		));
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

#[derive(PartialEq, Eq, Clone, Debug)]
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
pub const fn fmt_with<TA, PA>(
	value: WhitespaceConfig,
	punct: WhitespaceConfig,
	value_args: TA,
	punct_args: PA,
) -> FmtArgs<TA, PA> {
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

#[derive(Clone, Copy, Debug)]
pub struct FmtIndentColumns {
	pub columns: Option<usize>,
}

impl<T: Format<WhitespaceConfig, ()>, P: Format<WhitespaceConfig, ()>> Format<WhitespaceConfig, FmtIndentColumns> for Punctuated<T, P> {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtIndentColumns
	) -> rustidy_format::FormatOutput {
		let mut output = FormatOutput::default();

		let mut cur_idx = 0;
		let mut prefix_ws = Some(prefix_ws);
		'values: while let Some(first) = self.value_mut(cur_idx) {
			ctx
				.format(first, prefix_ws.unwrap_or(Whitespace::INDENT))
				.append_to(&mut output);
			prefix_ws.take_if(|_| !output.is_empty);

			let Some(punct) = self.punct_mut(cur_idx) else { break };
			ctx
				.format(punct, prefix_ws.unwrap_or(Whitespace::REMOVE))
				.append_to(&mut output);
			prefix_ws.take_if(|_| !output.is_empty);
			cur_idx += 1;

			let row_len = args.columns.unwrap_or(1);
			let row_rest_len = row_len.saturating_sub(1);
			for _ in 0..row_rest_len {
				let Some(value) = self.value_mut(cur_idx) else { break 'values };
				ctx
					.format(value, prefix_ws.unwrap_or(Whitespace::SINGLE))
					.append_to(&mut output);
				prefix_ws.take_if(|_| !output.is_empty);

				let Some(punct) = self.punct_mut(cur_idx) else { break 'values };
				ctx
					.format(punct, prefix_ws.unwrap_or(Whitespace::REMOVE))
					.append_to(&mut output);
				prefix_ws.take_if(|_| !output.is_empty);

				cur_idx += 1;
			}
		}

		output
	}
}

impl<T: Format<WhitespaceConfig, ()>, P: Format<WhitespaceConfig, ()>> Format<WhitespaceConfig, FmtIndentColumns> for PunctuatedTrailing<T, P> {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtIndentColumns
	) -> rustidy_format::FormatOutput {
		let mut output = FormatOutput::default();
		ctx
			.format_with(&mut self.punctuated, prefix_ws, args)
			.append_to(&mut output);
		ctx
			.format(&mut self.trailing, Whitespace::REMOVE)
			.append_to(&mut output);

		output
	}
}
