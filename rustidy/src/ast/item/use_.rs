//! Use statements

// Imports
use {
	crate::ast::{path::SimplePath, token, util::Braced, whitespace::Whitespace},
	rustidy_ast_util::{Identifier, PunctuatedTrailing},
	rustidy_format::{Format, FormatRef},
	rustidy_parse::Parse,
	rustidy_print::Print,
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

		// TODO: This should include the whitespace on the current line
		if tree.output_len(ctx) > ctx.config().max_use_tree_len {
			if let Some(punct) = &mut tree.value {
				let pos = punct.input_range(ctx).expect("Should have a range").end;
				if punct.trailing.is_none() {
					let ws = Whitespace::empty(pos);
					let comma = ctx.create_str_at_pos_with_replacement(pos, ",");
					punct.trailing = Some(token::Comma(ws, comma));
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
