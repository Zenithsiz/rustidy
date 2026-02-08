//! Formatting

// Features
#![feature(
	decl_macro,
	never_type,
	coverage_attribute,
	macro_metavar_expr_concat,
	trait_alias,
	iter_advance_by
)]

// Modules
#[doc(hidden)]
pub mod whitespace;

// Exports
pub use {self::whitespace::WhitespaceFormat, rustidy_macros::Format};

// Imports
use {
	crate as rustidy_format,
	core::{marker::PhantomData, ops::ControlFlow},
	rustidy_util::{ArenaData, ArenaIdx, AstStr, Config, Whitespace},
	std::borrow::Cow,
};

/// Formattable type
pub trait Format {
	/// Iterates over all strings in this type
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O>;

	/// Returns the length of this type
	fn len(&mut self, ctx: &mut Context) -> usize {
		let mut len = 0;
		self.with_strings::<!>(ctx, &mut |s, _ctx| {
			len += AstStr::len(s);
			ControlFlow::Continue(())
		});
		len
	}

	/// Returns if this type is blank
	fn is_blank(&mut self, ctx: &mut Context) -> bool {
		self.with_strings(ctx, &mut |s, ctx| match AstStr::is_blank(s, ctx.input) {
			true => ControlFlow::Continue(()),
			false => ControlFlow::Break(()),
		})
		.is_continue()
	}

	/// Returns the length of this type without the prefix whitespace
	fn len_without_prefix_ws(&mut self, ctx: &mut Context) -> usize {
		let mut len = self.len(ctx);
		self.with_prefix_ws(ctx, &mut |ws, ctx| len -= ws.len(ctx));
		len
	}

	/// Formats this type
	fn format(&mut self, ctx: &mut Context);

	/// Uses the first whitespace of this type, if any.
	///
	/// Returns if successfully used.
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Option<O>;

	/// Returns if the prefix whitespace is pure
	fn prefix_ws_is_pure(&mut self, ctx: &mut Context) -> bool {
		self.with_prefix_ws(ctx, &mut |ws, ctx| ws.is_pure(ctx))
			.unwrap_or(false)
	}

	/// Remove the prefix whitespace, if any
	fn prefix_ws_remove(&mut self, ctx: &mut Context) {
		self.with_prefix_ws(ctx, &mut |ws, ctx| ws.remove(ctx));
	}

	/// Sets the prefix whitespace to spaces
	fn prefix_ws_set_spaces(&mut self, ctx: &mut Context, len: usize) {
		self.with_prefix_ws(ctx, &mut |ws, ctx| ws.set_spaces(ctx, len));
	}

	/// Sets the prefix whitespace to a single space
	fn prefix_ws_set_single(&mut self, ctx: &mut Context) {
		self.prefix_ws_set_spaces(ctx, 1);
	}

	/// Sets the prefix whitespace to an indentation
	fn prefix_ws_set_indent(&mut self, ctx: &mut Context, offset: isize, remove_if_empty: bool) {
		self.with_prefix_ws(ctx, &mut |ws, ctx| ws.set_indent(ctx, offset, remove_if_empty));
	}

	/// Sets the prefix whitespace to the current indentation without removing if empty
	fn prefix_ws_set_cur_indent(&mut self, ctx: &mut Context) {
		self.prefix_ws_set_indent(ctx, 0, false);
	}

	/// Joins a whitespace into this type's prefix whitespace as a suffix.
	fn prefix_ws_join_suffix(&mut self, ctx: &mut Context, ws: Whitespace) -> Result<(), Whitespace> {
		let mut new_ws = Some(ws);

		self.with_prefix_ws(ctx, &mut |ws, _ctx| {
			ws.join_suffix(new_ws.take().expect("Should exist"));
		});

		match new_ws {
			Some(ws) => Err(ws),
			None => Ok(()),
		}
	}

	/// Joins a whitespace into this type's prefix whitespace as a prefix.
	fn prefix_ws_join_prefix(&mut self, ctx: &mut Context, ws: Whitespace) -> Result<(), Whitespace> {
		let mut new_ws = Some(ws);

		self.with_prefix_ws(ctx, &mut |ws, _ctx| {
			ws.join_prefix(new_ws.take().expect("Should exist"));
		});

		match new_ws {
			Some(ws) => Err(ws),
			None => Ok(()),
		}
	}
}

impl<T: Format> Format for &'_ mut T {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		(**self).with_strings(ctx, f)
	}

	fn format(&mut self, ctx: &mut Context) {
		(**self).format(ctx);
	}

	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Option<O> {
		(**self).with_prefix_ws(ctx, f)
	}
}

impl<T: Format> Format for Box<T> {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		(**self).with_strings(ctx, f)
	}

	fn format(&mut self, ctx: &mut Context) {
		(**self).format(ctx);
	}

	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Option<O> {
		(**self).with_prefix_ws(ctx, f)
	}
}

impl<T: Format> Format for Option<T> {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		match self {
			Some(value) => value.with_strings(ctx, f),
			None => ControlFlow::Continue(()),
		}
	}

	fn format(&mut self, ctx: &mut Context) {
		if let Some(value) = self {
			value.format(ctx);
		}
	}

	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Option<O> {
		match self {
			Some(value) => value.with_prefix_ws(ctx, f),
			_ => None,
		}
	}
}

impl<T: Format> Format for Vec<T> {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		for value in self {
			value.with_strings(ctx, f)?;
		}

		ControlFlow::Continue(())
	}

	fn format(&mut self, ctx: &mut Context) {
		for value in self {
			value.format(ctx);
		}
	}

	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Option<O> {
		match self.first_mut() {
			Some(value) => value.with_prefix_ws(ctx, f),
			None => None,
		}
	}
}

impl Format for ! {
	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		*self
	}

	fn format(&mut self, _ctx: &mut Context) {
		*self
	}

	fn with_prefix_ws<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Option<O> {
		*self
	}
}

impl<T> Format for PhantomData<T> {
	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		ControlFlow::Continue(())
	}

	fn format(&mut self, _ctx: &mut Context) {}

	fn with_prefix_ws<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Option<O> {
		None
	}
}

impl Format for () {
	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		ControlFlow::Continue(())
	}

	fn format(&mut self, _ctx: &mut Context) {}

	fn with_prefix_ws<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Option<O> {
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
		fn with_strings<O>(
			&mut self,
			ctx: &mut Context,
			f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
		) -> ControlFlow<O> {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.with_strings(ctx, f)
		}

		fn format(&mut self, ctx: &mut Context) {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.format(ctx)
		}

		fn with_prefix_ws<O>(
			&mut self,
			ctx: &mut Context,
			f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
		) -> Option<O> {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.with_prefix_ws(ctx, f)
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }

impl Format for AstStr {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Self, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		f(self, ctx)
	}

	fn format(&mut self, _ctx: &mut Context) {}

	fn with_prefix_ws<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Option<O> {
		None
	}
}

impl<T: ArenaData<Data: Format>> Format for ArenaIdx<T> {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		self.get_mut().with_strings(ctx, f)
	}

	fn format(&mut self, ctx: &mut Context) {
		self.get_mut().format(ctx);
	}

	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Option<O> {
		self.get_mut().with_prefix_ws(ctx, f)
	}
}

/// Format context
pub struct Context<'a, 'input> {
	input:        &'input str,
	config:       Cow<'a, Config>,
	indent_depth: usize,
}

impl<'a, 'input> Context<'a, 'input> {
	/// Creates a new context
	#[must_use]
	pub const fn new(input: &'input str, config: &'a Config) -> Self {
		Self {
			input,
			config: Cow::Borrowed(config),
			indent_depth: 0,
		}
	}

	/// Returns the input
	#[must_use]
	pub const fn input(&self) -> &'input str {
		self.input
	}

	/// Returns the string of a string
	#[must_use]
	pub fn str(&mut self, s: &AstStr) -> Cow<'input, str> {
		s.str(self.input)
	}

	/// Returns the config
	#[must_use]
	pub fn config(&self) -> &Config {
		&self.config
	}

	/// Returns the config mutably
	#[must_use]
	pub fn config_mut(&mut self) -> &mut Config {
		Cow::to_mut(&mut self.config)
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
		self.with_indent_offset(-1, f)
	}

	/// Runs `f` with one less indentation level if `pred` is true, otherwise
	/// runs it with the current indent
	pub fn without_indent_if<O>(&mut self, pred: bool, f: impl for<'b> FnOnce(&'b mut Self) -> O) -> O {
		self.with_indent_offset_if(-1, pred, f)
	}

	/// Runs `f` with an indentation offset of `offset`
	pub fn with_indent_offset<O>(&mut self, offset: isize, f: impl for<'b> FnOnce(&'b mut Self) -> O) -> O {
		let prev_depth = self.indent_depth;
		self.indent_depth = prev_depth.saturating_add_signed(offset);
		let output = f(self);
		self.indent_depth = prev_depth;
		output
	}

	/// Runs `f` with an indentation offset of `offset` if `pred` is true
	pub fn with_indent_offset_if<O>(
		&mut self,
		offset: isize,
		pred: bool,
		f: impl for<'b> FnOnce(&'b mut Self) -> O,
	) -> O {
		match pred {
			true => self.with_indent_offset(offset, f),
			false => f(self),
		}
	}

	#[doc(hidden)]
	pub const fn set_indent_depth(&mut self, indent_depth: usize) {
		self.indent_depth = indent_depth;
	}

	/// Creates a sub-context.
	///
	/// Sub contexts have their own configuration
	pub fn sub_context(&mut self) -> Context<'_, 'input> {
		Context {
			input:        self.input,
			config:       Cow::Borrowed(&self.config),
			indent_depth: self.indent_depth,
		}
	}
}

/// A formatting function
pub trait FormatFn<T: ?Sized> = Fn(&mut T, &mut Context);

/// Formats an arena value
pub fn arena<T: ArenaData>(f: impl FormatFn<T::Data>) -> impl FormatFn<ArenaIdx<T>> {
	move |idx, ctx| {
		let mut value = idx.get_mut();
		f(&mut value, ctx);
	}
}

/// Formats an `Option<Self>` with `f` if it is `Some`.
pub fn format_option_with<T>(f: impl FormatFn<T>) -> impl FormatFn<Option<T>> {
	move |value, ctx| {
		if let Some(value) = value {
			f(value, ctx);
		}
	}
}

/// Formats *all* items of a `Vec<T>` with `f`
pub fn format_vec_each_with_all<T>(f: impl FormatFn<T>) -> impl FormatFn<Vec<T>> {
	move |values, ctx| {
		for value in values {
			f(value, ctx);
		}
	}
}

/// Formats a `Vec<T>` (except the first element) with `f`
pub fn format_vec_each_with<T>(f: impl FormatFn<T>) -> impl FormatFn<Vec<T>> {
	move |values, ctx| {
		if let Some(values) = values.get_mut(1..) {
			for value in values {
				f(value, ctx);
			}
		}
	}
}
