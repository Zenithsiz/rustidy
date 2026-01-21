//! Delimited

// Imports
use {rustidy_format::Format, rustidy_parse::Parse, rustidy_print::Print};

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
	/// Formats this delimited with a single space if non-blank, otherwise removes
	pub fn format_single_if_non_blank(&mut self, ctx: &mut rustidy_format::Context)
	where
		T: Format,
		R: Format,
	{
		match self.value.is_blank(ctx) {
			true => {
				self.value.prefix_ws_set_single(ctx);
				self.suffix.prefix_ws_remove(ctx);
			},
			false => {
				self.value.prefix_ws_set_single(ctx);
				self.suffix.prefix_ws_set_single(ctx);
			},
		}
	}

	/// Formats this delimited by indenting if non-blank, otherwise removing
	pub fn format_indent_if_non_blank(&mut self, ctx: &mut rustidy_format::Context)
	where
		T: Format,
		R: Format,
	{
		match self.value.is_blank(ctx) {
			true => {
				self.value.prefix_ws_remove(ctx);
				self.suffix.prefix_ws_set_indent(ctx, -1, true);
			},
			false => {
				self.value.prefix_ws_set_indent(ctx, 0, false);
				self.suffix.prefix_ws_set_indent(ctx, -1, false);
			},
		}
	}

	/// Formats this delimited by removing all inner whitespace
	pub fn format_remove(&mut self, ctx: &mut rustidy_format::Context)
	where
		T: Format,
		R: Format,
	{
		self.value.prefix_ws_remove(ctx);
		self.suffix.prefix_ws_remove(ctx);
	}
}
