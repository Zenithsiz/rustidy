//! Use statements

// Imports
use {
	crate::ast::{path::SimplePath, token, util::Braced, whitespace::Whitespace},
	rustidy_ast_util::{Identifier, Punctuated, PunctuatedTrailing},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::AstStr,
};

/// `UseDeclaration`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "use declaration")]
pub struct UseDeclaration {
	pub use_: token::Use,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub tree: UseTree,
	#[format(before_with = Format::prefix_ws_remove)]
	pub semi: token::Semi,
}

impl UseDeclaration {
	/// Merges several use declarations into this one
	pub fn merge(&mut self, others: impl IntoIterator<Item = Self, IntoIter: ExactSizeIterator>) {
		let others = others.into_iter();

		// Note: This is to avoid creating an unnecessary group
		if others.is_empty() {
			return;
		}

		replace_with::replace_with_or_abort(&mut self.tree, |tree| {
			let mut group_tree = match tree {
				UseTree::Group(tree) => tree,
				_ => UseTreeGroup {
					prefix: None,
					tree:   Braced {
						prefix: token::BracesOpen(Whitespace::empty(), AstStr::new("{")),
						value:  Some(PunctuatedTrailing {
							punctuated: Punctuated {
								first: Box::new(tree),
								rest:  vec![],
							},
							trailing:   None,
						}),
						suffix: token::BracesClose(Whitespace::empty(), AstStr::new("}")),
					},
				},
			};

			// TODO: We should probably flatten group use declarations here
			//       even if we do a flattening step later on to avoid duplicate work.
			for use_decl in others {
				match &mut group_tree.tree.value {
					Some(inner) => {
						let comma = token::Comma(Whitespace::empty(), AstStr::new(","));
						inner.punctuated.rest.push((comma, Box::new(use_decl.tree)));
					},
					None =>
						group_tree.tree.value = Some(PunctuatedTrailing {
							punctuated: Punctuated {
								first: Box::new(use_decl.tree),
								rest:  vec![],
							},
							trailing:   None,
						}),
				}
			}

			UseTree::Group(group_tree)
		});
	}
}

/// `UseTree`
#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum UseTree {
	Glob(UseTreeGlob),
	Group(UseTreeGroup),
	Simple(UseTreeSimple),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGlob {
	pub prefix: Option<UseTreeGlobPrefix>,
	#[format(before_with(expr = Format::prefix_ws_remove, if = self.prefix.is_some()))]
	pub glob:   token::Star,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGlobPrefix {
	pub path: Option<SimplePath>,
	#[format(before_with(expr = Format::prefix_ws_remove, if = self.path.is_some()))]
	pub sep:  token::PathSep,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGroup {
	pub prefix: Option<UseTreeGroupPrefix>,
	#[format(before_with(expr = Format::prefix_ws_remove, if = self.prefix.is_some()))]
	#[format(indent)]
	#[format(and_with = Self::format_tree)]
	pub tree:   Braced<Option<PunctuatedTrailing<Box<UseTree>, token::Comma>>>,
}

impl UseTreeGroup {
	fn format_tree_compact(
		tree: &mut Braced<Option<PunctuatedTrailing<Box<UseTree>, token::Comma>>>,
		ctx: &mut rustidy_format::Context,
	) {
		if let Some(punct) = &mut tree.value {
			punct.trailing = None;
			punct.format(ctx, Format::prefix_ws_set_single, Format::prefix_ws_remove);
		}
		tree.format_remove(ctx);
	}

	fn format_tree(
		tree: &mut Braced<Option<PunctuatedTrailing<Box<UseTree>, token::Comma>>>,
		ctx: &mut rustidy_format::Context,
	) {
		Self::format_tree_compact(tree, ctx);

		if tree.output_len_without_prefix_ws(ctx) > ctx.config().max_use_tree_len {
			if let Some(punct) = &mut tree.value {
				if punct.trailing.is_none() {
					punct.trailing = Some(token::Comma(Whitespace::empty(), AstStr::new(",")));
				}

				punct.format(ctx, Format::prefix_ws_set_cur_indent, Format::prefix_ws_remove);
			}

			tree.format_indent_if_non_blank(ctx);
		}
	}
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeGroupPrefix {
	pub path: Option<SimplePath>,
	#[format(before_with(expr = Format::prefix_ws_remove, if = self.path.is_some()))]
	pub sep:  token::PathSep,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeSimple {
	pub path: SimplePath,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub as_:  Option<UseTreeSimpleAs>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseTreeSimpleAs {
	pub as_:   token::As,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub value: UseTreeSimpleAsValue,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum UseTreeSimpleAsValue {
	Ident(Identifier),
	Underscore(token::Underscore),
}
