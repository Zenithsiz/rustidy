//! Use statements

// Imports
use crate::{
	Format,
	ast::{delimited::Braced, ident::Identifier, path::SimplePath, punct::PunctuatedTrailing, token},
	parser::Parse,
	print::Print,
};

/// `UseDeclaration`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "use declaration")]
pub struct UseDeclaration {
	pub use_: token::Use,
	#[parse(fatal)]
	pub tree: UseTree,
	pub semi: token::Semi,
}

/// `UseTree`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum UseTree {
	Glob(UseTreeGlob),
	Group(UseTreeGroup),
	Simple(UseTreeSimple),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGlob {
	pub prefix: Option<UseTreeGlobPrefix>,
	pub glob:   token::Star,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGlobPrefix {
	pub path: Option<SimplePath>,
	pub sep:  token::PathSep,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGroup {
	pub prefix: Option<UseTreeGroupPrefix>,
	pub tree:   Braced<Option<PunctuatedTrailing<Box<UseTree>, token::Comma>>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGroupPrefix {
	pub path: Option<SimplePath>,
	pub sep:  token::PathSep,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeSimple {
	pub path: SimplePath,
	pub as_:  Option<UseTreeSimpleAs>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeSimpleAs {
	pub as_:   token::As,
	#[parse(fatal)]
	pub value: UseTreeSimpleAsValue,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum UseTreeSimpleAsValue {
	Ident(Identifier),
	Underscore(token::Underscore),
}
