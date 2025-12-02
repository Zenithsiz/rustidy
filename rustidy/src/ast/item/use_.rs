//! Use statements

// Imports
use crate::{
	Format,
	ast::{ident::Ident, path::SimplePath, punct::PunctuatedTrailing, token},
	parser::Parse,
	print::Print,
};

/// `UseDeclaration`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "use declaration")]
pub struct UseDeclaration {
	use_: token::Use,
	#[parse(fatal)]
	tree: UseTree,
	semi: token::Semi,
}

/// `UseTree`
#[derive(PartialEq, Eq, Clone, Debug)]
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
	prefix: Option<UseTreeGlobPrefix>,
	glob:   token::Star,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGlobPrefix {
	path: Option<SimplePath>,
	sep:  token::PathSep,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGroup {
	prefix: Option<UseTreeGroupPrefix>,
	open:   token::BracesOpen,
	#[parse(fatal)]
	tree:   Option<PunctuatedTrailing<Box<UseTree>, token::Comma>>,
	close:  token::BracesClose,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGroupPrefix {
	path: Option<SimplePath>,
	sep:  token::PathSep,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeSimple {
	path: SimplePath,
	as_:  Option<UseTreeSimpleAs>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeSimpleAs {
	as_:   token::As,
	#[parse(fatal)]
	value: UseTreeSimpleAsValue,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum UseTreeSimpleAsValue {
	Ident(Ident),
	Underscore(token::Underscore),
}
