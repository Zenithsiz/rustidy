//! Parsing

// Features
#![feature(
	never_type,
	try_trait_v2,
	try_trait_v2_residual,
	pattern,
	coverage_attribute,
	decl_macro,
	macro_metavar_expr_concat,
	unwrap_infallible,
	substr_range
)]

// Modules
mod error;
mod recursive;

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
	},
	rustidy_macros::Parse,
};

// Imports
use {
	app_error::AppError,
	core::{
		marker::PhantomData,
		mem,
		ops::{Residual, Try},
	},
	rustidy_util::{ArenaData, ArenaIdx, AstPos, AstRange, AstStr, Config},
	std::{borrow::Cow, fmt, str::pattern::Pattern},
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
	#[must_use]
	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	/// Parses this type from `input`, mutating it in-place.
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error>;
}

#[derive(Debug)]
pub struct NeverError;

impl ParseError for NeverError {
	fn is_fatal(&self) -> bool {
		false
	}

	fn pos(&self) -> Option<AstPos> {
		None
	}

	fn to_app_error(&self, _parser: &Parser) -> AppError {
		AppError::from_multiple([])
	}
}

impl Parse for ! {
	type Error = NeverError;

	fn parse_from(_parser: &mut Parser) -> Result<Self, Self::Error> {
		Err(NeverError)
	}
}

impl<T> Parse for PhantomData<T> {
	type Error = !;

	fn parse_from(_parser: &mut Parser) -> Result<Self, Self::Error> {
		Ok(Self)
	}
}

impl<T> Parse for Box<T>
where
	T: Parse,
{
	type Error = ParserError<T>;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser.parse::<T>().map(Self::new)
	}
}

impl<T> Parse for Option<T>
where
	T: Parse,
{
	type Error = ParserError<T>;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser.try_parse::<T>().map(Result::ok)
	}
}

impl<T> Parse for Vec<T>
where
	T: Parse,
{
	type Error = ParserError<T>;

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

	fn parse_from(_parser: &mut Parser) -> Result<Self, Self::Error> {
		Ok(())
	}
}

macro tuple_impl($N:literal, $($T:ident),* $(,)?) {
	#[automatically_derived]
	impl< $($T: Parse,)* > Parse for ( $($T,)* ) {
		type Error = ${concat( Tuple, $N, Error )}< $($T,)* >;

		#[expect(non_snake_case)]
		fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
			$(
				let $T = parser.parse().map_err(Self::Error::$T)?;
			)*
			Ok(( $( $T, )* ))
		}
	}

	#[derive(derive_more::Debug)]
	pub enum ${concat( Tuple, $N, Error )}< $($T: Parse,)* > {
		$(
			$T(ParserError<$T>),
		)*
	}

	#[automatically_derived]
	impl< $($T: Parse,)* > ParseError for ${concat( Tuple, $N, Error )}< $($T,)* > {
		fn is_fatal(&self) -> bool {
			match *self {
				$(
					Self::$T(ref err, ..) => err.is_fatal(),
				)*
			}
		}

		fn pos(&self) -> Option<AstPos> {
			match *self {
				$(
					Self::$T(ref err, ..) => err.pos(),
				)*
			}
		}

		fn to_app_error(&self, parser: &Parser) -> AppError {
			match *self {
				$(
					Self::$T(ref err, ..) => err.to_app_error(parser),
				)*
			}
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }
tuple_impl! { 4, T0, T1, T2, T3 }

impl<T: ArenaData<Data: Parse>> Parse for ArenaIdx<T> {
	type Error = ParserError<T::Data>;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let value = parser.parse::<T::Data>()?;
		let idx = Self::new(value);
		Ok(idx)
	}
}

/// Parser
#[derive(Debug)]
pub struct Parser<'a, 'input> {
	/// Input
	input: &'input str,

	/// Current position
	cur_pos: AstPos,

	/// Tags
	// Note: Always sorted by ast position.
	tags: Vec<(AstPos, ParserTag)>,

	config: &'a Config,
}

impl<'a, 'input> Parser<'a, 'input> {
	/// Creates a new parser
	#[must_use]
	pub const fn new(input: &'input str, config: &'a Config) -> Self {
		Self {
			input,
			cur_pos: AstPos::from_usize(0),
			tags: vec![],
			config,
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
	pub const fn cur_pos(&mut self) -> AstPos {
		self.cur_pos
	}

	/// Sets the position of this parser
	pub const fn set_pos(&mut self, pos: AstPos) {
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
	pub fn loc(&self, pos: AstPos) -> ParserLoc {
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
	pub fn str(&mut self, s: &AstStr) -> Cow<'input, str> {
		s.str(self.input, self.config)
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

		let output_range = AstRange {
			start: AstPos(output_range.start),
			end:   AstPos(output_range.end),
		};

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
		T::parse_from(self).map_err(|source| ParserError::new(source, AstRange::new(start_pos, self.cur_pos)))
	}

	/// Parses `T` from this parser with a peeked value
	pub fn parse_with_peeked<T: ParsePeeked<U>, U>(&mut self, parsed: U) -> Result<T, ParserError<T>> {
		let start_pos = self.cur_pos;
		T::parse_from_with_peeked(self, parsed)
			.map_err(|source| ParserError::new(source, AstRange::new(start_pos, self.cur_pos)))
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
	pub fn peek<T: Parse>(&mut self) -> Result<Result<(T, PeekState), ParserError<T>>, ParserError<T>> {
		let start_pos = self.cur_pos;
		let output = match self.parse::<T>() {
			Ok(value) => Ok(value),
			Err(err) if err.is_fatal() => return Err(err),
			Err(err) => Err(err),
		};

		let peek_state = PeekState { cur_pos: self.cur_pos };
		self.cur_pos = start_pos;

		let output = output.map(|value| (value, peek_state));
		Ok(output)
	}

	/// Accepts a peeked state.
	#[expect(
		clippy::needless_pass_by_value,
		reason = "It's to ensure the user doesn't use the same peek state multiple times"
	)]
	pub const fn set_peeked(&mut self, peek_state: PeekState) {
		self.cur_pos = peek_state.cur_pos;
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
	cur_pos: AstPos,
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

/// Types that may be parsed using a peek into itself
pub trait ParsePeeked<T>: Parse {
	fn parse_from_with_peeked(parser: &mut Parser, parsed: T) -> Result<Self, Self::Error>;
}

impl<T, U> ParsePeeked<U> for T
where
	T: Parse + From<U>,
{
	fn parse_from_with_peeked(_parser: &mut Parser, parsed: U) -> Result<Self, Self::Error> {
		Ok(parsed.into())
	}
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
