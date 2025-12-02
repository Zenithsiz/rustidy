//! Parser

// Modules
mod error;
mod recursive;

// TODO: Replace all usages of `AstStr` with `ParserRange`?

// Exports
pub use {
	self::{
		error::{ParseError, ParserError},
		recursive::{ParsableRecursive, ParseRecursive, RecursiveWrapper},
	},
	rustidy_macros::Parse,
};

// Imports
use {
	crate::AstStr,
	app_error::AppError,
	core::{
		mem,
		ops::{Index, Range, Residual, Try},
	},
	std::{fmt, str::pattern::Pattern},
};


/// Parsable types
pub trait Parse: Sized {
	/// Error type
	type Error: ParseError;

	/// A name for this type.
	///
	/// This is mostly used in error reporting and should be a lower case name,
	/// with `a` / `an` prefixed. Used for example in the following way:
	/// `expected {name}, found {other_name}`.
	///
	/// If this returns `None`, no extra error is displayed when parsing the type.
	/// This is useful for containers that usually don't want to expose themselves
	fn name() -> Option<impl fmt::Display>;

	/// Parses this type from `input`, mutating it in-place.
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error>;
}

#[derive(Debug)]
pub struct NeverError;

impl ParseError for NeverError {
	fn is_fatal(&self) -> bool {
		false
	}

	fn pos(&self) -> Option<ParserPos> {
		None
	}

	fn to_app_error(&self, _parser: &Parser) -> AppError {
		AppError::from_multiple([])
	}
}

impl Parse for ! {
	type Error = NeverError;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(_parser: &mut Parser) -> Result<Self, Self::Error> {
		Err(NeverError)
	}
}

impl<T> Parse for Box<T>
where
	T: Parse,
{
	type Error = ParserError<T>;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser.parse::<T>().map(Self::new)
	}
}

impl<T> Parse for Option<T>
where
	T: Parse,
{
	type Error = ParserError<T>;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser.try_parse::<T>().map(Result::ok)
	}
}

impl<T> Parse for Vec<T>
where
	T: Parse,
{
	type Error = ParserError<T>;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let mut values = vec![];
		loop {
			let start_pos = parser.cur_pos;
			match parser.try_parse::<T>()? {
				Ok(value) if parser.cur_pos != start_pos => values.push(value),
				_ => break,
			}
		}

		Ok(values)
	}
}

impl Parse for () {
	type Error = !;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(_parser: &mut Parser) -> Result<Self, Self::Error> {
		Ok(())
	}
}

macro tuple_impl($N:literal, $($T:ident),* $(,)?) {
	#[derive(Debug, Parse)]
	struct ${concat( Tuple, $N )}< $( $T, )* >( $( $T, )* );

	#[automatically_derived]
	impl< $($T: Parse,)* > Parse for ( $($T,)* ) {
		type Error = ${concat( Tuple, $N, Error )}< $($T,)* >;

		#[coverage(off)]
		fn name() -> Option<impl fmt::Display> {
			None::<!>
		}

		#[expect(non_snake_case)]
		fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
			let ${concat( Tuple, $N )}( $($T,)* ) = < ${concat( Tuple, $N )}< $($T,)* > >::parse_from(parser)?;
			Ok( ( $($T,)* ) )
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }
tuple_impl! { 4, T0, T1, T2, T3 }

/// Parser
#[derive(Debug)]
pub struct Parser<'input> {
	/// Input
	input: &'input str,

	/// Current position
	cur_pos: ParserPos,

	/// Tags
	tags: Vec<ParserTag>,
}

impl<'input> Parser<'input> {
	/// Creates a new parser
	#[must_use]
	pub const fn new(input: &'input str) -> Self {
		Self {
			input,
			cur_pos: ParserPos(0),
			tags: vec![],
		}
	}

	/// Returns the whole input of the parser
	#[must_use]
	pub const fn input(&self) -> &'input str {
		self.input
	}

	/// Returns the remaining string for the parser
	#[must_use]
	pub fn remaining(&self) -> &'input str {
		&self.input[self.cur_pos.0..]
	}

	/// Returns the current position of the parser
	pub const fn cur_pos(&mut self) -> ParserPos {
		self.cur_pos
	}

	/// Sets the position of this parser
	pub const fn set_pos(&mut self, pos: ParserPos) {
		self.cur_pos = pos;
	}

	/// Reverses all whitespace (except the last) in the current position
	pub fn reverse_whitespace(&mut self) {
		self.cur_pos.0 = self.input[..self.cur_pos.0]
			.rfind(|ch: char| !ch.is_whitespace())
			.map_or(0, |idx| idx + 1);
	}

	/// Reverses to the start of the current line
	pub fn reverse_line(&mut self) {
		self.cur_pos.0 = self.input[..self.cur_pos.0].rfind('\n').map_or(0, |idx| idx + 1);
	}

	/// Returns the current line of the parser, not including the end
	#[must_use]
	pub fn cur_line(&self) -> &'input str {
		let start = self.input[..self.cur_pos.0].rfind('\n').map_or(0, |idx| idx + 1);
		let end = self.cur_pos.0 +
			self.input[self.cur_pos.0..]
				.find('\n')
				.unwrap_or(self.input.len() - self.cur_pos.0);

		&self.input[start..end]
	}

	/// Gets the position (0-indexed) of the parser at a position
	#[must_use]
	pub fn loc(&self, pos: ParserPos) -> ParserLoc {
		let line = self.input[..pos.0].chars().filter(|&ch| ch == '\n').count();
		let column = match self.input[..pos.0].rfind('\n') {
			Some(newline_pos) => pos.0 - newline_pos - 1,
			None => pos.0,
		};

		ParserLoc { line, column }
	}

	/// Gets the current position (0-indexed) of the parser
	#[must_use]
	pub fn cur_loc(&self) -> ParserLoc {
		self.loc(self.cur_pos)
	}

	/// Returns the string of an `AstStr`.
	///
	/// Ignores any replacement on the string
	#[must_use]
	pub fn str(&self, s: &AstStr) -> &'input str {
		&self.input[s.range()]
	}

	/// Returns everything after a range.
	#[must_use]
	pub fn str_after(&self, s: &AstStr) -> &'input str {
		&self.input[s.range().0.end.0..]
	}

	/// Returns if the parser is finished
	#[must_use]
	pub fn is_finished(&self) -> bool {
		self.remaining().is_empty()
	}

	/// Updates this parser from a string.
	///
	/// See [`Self::try_update_with`] for more details.
	pub fn update_with<F>(&mut self, f: F) -> AstStr
	where
		F: FnOnce(&mut &'input str),
	{
		self.try_update_with(|remaining| {
			f(remaining);
			Ok::<_, !>(())
		})
		.into_ok()
	}

	/// Updates this parser from a string.
	///
	/// The function `f` receives a string to update.
	/// The value it is updated with *must* be a substring of the
	/// received function.
	///
	/// # Success
	/// When `f` returns successfully, the parser is updated from
	/// the state of the string.
	///
	/// # Failure
	/// If `f` returns unsuccessfully, an error will be returned
	/// with the latest change to the string as it's position.
	pub fn try_update_with<F, T>(&mut self, f: F) -> <T::Residual as Residual<AstStr>>::TryType
	where
		F: FnOnce(&mut &'input str) -> T,
		T: Try<Output = (), Residual: Residual<AstStr>>,
	{
		let mut remaining = self.remaining();
		let res = f(&mut remaining);

		let range = self
			.remaining()
			.substr_range(remaining)
			.expect("Result was not a substring of the input");
		assert_eq!(self.cur_pos.0 + range.end, self.input.len());

		let output = &self.remaining()[..range.start];
		self.cur_pos.0 += range.start;

		// After updating the remaining, quit if an error occurred
		let () = res?;

		// Else get the output string
		let output_range = self
			.input
			.substr_range(output)
			.expect("Output was not a substring of the input");

		let output_range = ParserRange(ParserPos(output_range.start)..ParserPos(output_range.end));

		<_>::from_output(AstStr::new(output_range))
	}

	/// Strips a prefix `s` from the parser
	#[expect(clippy::needless_pass_by_value, reason = "It's more ergonomic")]
	pub fn strip_prefix<S>(&mut self, s: S) -> Option<AstStr>
	where
		S: Pattern + Clone + Into<String>,
	{
		self.try_update_with(|remaining| {
			*remaining = remaining.strip_prefix(s.clone())?;
			Some(())
		})
	}

	/// Parses `T` from this parser
	pub fn parse<T: Parse>(&mut self) -> Result<T, ParserError<T>> {
		let start_pos = self.cur_pos;
		T::parse_from(self).map_err(|source| ParserError::new(source, ParserRange::new(start_pos, self.cur_pos)))
	}

	/// Tries to parses `T` from this parser.
	///
	/// On error, nothing is modified.
	pub fn try_parse<T: Parse>(&mut self) -> Result<Result<T, ParserError<T>>, ParserError<T>> {
		let prev_pos = self.cur_pos;
		match self.parse::<T>() {
			Ok(value) => Ok(Ok(value)),
			Err(err) if err.is_fatal() => Err(err),
			Err(err) => {
				self.cur_pos = prev_pos;
				Ok(Err(err))
			},
		}
	}

	/// Peeks a `T` from this parser.
	///
	/// Parser is only advanced is a fatal error occurs
	#[expect(clippy::type_complexity, reason = "TODO")]
	pub fn peek<T: Parse>(&mut self) -> Result<Result<(T, ParserPos), ParserError<T>>, ParserError<T>> {
		let start_pos = self.cur_pos;
		let output = match self.parse::<T>() {
			Ok(value) => Ok((value, self.cur_pos)),
			Err(err) if err.is_fatal() => return Err(err),
			Err(err) => Err(err),
		};
		self.cur_pos = start_pos;

		Ok(output)
	}

	/// Returns if this parser has a tag
	pub fn has_tag(&self, tag: impl Into<ParserTag>) -> bool {
		self.tags.contains(&tag.into())
	}

	/// Calls `f` with tag `tag` added to this parser
	pub fn with_tag<O>(&mut self, tag: impl Into<ParserTag>, f: impl FnOnce(&mut Self) -> O) -> O {
		self.tags.push(tag.into());
		let output = f(self);
		self.tags.pop();
		output
	}

	/// Calls `f` with all tags removed.
	pub fn without_tags<O>(&mut self, f: impl FnOnce(&mut Self) -> O) -> O {
		// TODO: Just add an offset to the start of the new tags
		//       to reduce an allocation?
		let tags = mem::take(&mut self.tags);
		let output = f(self);
		self.tags = tags;

		output
	}
}

/// Parser range
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(derive_more::From)]
#[serde(transparent)]
pub struct ParserRange(Range<ParserPos>);

impl ParserRange {
	/// An empty range, used for missing ranges
	pub const EMPTY: Self = Self(ParserPos(0)..ParserPos(0));

	/// Creates a parser range from a start and end position
	#[must_use]
	pub const fn new(start: ParserPos, end: ParserPos) -> Self {
		Self(start..end)
	}

	/// Returns the start of this range
	#[must_use]
	pub const fn start(&self) -> ParserPos {
		self.0.start
	}

	/// Returns the end of this range
	#[must_use]
	pub const fn end(&self) -> ParserPos {
		self.0.end
	}

	/// Returns the length of this range
	#[must_use]
	pub const fn len(&self) -> usize {
		self.0.end.0 - self.0.start.0
	}

	/// Returns if this range is empty
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

impl Index<ParserRange> for str {
	type Output = Self;

	fn index(&self, index: ParserRange) -> &Self::Output {
		&self[index.0.start.0..index.0.end.0]
	}
}

/// Parser position
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(derive_more::From)]
#[serde(transparent)]
pub struct ParserPos(usize);

impl ParserPos {
	/// Creates a parser position from a usize
	// TODO: Should we allow this?
	#[must_use]
	pub const fn from_usize(pos: usize) -> Self {
		Self(pos)
	}

	/// Returns the index corresponding to this position
	// TODO: Should we allow this?
	#[must_use]
	pub const fn to_usize(self) -> usize {
		self.0
	}
}

/// Parser tag
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[derive(derive_more::From)]
pub struct ParserTag(pub &'static str);

/// Parser location (0-indexed).
///
/// # Display
/// The display impl transforms this into a 1-indexed format of `{line}:{column}`
#[derive(Debug)]
#[derive(derive_more::Display)]
#[display("{}:{}", line+1, column+1)]
pub struct ParserLoc {
	pub line:   usize,
	pub column: usize,
}

/// Types that may be parsed from another
pub trait ParsableFrom<T> {
	fn from_parsable(value: T) -> Self;
}
