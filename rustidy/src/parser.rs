//! Parser

// Modules
mod error;
mod recursive;
mod str;

// Exports
pub use {
	self::{
		error::{ParseError, ParserError},
		recursive::{
			FromRecursiveRoot,
			IntoRecursiveRoot,
			ParsableRecursive,
			ParseRecursive,
			RecursiveWrapper,
			TryFromRecursiveRoot,
		},
		str::ParserStr,
	},
	rustidy_macros::Parse,
};

// Imports
use {
	crate::{
		Arenas,
		arena::{ArenaData, ArenaIdx, ArenasCheckpointStash, WithArena},
	},
	app_error::AppError,
	core::{
		marker::PhantomData,
		mem,
		ops::{Residual, Try},
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

impl<T> Parse for PhantomData<T> {
	type Error = !;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(_parser: &mut Parser) -> Result<Self, Self::Error> {
		Ok(Self)
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

impl<T: ArenaData<Data: Parse> + WithArena> Parse for ArenaIdx<T> {
	type Error = ParserError<T::Data>;

	fn name() -> Option<impl fmt::Display> {
		T::Data::name()
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let value = parser.parse::<T::Data>()?;
		let idx = parser.arenas.arena::<T>().push(value);
		Ok(idx)
	}
}

/// Parser
#[derive(Debug)]
pub struct Parser<'a, 'input> {
	/// Input
	input: &'input str,

	/// Current position
	cur_pos: ParserPos,

	/// Tags
	// Note: Always sorted by parser position.
	tags: Vec<(ParserPos, ParserTag)>,

	/// Arenas
	arenas: &'a Arenas,
}

impl<'a, 'input> Parser<'a, 'input> {
	/// Creates a new parser
	#[must_use]
	pub const fn new(input: &'input str, arenas: &'a Arenas) -> Self {
		Self {
			input,
			cur_pos: ParserPos(0),
			tags: vec![],
			arenas,
		}
	}

	/// Returns the whole input of the parser
	#[must_use]
	pub const fn input(&self) -> &'input str {
		self.input
	}

	/// Returns the arenas
	#[must_use]
	pub const fn arenas(&self) -> &'a Arenas {
		self.arenas
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

	/// Returns the string of an range
	#[must_use]
	pub fn str(&mut self, s: ParserStr) -> &'input str {
		s.range(self.arenas).str(self.input)
	}

	/// Returns if the parser is finished
	#[must_use]
	pub fn is_finished(&self) -> bool {
		self.remaining().is_empty()
	}

	/// Updates this parser from a string.
	///
	/// See [`Self::try_update_with`] for more details.
	pub fn update_with<F>(&mut self, f: F) -> ParserStr
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
	pub fn try_update_with<F, T>(&mut self, f: F) -> <T::Residual as Residual<ParserStr>>::TryType
	where
		F: FnOnce(&mut &'input str) -> T,
		T: Try<Output = (), Residual: Residual<ParserStr>>,
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

		let output_range = ParserRange {
			start: ParserPos(output_range.start),
			end:   ParserPos(output_range.end),
		};
		let idx = self.arenas.arena::<ParserStr>().push(output_range);

		<_>::from_output(ParserStr(idx))
	}

	/// Strips a prefix `s` from the parser
	#[expect(clippy::needless_pass_by_value, reason = "It's more ergonomic")]
	pub fn strip_prefix<S>(&mut self, s: S) -> Option<ParserStr>
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
		let arenas_checkpoint = self.arenas.checkpoint();
		match self.parse::<T>() {
			Ok(value) => Ok(Ok(value)),
			Err(err) if err.is_fatal() => Err(err),
			Err(err) => {
				self.cur_pos = prev_pos;
				self.arenas.undo_checkpoint(arenas_checkpoint);
				Ok(Err(err))
			},
		}
	}

	/// Peeks a `T` from this parser.
	///
	/// Parser is only advanced is a fatal error occurs
	#[expect(clippy::type_complexity, reason = "TODO")]
	pub fn peek<T: Parse>(&mut self) -> Result<Result<(T, PeekState), ParserError<T>>, ParserError<T>> {
		let start_pos = self.cur_pos;
		let arenas_checkpoint = self.arenas.checkpoint();

		let output = match self.parse::<T>() {
			Ok(value) => Ok(value),
			Err(err) if err.is_fatal() => return Err(err),
			Err(err) => Err(err),
		};

		let peek_state = PeekState {
			cur_pos:      self.cur_pos,
			arenas_stash: self.arenas.stash_checkpoint(arenas_checkpoint),
		};
		self.cur_pos = start_pos;

		let output = output.map(|value| (value, peek_state));
		Ok(output)
	}

	/// Accepts a peeked state.
	// TODO: We should validate that the user doesn't use a previous peek
	pub fn set_peeked(&mut self, peek_state: PeekState) {
		self.cur_pos = peek_state.cur_pos;
		self.arenas.apply_checkpoint_stash(peek_state.arenas_stash);
	}

	/// Returns all current tags
	pub fn tags(&self) -> impl Iterator<Item = ParserTag> {
		self.tags
			.iter()
			.rev()
			.take_while(|&&(pos, _)| pos == self.cur_pos)
			.map(|&(_, tag)| tag)
	}

	/// Returns if this parser has a tag
	#[must_use]
	pub fn has_tag(&self, tag_name: &'static str) -> bool {
		self.tags().any(|tag| tag.name == tag_name)
	}

	/// Calls `f` with tags `tags` added to this parser
	pub fn with_tags<O>(&mut self, tags: impl IntoIterator<Item = ParserTag>, f: impl FnOnce(&mut Self) -> O) -> O {
		let tags_len = self.tags.len();

		for tag in tags {
			self.tags.push((self.cur_pos, tag));
		}
		let output = f(self);
		self.tags.truncate(tags_len);

		output
	}

	/// Calls `f` with tag `tag` added to this parser
	pub fn with_tag<O>(&mut self, tag: impl Into<ParserTag>, f: impl FnOnce(&mut Self) -> O) -> O {
		self.with_tags([tag.into()], f)
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

/// Peek state
#[derive(Debug)]
pub struct PeekState {
	cur_pos:      ParserPos,
	arenas_stash: ArenasCheckpointStash,
}

impl PeekState {
	/// Returns if this peek state is further ahead than another
	#[must_use]
	pub fn ahead_of(&self, other: &Self) -> bool {
		self.cur_pos > other.cur_pos
	}

	/// Returns if this peek state is further ahead or equal to another
	#[must_use]
	pub fn ahead_of_or_equal(&self, other: &Self) -> bool {
		self.cur_pos >= other.cur_pos
	}
}

/// Parser range
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ParserRange {
	pub start: ParserPos,
	pub end:   ParserPos,
}

impl ParserRange {
	/// Creates a parser range from a start and end position
	#[must_use]
	pub const fn new(start: ParserPos, end: ParserPos) -> Self {
		Self { start, end }
	}

	/// Returns the length of this range
	#[must_use]
	pub const fn len(&self) -> usize {
		self.end.0 - self.start.0
	}

	/// Returns if this range is empty
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Slices the input string with this range
	#[must_use]
	pub fn str<'input>(&self, input: &'input str) -> &'input str {
		&input[self.start.0..self.end.0]
	}

	/// Slices the input string before this range
	#[must_use]
	pub fn str_before(self, s: &str) -> &str {
		&s[..self.start.0]
	}
}

/// Parser position
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
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
pub struct ParserTag {
	pub name: &'static str,
}

impl From<&'static str> for ParserTag {
	fn from(name: &'static str) -> Self {
		Self { name }
	}
}

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

/// `[Parser::parse]` for strings
pub fn parse_from_str<'a, F, E>(s: &mut &'a str, parse: F) -> Result<&'a str, E>
where
	F: FnOnce(&mut &'a str) -> Result<(), E>,
	E: ParseError,
{
	let start = *s;
	parse(s)?;
	let range = start.substr_range(s).expect("Output was not a substring of the input");
	let parsed = &start[..range.start];
	Ok(parsed)
}

/// `[Parser::try_parse]` for strings
pub fn try_parse_from_str<'a, F, E>(s: &mut &'a str, parse: F) -> Result<Result<&'a str, E>, E>
where
	F: FnOnce(&mut &'a str) -> Result<(), E>,
	E: ParseError,
{
	match self::parse_from_str(s, parse) {
		Ok(value) => Ok(Ok(value)),
		Err(err) if err.is_fatal() => Err(err),
		Err(err) => Ok(Err(err)),
	}
}
