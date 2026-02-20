//! Whitespace impls

// Imports
use {
	crate::{self as rustidy_parse, Parse, ParseError, Parser, ParserError, ParserTag},
	rustidy_util::{
		ArenaIdx,
		whitespace::{
			BlockComment,
			Comment,
			LineComment,
			PureWhitespace,
			Whitespace,
			WhitespaceInner,
		},
	},
};

impl Parse for Whitespace {
	type Error = WhitespaceError;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		if parser.has_tag(ParserTag::SkipWhitespace) {
			let (s, ()) = parser.update_with(|_| ());
			let inner = WhitespaceInner { first: PureWhitespace(s), rest: vec![], };
			let idx = ArenaIdx::new(inner);

			return Ok(Self(idx));
		}

		parser
			.parse::<ArenaIdx<WhitespaceInner>>()
			.map(Self)
			.map_err(WhitespaceError)
	}
}

#[derive(Debug, derive_more::From, ParseError)]
#[parse_error(transparent)]
pub struct WhitespaceError(ParserError<ArenaIdx<WhitespaceInner>>);


impl Parse for WhitespaceInner {
	type Error = WhitespaceInnerError;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let first = parser.parse::<PureWhitespace>()?;
		let mut rest = vec![];
		while let Ok(comment) = parser.try_parse::<Comment>()? {
			let pure = parser.parse::<PureWhitespace>()?;
			rest.push((comment, pure));
		}

		Ok(Self { first, rest })
	}
}

#[derive(Debug, derive_more::From, ParseError)]
pub enum WhitespaceInnerError {
	Pure(ParserError<PureWhitespace>),
	Comment(ParserError<Comment>),
}


impl Parse for Comment {
	type Error = CommentError;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let block_err = match parser.try_parse().map_err(CommentError::Block)? {
			Ok(block) => return Ok(Self::Block(block)),
			Err(err) => err,
		};

		let line_err = match parser.try_parse().map_err(CommentError::Line)? {
			Ok(line) => return Ok(Self::Line(line)),
			Err(err) => err,
		};

		Err(CommentError::None { block: block_err, line: line_err, })
	}
}

#[derive(Debug, ParseError)]
pub enum CommentError {
	#[parse_error(transparent)]
	Block(ParserError<BlockComment>),
	#[parse_error(transparent)]
	Line(ParserError<LineComment>),

	#[parse_error(fmt = "Expected a block or line comment")]
	#[parse_error(multiple)]
	None {
		block: ParserError<BlockComment>,
		line:  ParserError<LineComment>,
	},
}


impl Parse for BlockComment {
	type Error = BlockCommentError;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser
			.try_update_with(|s| {
				let is_doc_comment = (s.starts_with("/**") && !s.starts_with("/***") && !s.starts_with("/**/")) || s.starts_with("/*!");

				match s.strip_prefix("/*") {
					Some(rest) if !is_doc_comment => {
						*s = rest;
						let mut depth = 1;
						while depth != 0 {
							let close_idx = s
								.find("*/")
								.ok_or(BlockCommentError::MissingCommentEnd)?;

							match s[..close_idx].find("/*") {
								Some(open_idx) => {
									*s = &s[open_idx + 2..];
									depth += 1;
								},
								None => {
									*s = &s[close_idx + 2..];
									depth -= 1;
								},
							}
						}
						Ok(())
					},
					_ => Err(BlockCommentError::NoComment),
				}
			})
			.map(|(s, ())| Self(s))
	}
}

#[derive(Debug, ParseError)]
pub enum BlockCommentError {
	#[parse_error(fmt = "Expected `/*` (except `/*!` or `/**`)")]
	NoComment,
	#[parse_error(fmt = "Expected `*/` after `/*`")]
	#[parse_error(fatal)]
	MissingCommentEnd,
}


impl Parse for LineComment {
	type Error = LineCommentError;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser
			.try_update_with(|s| {
				let is_doc_comment = (s.starts_with("///") && !s.starts_with("////")) || s.starts_with("//!");
				match s.starts_with("//") && !is_doc_comment {
					true => {
						let nl_idx = s.find('\n').ok_or(LineCommentError::Newline)?;
						*s = &s[nl_idx + 1..];
						Ok(())
					},
					false => Err(LineCommentError::NoComment),
				}
			})
			.map(|(s, ())| Self(s))
	}
}

#[derive(Debug, ParseError)]
pub enum LineCommentError {
	#[parse_error(fmt = "Expected `//` (except `///` or `//!`)")]
	NoComment,
	#[parse_error(fmt = "Expected newline after `//`")]
	Newline,
}


impl Parse for PureWhitespace {
	type Error = !;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let (s, ()) = parser.update_with(|s| *s = s.trim_start());
		Ok(Self(s))
	}
}
