//! Recursively parsable types

// Exports
pub use rustidy_macros::ParseRecursive;

// Imports
use {
	super::{ParsableFrom, Parse, ParseError, Parser, ParserError},
	crate::ast::punct::Punctuated,
	core::{marker::PhantomData, mem},
	either::Either,
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

	/// Converts this type into the root
	fn into_root(self) -> R;

	/// Creates this type from it's parts
	fn join_prefix(prefix: Self::Prefix, root: R) -> Self;

	/// Converts the base to this type
	fn from_base(base: Self::Base) -> Self;

	/// Creates this type from it's parts
	fn join_suffix(root: R, suffix: Self::Suffix) -> Self;

	/// Creates this type from it's parts
	fn join_infix(lhs: R, infix: Self::Infix, rhs: R) -> Self;
}

impl<R> ParsableRecursive<R> for ! {
	type Base = !;
	type Infix = !;
	type Prefix = !;
	type Suffix = !;

	fn into_root(self) -> R {
		self
	}

	fn join_prefix(prefix: Self::Prefix, _: R) -> Self {
		prefix
	}

	fn from_base(base: Self::Base) -> Self {
		base
	}

	fn join_suffix(_: R, suffix: Self::Suffix) -> Self {
		suffix
	}

	fn join_infix(_: R, infix: Self::Infix, _: R) -> Self {
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
	T: ParsableRecursive<R> + TryFrom<R>,
	R: ParsableRecursive<R>,
{
	type Error = RecursiveWrapperError<R>;

	#[coverage(off)]
	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	// TODO: Account for precedence
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let convert_inner = |inner: RecursiveWrapperInner<R>| {
			let mut base = R::from_base(inner.base);
			for prefix in inner.prefixes.into_iter().rev() {
				base = R::join_prefix(prefix, R::into_root(base));
			}
			for suffix in inner.suffixes {
				base = R::join_suffix(R::into_root(base), suffix);
			}

			base
		};

		let raw = self::parse(parser)?;

		let mut base = convert_inner(raw.first);
		for (infix, rhs) in raw.rest {
			base = R::join_infix(R::into_root(base), infix, R::into_root(convert_inner(rhs)));
		}

		let base = T::try_from(base).map_err(|_| Self::Error::FromRoot)?;

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
	prefixes: Vec<R::Prefix>,
	base:     R::Base,
	suffixes: Vec<R::Suffix>,
}

// Todo: Not clone parser states here
fn parse<R: ParsableRecursive<R>>(
	parser: &mut Parser,
) -> Result<Punctuated<RecursiveWrapperInner<R>, R::Infix>, RecursiveWrapperError<R>> {
	let mut inners = vec![];

	let mut cur_prefixes = vec![];
	let last_inner = loop {
		let prefix = parser.peek::<R::Prefix>().map_err(RecursiveWrapperError::Prefix)?;
		let base = parser.peek::<R::Base>().map_err(RecursiveWrapperError::Base)?;

		let parsed = match (prefix, base) {
			(Ok((prefix, prefix_state)), Ok((base, base_state))) => {
				parser.set_peeked(base_state.clone());
				match parser.peek::<R::Base>().map_err(RecursiveWrapperError::Base)?.is_ok() {
					true => Either::Left((prefix, prefix_state)),
					false => Either::Right((base, base_state)),
				}
			},
			(Ok((prefix, prefix_state)), Err(_)) => Either::Left((prefix, prefix_state)),
			(Err(_), Ok((base, base_state))) => Either::Right((base, base_state)),
			(Err(prefix), Err(base)) => return Err(RecursiveWrapperError::PrefixOrBase { prefix, base }),
		};

		match parsed {
			Either::Left((prefix, prefix_state)) => {
				parser.set_peeked(prefix_state);
				cur_prefixes.push(prefix);
			},
			Either::Right((base, base_state)) => {
				parser.set_peeked(base_state);

				let mut cur_suffixes = vec![];
				let infix = loop {
					let suffix = parser.peek::<R::Suffix>().map_err(RecursiveWrapperError::Suffix)?;
					let infix = parser.peek::<R::Infix>().map_err(RecursiveWrapperError::Infix)?;

					let parsed = match (suffix, infix) {
						(Ok((suffix, suffix_state)), Ok((infix, infix_state))) => {
							parser.set_peeked(infix_state.clone());
							match parser
								.peek::<R::Prefix>()
								.map_err(RecursiveWrapperError::Prefix)?
								.is_ok() || parser.peek::<R::Base>().map_err(RecursiveWrapperError::Base)?.is_ok()
							{
								true => Either::Right((infix, infix_state)),
								false => Either::Left((suffix, suffix_state)),
							}
						},
						(Ok((suffix, suffix_state)), Err(_)) => Either::Left((suffix, suffix_state)),
						(Err(_), Ok((infix, infix_state))) => Either::Right((infix, infix_state)),
						(Err(_), Err(_)) => break None,
					};

					match parsed {
						Either::Left((suffix, suffix_state)) => {
							parser.set_peeked(suffix_state);
							cur_suffixes.push(suffix);
						},
						Either::Right((infix, infix_state)) => {
							parser.set_peeked(infix_state);
							break Some(infix);
						},
					}
				};

				let inner = RecursiveWrapperInner {
					prefixes: mem::take(&mut cur_prefixes),
					base,
					suffixes: cur_suffixes,
				};

				match infix {
					Some(infix) => inners.push((inner, infix)),
					None => break inner,
				}
			},
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

	Ok(Punctuated { first, rest })
}
