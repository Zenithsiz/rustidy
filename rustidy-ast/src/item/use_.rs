//! Use statements

// Imports
use {
	crate::{path::SimplePath, token, util::Braced},
	rustidy_ast_util::{Identifier, Punctuated, PunctuatedTrailing},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
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
			let mut group_tree = tree.into_group();
			for use_decl in others {
				// TODO: Here we're discarding the whitespace after `use` and before `;`,
				//       we should probably instead return an error saying we couldn't merge.
				group_tree.push(use_decl.tree);
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

impl UseTree {
	/// Converts this tree into a group.
	#[must_use]
	pub fn into_group(self) -> UseTreeGroup {
		match self {
			Self::Group(tree) => tree,

			_ => UseTreeGroup {
				prefix: None,
				tree:   Braced::from_value(Some(PunctuatedTrailing::single(Box::new(self)))),
			},
		}
	}
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
#[format(before_with = Self::flatten)]
pub struct UseTreeGroup {
	pub prefix: Option<UseTreeGroupPrefix>,
	#[format(before_with(expr = Format::prefix_ws_remove, if = self.prefix.is_some()))]
	#[format(indent)]
	#[format(and_with = Self::format_tree)]
	pub tree:   Braced<Option<PunctuatedTrailing<Box<UseTree>, token::Comma>>>,
}

impl UseTreeGroup {
	/// Pushes a tree into this group
	pub fn push(&mut self, tree: UseTree) {
		// TODO: We should probably flatten group use declarations here to avoid duplicate work.
		match &mut self.tree.value {
			Some(inner) => inner.push_value(Box::new(tree)),
			None => self.tree.value = Some(PunctuatedTrailing::single(Box::new(tree))),
		}
	}

	pub fn flatten(&mut self, ctx: &mut rustidy_format::Context) {
		replace_with::replace_with_or_abort(&mut self.tree.value, |trees| {
			let mut trees = trees?;
			let mut trees_first = Some((token::Comma::new(), trees.punctuated.first));
			let mut sub_trees = vec![];
			let mut new_trees: Vec<(token::Comma, _)> = vec![];
			let mut trailing_comma = trees.trailing;

			// Note: We process the trees backwards to ensure that we always have
			//       somewhere to add the whitespace of the braces we're removing.
			while let Some((mut comma, tree)) = sub_trees
				.pop()
				.or_else(|| trees.punctuated.rest.pop())
				.or_else(|| trees_first.take())
			{
				// Joins a prefix whitespace to the latest whitespace we have
				let mut latest_ws_join_prefix = |ws: Whitespace| match new_trees.last_mut() {
					Some((last_comma, _)) => last_comma.ws.join_prefix(ws),
					None => match &mut trailing_comma {
						Some(trailing_comma) => trailing_comma.ws.join_prefix(ws),
						None => self.tree.suffix.ws.join_prefix(ws),
					},
				};

				match *tree {
					UseTree::Group(group) if group.prefix.is_none() => {
						latest_ws_join_prefix(group.tree.suffix.ws);

						match group.tree.value {
							Some(trees) => {
								comma.ws.join_prefix(group.tree.prefix.ws);
								sub_trees.push((comma, trees.punctuated.first));
								for (comma, tree) in trees.punctuated.rest {
									sub_trees.push((comma, tree));
								}
							},
							None => latest_ws_join_prefix(group.tree.prefix.ws),
						}
					},
					_ => new_trees.push((comma, tree)),
				}
			}

			new_trees.pop().map(|(first_comma, mut first)| {
				first
					.prefix_ws_join_prefix(ctx, first_comma.ws)
					.expect("Should have prefix whitespace");

				new_trees.reverse();
				PunctuatedTrailing {
					punctuated: Punctuated { first, rest: new_trees },
					trailing:   trailing_comma,
				}
			})
		});
	}

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

		if tree.len(ctx, true) > ctx.config().max_use_tree_len {
			if let Some(punct) = &mut tree.value {
				if punct.trailing.is_none() {
					punct.trailing = Some(token::Comma::new());
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
