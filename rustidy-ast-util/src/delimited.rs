//! Delimited

// Imports
use {
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// A value `T` delimited by prefix `L` and suffix `R`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Delimited<T, L, R> {
	pub prefix: L,
	// TODO: Should we always remove all tags here?
	#[parse(without_tags)]
	pub value:  T,
	pub suffix: R,
}

impl<T, L, R> Delimited<T, L, R> {
	/// Creates a new delimited from it's inner value
	pub fn from_value(value: T) -> Self
	where
		L: Default,
		R: Default,
	{
		Self {
			prefix: L::default(),
			value,
			suffix: R::default(),
		}
	}

	/// Formats this delimited with a single space if non-blank, otherwise removes
	pub fn format_single_if_non_blank(&mut self, ctx: &mut rustidy_format::Context)
	where
		T: Format,
		R: Format,
	{
		match self.value.is_blank(ctx, false) {
			true => {
				self.value.format(ctx, &mut Whitespace::set_single);
				self.suffix.format(ctx, &mut Whitespace::remove);
			},
			false => {
				self.value.format(ctx, &mut Whitespace::set_single);
				self.suffix.format(ctx, &mut Whitespace::set_single);
			},
		}
	}

	/// Formats this delimited by indenting if non-blank, otherwise removing
	pub fn format_indent_if_non_blank(&mut self, ctx: &mut rustidy_format::Context)
	where
		T: Format,
		R: Format,
	{
		match self.value.is_blank(ctx, false) {
			true => {
				self.value.format(ctx, &mut Whitespace::remove);
				self.suffix.format(ctx, &mut Whitespace::set_prev_indent_remove_if_pure);
			},
			false => {
				self.value.format(ctx, &mut Whitespace::set_cur_indent);
				self.suffix.format(ctx, &mut Whitespace::set_prev_indent);
			},
		}
	}

	/// Formats this delimited by removing all inner whitespace
	pub fn format_remove(&mut self, ctx: &mut rustidy_format::Context)
	where
		T: Format,
		R: Format,
	{
		self.value.format(ctx, &mut Whitespace::remove);
		self.suffix.format(ctx, &mut Whitespace::remove);
	}
}
