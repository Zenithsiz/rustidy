//! Use statements

// Imports
use {
	crate::{path::{SimplePath, SimplePathSegment}, token, util::Braced},
	rustidy_ast_literal::Identifier,
	rustidy_ast_util::{Punctuated, PunctuatedTrailing, delimited, punct::{self, PunctuatedRest}},
	rustidy_format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
	std::{borrow::Cow, cmp},
};

/// `UseDeclaration`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "use declaration")]
pub struct UseDeclaration {
	pub use_: token::Use,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub tree: UseTree,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi: token::Semi,
}

impl UseDeclaration {
	/// Merges another use declaration into this one
	pub fn merge(&mut self, other: Self) {
		self.tree.merge(other.tree);
	}
}

/// `UseTree`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
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
				tree: Braced::from_value(Some(PunctuatedTrailing::single(Box::new(self)))),
			},
		}
	}

	/// Merges another tree into this one
	pub fn merge(&mut self, other: Self) {
		match (self, other) {
			(Self::Group(lhs), rhs) => lhs.push_back(rhs),
			(lhs, Self::Group(mut rhs)) => replace_with::replace_with_or_abort(lhs, |lhs| {
				rhs.push_front(lhs);
				Self::Group(rhs)
			}),

			(lhs, rhs) => replace_with::replace_with_or_abort(lhs, |lhs| {
				let mut values = PunctuatedTrailing::single(Box::new(lhs));
				values.push_value(Box::new(rhs));
				Self::Group(UseTreeGroup {
					prefix: None,
					tree: Braced::from_value(Some(values))
				})
			}),
		}
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct UseTreeGlob {
	pub prefix: Option<UseTreeGlobPrefix>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if_ = self.prefix.is_some()))]
	pub glob:   token::Star,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct UseTreeGlobPrefix {
	pub path: Option<SimplePath>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if_ = self.path.is_some()))]
	pub sep:  token::PathSep,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(before_with = Self::flatten)]
#[format(before_with = Self::sort)]
pub struct UseTreeGroup {
	pub prefix: Option<UseTreeGroupPrefix>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if_ = self.prefix.is_some()))]
	#[format(with = Self::format_tree)]
	pub tree:   Braced<Option<PunctuatedTrailing<Box<UseTree>, token::Comma>>>,
}

impl UseTreeGroup {
	/// Pushes a tree at the front of this group
	pub fn push_front(&mut self, tree: UseTree) {
		match tree {
			UseTree::Group(rhs) if self.prefix == rhs.prefix => match &mut self.tree.value {
				Some(lhs) => if let Some(mut rhs) = rhs.tree.value {
					replace_with::replace_with_or_abort(lhs, |lhs| {
						rhs.extend_from_punctuated_trailing(lhs);
						rhs
					});
				},
				None => self.tree.value = rhs.tree.value,
			},

			UseTree::Group(rhs) => replace_with::replace_with_or_abort(self, |lhs| {
				let mut values = PunctuatedTrailing::single(Box::new(UseTree::Group(rhs)));
				values
					.push_value(Box::new(UseTree::Group(lhs)));
				Self {
					prefix: None,
					tree: Braced::from_value(Some(values))
				}
			}),

			_ => match &mut self.tree.value {
				Some(lhs) => lhs.push_front_value(Box::new(tree)),
				None => self.tree.value = Some(PunctuatedTrailing::single(Box::new(tree))),
			},
		}
	}

	/// Pushes a tree at the back of this group
	pub fn push_back(&mut self, tree: UseTree) {
		match tree {
			UseTree::Group(rhs) if self.prefix == rhs.prefix => match &mut self.tree.value {
				Some(lhs) => if let Some(rhs) = rhs.tree.value {
					lhs.extend_from_punctuated_trailing(rhs);
				},
				None => self.tree.value = rhs.tree.value,
			},

			UseTree::Group(rhs) => replace_with::replace_with_or_abort(self, |lhs| {
				let mut values = PunctuatedTrailing::single(Box::new(UseTree::Group(lhs)));
				values
					.push_value(Box::new(UseTree::Group(rhs)));
				Self {
					prefix: None,
					tree: Braced::from_value(Some(values))
				}
			}),

			_ => match &mut self.tree.value {
				Some(lhs) => lhs.push_value(Box::new(tree)),
				None => self.tree.value = Some(PunctuatedTrailing::single(Box::new(tree))),
			},
		}
	}

	/// Sorts all trees inside
	pub fn sort(&mut self, _ctx: &mut rustidy_format::Context) {
		let Some(trees) = &mut self.tree.value else { return };

		// TODO: Move this wrapper elsewhere
		struct SimplePathSortOrder<'a>(&'a SimplePath);

		impl PartialEq for SimplePathSortOrder<'_> {
			fn eq(&self, other: &Self) -> bool {
				self.cmp(other).is_eq()
			}
		}
		impl Eq for SimplePathSortOrder<'_> {}
		impl PartialOrd for SimplePathSortOrder<'_> {
			fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
				Some(self.cmp(other))
			}
		}
		impl Ord for SimplePathSortOrder<'_> {
			fn cmp(&self, other: &Self) -> cmp::Ordering {
				let prefix_cmp = self.0
					.prefix
					.is_some()
					.cmp(&other.0.prefix.is_some());
				if !prefix_cmp.is_eq() {
					return prefix_cmp;
				}

				let mut lhs = self.0.segments.values();
				let mut rhs = other.0.segments.values();
				loop {
					let (lhs, rhs) = match (lhs.next(), rhs.next()) {
						(Some(lhs), Some(rhs)) => (lhs, rhs),
						(Some(_), None) => return cmp::Ordering::Less,
						(None, Some(_)) => return cmp::Ordering::Greater,
						(None, None) => return cmp::Ordering::Equal,
					};

					#[derive(PartialEq, Eq, PartialOrd, Ord)]
					pub enum Segment<'a> {
						Crate,
						Super,
						SelfLower,
						DollarCrate,
						Ident(Cow<'a, str>),
					}
					fn segment(segment: &SimplePathSegment) -> Segment<'_> {
						match segment {
							SimplePathSegment::Super(_) => Segment::Super,
							SimplePathSegment::SelfLower(_) => Segment::SelfLower,
							SimplePathSegment::Crate(_) => Segment::Crate,
							SimplePathSegment::DollarCrate(_) => Segment::DollarCrate,
							SimplePathSegment::Ident(ident) => Segment::Ident(ident.as_str()),
						}
					}

					let segment_cmp = segment(lhs).cmp(&segment(rhs));
					if !segment_cmp.is_eq() {
						return segment_cmp;
					}
				}
			}
		}

		#[derive(PartialEq, Eq, PartialOrd, Ord)]
		enum SortOrder<'a> {
			GroupWithPrefixRoot,
			Glob,
			WithPath(SimplePathSortOrder<'a>),
			GroupNoPrefix,
		}

		#[expect(clippy::borrowed_box, reason = "It's necessary for the closure")]
		trees.sort_values_by_key(
			for<'a> |tree: &'a Box<UseTree>, _: Option<&'a token::Comma>| -> SortOrder<'a> {
				match &**tree {
					UseTree::Glob(_) => SortOrder::Glob,
					UseTree::Group(tree) => match &tree.prefix {
						Some(prefix) => match &prefix.path {
							Some(path) => SortOrder::WithPath(SimplePathSortOrder(path)),
							None => SortOrder::GroupWithPrefixRoot,
						},
						None => SortOrder::GroupNoPrefix,
					},
					UseTree::Simple(tree) => SortOrder::WithPath(SimplePathSortOrder(&tree.path)),
				}
			}
		);
	}

	/// Flattens this use group
	pub fn flatten(&mut self, ctx: &mut rustidy_format::Context) {
		replace_with::replace_with_or_abort(&mut self.tree.value, |trees| {
			let mut trees = trees?;
			let mut trees_first = Some(PunctuatedRest {
				punct: token::Comma::new(),
				value: trees.punctuated.first,
			});
			let mut sub_trees = vec![];
			let mut new_trees: Vec<PunctuatedRest<_, token::Comma>> = vec![];
			let mut trailing_comma = trees.trailing;

			// Note: We process the trees backwards to ensure that we always have
			//       somewhere to add the whitespace of the braces we're removing.
			while let Some(PunctuatedRest { punct: mut comma, value: tree, }) = sub_trees
				.pop()
				.or_else(|| trees.punctuated.rest.pop())
				.or_else(|| trees_first.take()) {
				// Joins a prefix whitespace to the latest whitespace we have
				let mut latest_ws_join_prefix = |ws: Whitespace| match new_trees.last_mut() {
					Some(PunctuatedRest { punct: last_comma, .. }) => last_comma.ws.join_prefix(ws),
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
								comma
									.ws
									.join_prefix(group.tree.prefix.ws);
								sub_trees.push(
									PunctuatedRest { punct: comma, value: trees.punctuated.first, }
								);
								for rest in trees.punctuated.rest {
									sub_trees.push(rest);
								}
							},
							None => latest_ws_join_prefix(group.tree.prefix.ws),
						}
					},
					_ => new_trees
						.push(PunctuatedRest { punct: comma, value: tree, }),
				}
			}

			new_trees.pop().map(
				|PunctuatedRest { punct: first_comma, value: mut first }| {
					first
						.prefix_ws_join_prefix(ctx, first_comma.ws)
						.expect("Use tree should have prefix whitespace");

					new_trees.reverse();
					PunctuatedTrailing {
						punctuated: Punctuated { first, rest: new_trees },
						trailing: trailing_comma,
					}
				},
			)
		});
	}

	fn format_tree_compact(
		tree: &mut Braced<Option<PunctuatedTrailing<Box<UseTree>, token::Comma>>>,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
	) -> FormatOutput {
		if let Some(punct) = &mut tree.value {
			punct.trailing = None;
		}

		tree.format(
			ctx,
			prefix_ws,
			delimited::FmtRemoveWith(punct::FmtArgs {
				value_prefix_ws: Whitespace::SINGLE,
				punct_prefix_ws: Whitespace::REMOVE,
				value_args: (),
				punct_args: (),
			}),
		)
	}

	fn format_tree(
		tree: &mut Braced<Option<PunctuatedTrailing<Box<UseTree>, token::Comma>>>,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		_args: (),
	) -> FormatOutput {
		let compact_output = Self::format_tree_compact(tree, ctx, prefix_ws);

		// Note: We don't use `len_non_multiline_ws` because we never want to emit
		//       something like `{a, b::{\n...\n}, c, d}`, and if any newlines are
		//       found we'd instead want to make it multi-line.
		match compact_output.len_without_prefix_ws() > ctx.config().max_use_tree_len {
			true => {
				if let Some(punct) = &mut tree.value && punct.trailing.is_none() {
					punct.trailing = Some(token::Comma::new());
				}

				tree.format(
					ctx,
					prefix_ws,
					delimited::fmt_indent_if_non_blank_with((), punct::FmtArgs {
						value_prefix_ws: Whitespace::INDENT,
						punct_prefix_ws: Whitespace::REMOVE,
						value_args: (),
						punct_args: (),
					}, (),),
				)
			},
			false => compact_output,
		}
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct UseTreeGroupPrefix {
	pub path: Option<SimplePath>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if_ = self.path.is_some()))]
	pub sep:  token::PathSep,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct UseTreeSimple {
	pub path: SimplePath,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub as_:  Option<UseTreeSimpleAs>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct UseTreeSimpleAs {
	pub as_:   token::As,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub value: UseTreeSimpleAsValue,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum UseTreeSimpleAsValue {
	Ident(Identifier),
	Underscore(token::Underscore),
}
