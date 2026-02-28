//! Parser error

// Exports
pub use macros::ParseError;

// Imports
use {
	super::{AstPos, AstRange, Parse, Parser},
	app_error::AppError,
	core::{error::Error as StdError, fmt},
};

/// Parse error
pub trait ParseError {
	/// Returns whether this error is fatal
	fn is_fatal(&self) -> bool;

	/// Returns the position at which this error occurred.
	fn pos(&self) -> Option<AstPos>;

	/// Converts this error type to an `AppError`
	fn to_app_error(&self, parser: &Parser) -> AppError;
}

impl ParseError for ! {
	fn is_fatal(&self) -> bool {
		*self
	}

	fn pos(&self) -> Option<AstPos> {
		*self
	}

	fn to_app_error(&self, _parser: &Parser) -> AppError {
		*self
	}
}

impl<E: ParseError> ParseError for Box<E> {
	fn is_fatal(&self) -> bool {
		(**self).is_fatal()
	}

	fn pos(&self) -> Option<AstPos> {
		(**self).pos()
	}

	fn to_app_error(&self, parser: &Parser) -> AppError {
		(**self).to_app_error(parser)
	}
}

impl ParseError for () {
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


/// Parser error
pub struct ParserError<T: Parse> {
	// Note: This is behind an indirection to avoid overflowing the stack when
	//       we parse large enums.
	// TODO: Make this an `ArenaIdx` once either `#[fundamental]` can be applied
	//       to enums, or generic statics become available.
	source: Box<T::Error>,
	range:  AstRange,
}

impl<T: Parse> ParserError<T> {
	pub(super) fn new(source: T::Error, range: AstRange) -> Self {
		Self { source: Box::new(source), range, }
	}
}

impl<T: Parse<Error: fmt::Debug>> fmt::Debug for ParserError<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f
			.debug_struct("ParserError")
			.field("source", &self.source)
			.field("span", &self.range)
			.finish()
	}
}

impl<T: Parse<Error: StdError + 'static>> StdError for ParserError<T> {
	fn source(&self) -> Option<&( dyn StdError + 'static )> {
		match self::name_of::<T>().is_some() {
			true => Some(&self.source),
			false => self.source.source(),
		}
	}
}

impl<T: Parse<Error: fmt::Display>> fmt::Display for ParserError<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self::name_of::<T>() {
			Some(name) => write!(f, "Expected {name}"),
			None => self.source.fmt(f),
		}
	}
}

impl<T: Parse> ParseError for ParserError<T> {
	fn is_fatal(&self) -> bool {
		self.source.is_fatal()
	}

	fn pos(&self) -> Option<AstPos> {
		// Note: We prefer deeper positions since they contain the
		//       nearest position to the error.
		let pos = match self.source.pos() {
			Some(pos) => AstPos::max(pos, self.range.end),
			None => self.range.end,
		};

		Some(pos)
	}

	fn to_app_error(&self, parser: &Parser) -> AppError {
		let err = self.source.to_app_error(parser).flatten();
		match self::name_of::<T>() {
			Some(name) => err.with_context(
				|| format!("Expected {name} at {}", parser.loc(self.range.start))
			),
			None => err,
		}
	}
}

/// Gets the name of a parsable type
fn name_of<T: Parse>() -> Option<String> {
	let name = T::name().map(|s| s.to_string());

	#[cfg(feature = "parse-debug-name")]
	let name = Some(
		name
			.unwrap_or_else(|| std::any::type_name::<T>().to_owned())
	);

	name
}
