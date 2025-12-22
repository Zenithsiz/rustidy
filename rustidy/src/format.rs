//! Formattable types

// Modules
pub mod config;

// Exports
pub use {self::config::Config, rustidy_macros::Format};

// Imports
use crate::{Parser, ast::whitespace::Whitespace};

/// Formattable type
pub trait Format {
	// TODO: Separate part of these onto a super-trait so some methods can take `&self`

	/// Returns the length of this type
	fn len(&mut self, ctx: &mut Context) -> usize;

	/// Returns if this type is empty
	fn is_empty(&mut self, ctx: &mut Context) -> bool {
		self.len(ctx) == 0
	}

	/// Formats this type
	fn format(&mut self, ctx: &mut Context);

	/// Returns the last whitespace of this type, if any
	fn trailing_ws(&mut self, ctx: &mut Context) -> Option<&mut Whitespace>;

	/// Remove the trailing whitespace, if any
	fn trailing_ws_remove(&mut self, ctx: &mut Context) {
		if let Some(whitespace) = self.trailing_ws(ctx) {
			whitespace.remove(ctx);
		}
	}

	/// Sets the trailing whitespace to a single space
	fn trailing_ws_set_single(&mut self, ctx: &mut Context) {
		if let Some(whitespace) = self.trailing_ws(ctx) {
			whitespace.set_single(ctx);
		}
	}

	/// Sets the trailing whitespace to the current indentation
	fn trailing_ws_set_indent(&mut self, ctx: &mut Context, prev: bool, remove_if_empty: bool) {
		if let Some(whitespace) = self.trailing_ws(ctx) {
			whitespace.set_indent(ctx, prev, remove_if_empty);
		}
	}
}

impl<T: Format> Format for &'_ mut T {
	fn len(&mut self, ctx: &mut Context) -> usize {
		(**self).len(ctx)
	}

	fn format(&mut self, ctx: &mut Context) {
		(**self).format(ctx);
	}

	fn trailing_ws(&mut self, ctx: &mut Context) -> Option<&mut Whitespace> {
		(**self).trailing_ws(ctx)
	}
}

impl<T: Format> Format for Box<T> {
	fn len(&mut self, ctx: &mut Context) -> usize {
		(**self).len(ctx)
	}

	fn format(&mut self, ctx: &mut Context) {
		(**self).format(ctx);
	}

	fn trailing_ws(&mut self, ctx: &mut Context) -> Option<&mut Whitespace> {
		(**self).trailing_ws(ctx)
	}
}

impl<T: Format> Format for Option<T> {
	fn len(&mut self, ctx: &mut Context) -> usize {
		match self {
			Some(value) => value.len(ctx),
			None => 0,
		}
	}

	fn format(&mut self, ctx: &mut Context) {
		if let Some(value) = self {
			value.format(ctx);
		}
	}

	fn trailing_ws(&mut self, ctx: &mut Context) -> Option<&mut Whitespace> {
		self.as_mut().and_then(|value| value.trailing_ws(ctx))
	}
}

impl<T: Format> Format for Vec<T> {
	fn len(&mut self, ctx: &mut Context) -> usize {
		self.iter_mut().map(|value| value.len(ctx)).sum()
	}

	fn format(&mut self, ctx: &mut Context) {
		for value in self {
			value.format(ctx);
		}
	}

	fn trailing_ws(&mut self, ctx: &mut Context) -> Option<&mut Whitespace> {
		self.last_mut().and_then(|value| value.trailing_ws(ctx))
	}
}

impl Format for ! {
	fn len(&mut self, _ctx: &mut Context) -> usize {
		*self
	}

	fn format(&mut self, _ctx: &mut Context) {
		*self
	}

	fn trailing_ws(&mut self, _ctx: &mut Context) -> Option<&mut Whitespace> {
		*self
	}
}

impl Format for () {
	fn len(&mut self, _ctx: &mut Context) -> usize {
		0
	}

	fn format(&mut self, _ctx: &mut Context) {}

	fn trailing_ws(&mut self, _ctx: &mut Context) -> Option<&mut Whitespace> {
		None
	}
}

macro tuple_impl ($N:literal, $($T:ident),* $(,)?) {
	#[derive(Debug, Format)]
	#[expect(non_snake_case)]
	struct ${concat( Tuple, $N )}< $( $T, )* > {
		$( $T: $T, )*
	}

	#[automatically_derived]
	#[expect(non_snake_case)]
	impl< $($T: Format,)* > Format for ( $($T,)* ) {
		fn len(&mut self, ctx: &mut Context) -> usize {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.len(ctx)
		}

		fn format(&mut self, ctx: &mut Context) {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.format(ctx)
		}

		fn trailing_ws(&mut self, ctx: &mut Context) -> Option<&mut Whitespace> {
			let ( $($T,)* ) = self;
			let whitespace = &raw mut *${concat( Tuple, $N )} { $( $T, )* }.trailing_ws(ctx)?;

			// SAFETY: `whitespace` is borrowed from `self`
			Some(unsafe { &mut *whitespace })
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }


/// Format context
pub struct Context<'a, 'input> {
	parser:       &'a Parser<'input>,
	config:       &'a Config,
	indent_depth: usize,
}

impl<'a, 'input> Context<'a, 'input> {
	/// Creates a new context
	#[must_use]
	pub const fn new(parser: &'a Parser<'input>, config: &'a Config) -> Self {
		Self {
			parser,
			config,
			indent_depth: 0,
		}
	}

	/// Returns the parser
	#[must_use]
	pub const fn parser(&self) -> &'a Parser<'input> {
		self.parser
	}

	/// Returns the config
	#[must_use]
	pub const fn config(&self) -> &'a Config {
		self.config
	}

	/// Returns the indentation level
	#[must_use]
	pub const fn indent(&self) -> usize {
		self.indent_depth
	}

	/// Runs `f` with a further indentation level
	pub fn with_indent<O>(&mut self, f: impl FnOnce(&mut Self) -> O) -> O {
		self.indent_depth += 1;
		let output = f(self);
		self.indent_depth -= 1;
		output
	}

	/// Runs `f` with one less indentation level
	pub fn without_indent<O>(&mut self, f: impl for<'b> FnOnce(&'b mut Self) -> O) -> O {
		let prev_depth = self.indent_depth;
		self.indent_depth = prev_depth.saturating_sub(1);
		let output = f(self);
		self.indent_depth = prev_depth;
		output
	}

	/// Runs `f` with one less indentation level if `pred` is true, otherwise
	/// runs it with the current indent
	pub fn without_indent_if<O>(&mut self, pred: bool, f: impl for<'b> FnOnce(&'b mut Self) -> O) -> O {
		match pred {
			true => self.without_indent(f),
			false => f(self),
		}
	}

	#[cfg(test)]
	pub const fn set_indent_depth(&mut self, indent_depth: usize) {
		self.indent_depth = indent_depth;
	}
}

/// A formatting function
pub trait FormatFn<T: ?Sized> = Fn(&mut T, &mut Context);

/// Sets the trailing whitespace to the current indentation
pub fn trailing_ws_set_indent<T: Format>(prev: bool, remove_if_empty: bool) -> impl Fn(&mut T, &mut Context) {
	move |this, ctx| this.trailing_ws_set_indent(ctx, prev, remove_if_empty)
}

/// Formats an `Option<Self>` with `f` if it is `Some`.
pub fn format_option_with<T>(f: impl FormatFn<T>) -> impl FormatFn<Option<T>> {
	move |value, ctx| {
		if let Some(value) = value {
			f(value, ctx);
		}
	}
}

/// Formats a `Vec<Self>` with `f`
pub fn format_vec_each_with<T>(f: impl FormatFn<T>) -> impl FormatFn<Vec<T>> {
	move |values, ctx| {
		for value in values {
			f(value, ctx);
		}
	}
}
