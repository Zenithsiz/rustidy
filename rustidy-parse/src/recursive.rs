//! Recursively parsable types

// Exports
pub use macros::ParseRecursive;

// Imports
use {
	crate::{self as parse},
	super::{ParsableFrom, Parse, ParseError, Parser, ParserError, ParserTag},
	core::marker::PhantomData,
	util::AstPos,
};

/// Recursive type
pub trait ParsableRecursive<R> {
	/// The prefix for this type
	type Prefix: Parse;

	/// Base type
	type Base: Parse;

	/// The suffix for this type
	type Suffix: Parse;

	/// The infix of this type
	type Infix: Parse;

	/// Creates this type from it's parts
	fn join_prefix(prefix: Self::Prefix, root: R, parser: &mut Parser) -> Self;

	/// Converts the base to this type
	fn from_base(base: Self::Base, parser: &mut Parser) -> Self;

	/// Creates this type from it's parts
	fn join_suffix(root: R, suffix: Self::Suffix, parser: &mut Parser) -> Self;

	/// Creates this type from it's parts
	fn join_infix(lhs: R, infix: Self::Infix, rhs: R, parser: &mut Parser) -> Self;
}

pub trait FromRecursiveRoot<R> {
	fn from_recursive_root(root: R, parser: &mut Parser) -> Self;
}

impl<R, T> FromRecursiveRoot<R> for T
where
	T: From<R>, {
	fn from_recursive_root(root: R, _parser: &mut Parser) -> T {
		T::from(root)
	}
}

pub trait TryFromRecursiveRoot<R>: Sized {
	fn try_from_recursive_root(root: R, parser: &mut Parser) -> Option<Self>;
}

impl<R> TryFromRecursiveRoot<R> for R {
	fn try_from_recursive_root(root: R, _parser: &mut Parser) -> Option<Self> {
		Some(root)
	}
}

pub trait IntoRecursiveRoot<R> {
	fn into_recursive_root(self, parser: &mut Parser) -> R;
}


impl<R> IntoRecursiveRoot<R> for R {
	fn into_recursive_root(self, _parser: &mut Parser) -> Self {
		self
	}
}


impl<R> ParsableRecursive<R> for ! {
	type Base = !;
	type Infix = !;
	type Prefix = !;
	type Suffix = !;

	fn join_prefix(prefix: Self::Prefix, _: R, _parser: &mut Parser) -> Self {
		prefix
	}

	fn from_base(base: Self::Base, _parser: &mut Parser) -> Self {
		base
	}

	fn join_suffix(_: R, suffix: Self::Suffix, _parser: &mut Parser) -> Self {
		suffix
	}

	fn join_infix(_: R, infix: Self::Infix, _: R, _parser: &mut Parser) -> Self {
		infix
	}
}

/// Recursive type parser
#[derive(derive_more::Debug)]
pub struct RecursiveWrapper<T, R>(pub T, pub PhantomData<R>);

impl<T, R> ParsableFrom<RecursiveWrapper<T, R>> for T {
	fn from_parsable(wrapper: RecursiveWrapper<T, R>) -> Self {
		wrapper.0
	}
}

// TODO: Instead of parsing with `R` and then converting to `T`, can we
//       somehow parse the "top-level" with `T` and the bottom with `R`?
impl<T, R> crate::Parse for RecursiveWrapper<T, R>
where
	T: TryFromRecursiveRoot<R>,
	R: ParsableRecursive<R>, {
	type Error = RecursiveWrapperError<R>;

	// TODO: Account for precedence
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let convert_inner = |parser: &mut Parser, inner: RecursiveWrapperInnerPart<R>| {
			let mut base = R::from_base(inner.base, parser);
			for prefix in inner.prefixes.into_iter().rev() {
				base = R::join_prefix(prefix, base, parser);
			}
			for suffix in inner.suffixes {
				base = R::join_suffix(base, suffix, parser);
			}

			base
		};

		let inner = self::parse(parser)?;

		let mut base = convert_inner(parser, inner.first);
		for (infix, rhs) in inner.rest {
			base = R::join_infix(base, infix, convert_inner(parser, rhs), parser);
		}

		let base = T::try_from_recursive_root(base, parser)
			.ok_or(Self::Error::FromRoot)?;

		Ok(Self(base, PhantomData))
	}
}

#[derive(derive_more::Debug, ParseError)]
pub enum RecursiveWrapperError<R: ParsableRecursive<R>> {
	#[parse_error(transparent)]
	Prefix(ParserError<R::Prefix>),

	#[parse_error(transparent)]
	Suffix(ParserError<R::Suffix>),

	#[parse_error(transparent)]
	Infix(ParserError<R::Infix>),

	#[parse_error(transparent)]
	Base(ParserError<R::Base>),

	#[parse_error(transparent)]
	#[doc(hidden)]
	BracesOpen(ParserError<ParseBracesOpen>),

	#[parse_error(fmt = "Expected a prefix or base")]
	#[parse_error(multiple)]
	PrefixOrBase {
		prefix: ParserError<R::Prefix>,
		base:   ParserError<R::Base>,
	},

	#[parse_error(fmt = "Expected a suffix or infix")]
	#[parse_error(multiple)]
	SuffixOrInfix {
		suffix: ParserError<R::Suffix>,
		infix:  ParserError<R::Infix>,
	},

	#[parse_error(fmt = "None matched")]
	#[parse_error(multiple)]
	None {
		prefix: ParserError<R::Prefix>,
		suffix: ParserError<R::Suffix>,
		infix:  ParserError<R::Infix>,
		base:   ParserError<R::Base>,
	},

	#[parse_error(fmt = "Unable to convert from root")]
	FromRoot,
}

#[derive(derive_more::Debug)]
struct RecursiveWrapperInner<R: ParsableRecursive<R>> {
	first: RecursiveWrapperInnerPart<R>,
	rest:  Vec<(R::Infix, RecursiveWrapperInnerPart<R>)>,
}

#[derive(derive_more::Debug)]
struct RecursiveWrapperInnerPart<R: ParsableRecursive<R>> {
	prefixes: Vec<R::Prefix>,
	base:     R::Base,
	suffixes: Vec<R::Suffix>,
}

fn parse<R: ParsableRecursive<R>>(parser: &mut Parser) -> Result<RecursiveWrapperInner<R>, RecursiveWrapperError<R>> {
	// Note: We want to ensure that any tags that are active at the beginning
	//       stay active throughout the whole parsing, so we manually set them
	//       on each parse.
	// TODO: This is not a very good solution.
	#[expect(clippy::type_complexity, reason = "TODO")]
	fn peek<T: Parse>(parser: &mut Parser, tags: &[ParserTag],) -> Result<Result<(T, AstPos), ParserError<T>>, ParserError<T>> {
		parser
			.with_tags(tags.iter().copied(), Parser::peek::<T>)
	}
	let tags = parser.tags().collect::<Vec<_>>();

	let mut inners = vec![];

	let mut last_pos_before_infix = None;
	let last_inner = 'last_inner: loop {
		// Parse prefixes followed by a base
		let mut prefixes = vec![];
		let base = loop {
			let prefix = peek::<R::Prefix>(parser, &tags)
				.map_err(RecursiveWrapperError::Prefix)?;
			let base = peek::<R::Base>(parser, &tags)
				.map_err(RecursiveWrapperError::Base)?;
			match (prefix, base) {
				(Ok((prefix, prefix_pos)), Ok((base, base_pos))) => {
					// If we parsed both a valid prefix and base, then
					// look ahead to see if there's another base after
					// the base. If so, then consider this a prefix.
					parser.set_pos(base_pos);
					match peek::<R::Base>(parser, &tags)
						.map_err(RecursiveWrapperError::Base)?
						.is_ok() {
						true => {
							parser.set_pos(prefix_pos);
							prefixes.push(prefix);
						},
						false => break base,
					}
				},

				// If we parsed just one, then use it
				(Ok((prefix, prefix_pos)), Err(_)) => {
					parser.set_pos(prefix_pos);
					prefixes.push(prefix);
				},
				(Err(_), Ok((base, base_pos))) => {
					parser.set_pos(base_pos);
					break base;
				},

				(Err(prefix), Err(base)) => match inners.pop().zip(last_pos_before_infix) {
					// If we didn't parse any, but we had an inner + infix parsed, then just discard
					// the infix and return the last inner we parsed
					Some(((inner, _infix), last_pos_before_infix)) => {
						parser.set_pos(last_pos_before_infix);
						break 'last_inner inner;
					},

					// Otherwise, we're fully empty, so return an error
					None => return Err(
						RecursiveWrapperError::PrefixOrBase { prefix, base }
					),
				},
			}
		};

		// Then parse suffixes followed by an optional infix
		let mut suffixes = vec![];
		let infix = loop {
			last_pos_before_infix = Some(parser.cur_pos());
			let suffix = peek::<R::Suffix>(parser, &tags)
				.map_err(RecursiveWrapperError::Suffix)?;
			let infix = peek::<R::Infix>(parser, &tags)
				.map_err(RecursiveWrapperError::Infix)?;

			match (suffix, infix) {
				(Ok((suffix, suffix_pos)), Ok((infix, infix_pos))) => {
					parser.set_pos(infix_pos);

					// TODO: This is a semi-hack to ensure that we parse `for _ in 0.. {}` correctly.
					//       Technically this should be always correct, since the only suffix that can
					//       be equal to an infix is `..`, and it can't be chained, so the only time
					//       we'd ever find a block expression after `..` would be when it shouldn't
					//       be parsed.
					//       Despite that, this is a very inelegant way to perform this check, and we
					//       should instead just make optional suffixes / base expressions aware of the
					//       `SkipOptionalTrailingBlockExpression` tag and skip themselves or something
					//       similar that doesn't involve us doing anything.
					if tags
						.contains(&ParserTag::SkipOptionalTrailingBlockExpression) && peek::<ParseBracesOpen>(parser, &tags)
						.map_err(RecursiveWrapperError::BracesOpen)?
						.is_ok() {
						parser.set_pos(suffix_pos);
						suffixes.push(suffix);
						continue;
					}

					// If we parsed both a valid suffix and infix, then
					// look ahead to see if there's a prefix/base after the infix.
					// If so, then consider this to be an infix.
					match peek::<R::Prefix>(parser, &tags)
						.map_err(RecursiveWrapperError::Prefix)?
						.is_ok() || peek::<R::Base>(parser, &tags)
						.map_err(RecursiveWrapperError::Base)?
						.is_ok() {
						true => break Some(infix),
						false => {
							parser.set_pos(suffix_pos);
							suffixes.push(suffix);
						},
					}
				},
				(Ok((suffix, suffix_pos)), Err(_)) => {
					parser.set_pos(suffix_pos);
					suffixes.push(suffix);
				},
				(Err(_), Ok((infix, infix_pos))) => {
					parser.set_pos(infix_pos);
					break Some(infix);
				},
				(Err(_), Err(_)) => break None,
			}
		};

		// Finally if we didn't get an infix, we're done.
		let inner = RecursiveWrapperInnerPart { prefixes, base, suffixes, };
		match infix {
			Some(infix) => inners.push((inner, infix)),
			None => break inner,
		}
	};

	let mut inners = inners.into_iter();
	let (first, rest) = match inners.next() {
		Some((first_inner, mut infix)) => {
			let mut rest = vec![];
			for (inner, next_infix) in inners {
				rest.push((infix, inner));
				infix = next_infix;
			}
			rest.push((infix, last_inner));

			(first_inner, rest)
		},
		None => (last_inner, vec![]),
	};

	Ok(RecursiveWrapperInner { first, rest })
}

/// Hack to ensure we don't parse optional trailing block expressions
/// on range expressions.
#[doc(hidden)]
pub struct ParseBracesOpen;

impl Parse for ParseBracesOpen {
	type Error = ();

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		if parser.has_tag(ParserTag::SkipDelimiters) {
			return Err(());
		}

		parser.try_update_with(|s| {
			// TODO: Parse proper whitespace here
			*s = s.trim_start();
			match s.strip_prefix('{') {
				Some(rest) => {
					*s = rest;
					Ok(())
				},
				None => Err(()),
			}
		}).map(|_| Self)
	}
}
