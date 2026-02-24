//! Printing

// Features
#![feature(never_type, decl_macro, coverage_attribute, macro_metavar_expr_concat)]

// Modules
mod whitespace;

// Exports
pub use rustidy_macros::Print;

// Imports
use {core::marker::PhantomData, rustidy_util::{ArenaData, ArenaIdx, AstStr}};

/// Printable types
pub trait Print: Sized {
	/// Prints this type onto a writer
	fn print(&self, f: &mut PrintFmt);

	/// Prints this type onto a writer excluding whitespace
	///
	/// Note that this excludes necessary whitespace too, so this
	/// won't provide valid rust code.
	fn print_non_ws(&self, f: &mut PrintFmt);

	/// Prints this type onto a string
	fn print_to_string(&self) -> String {
		let mut f = PrintFmt::new();
		self.print(&mut f);
		f.output
	}
}

impl<T: Print> Print for &'_ T {
	fn print(&self, f: &mut PrintFmt) {
		(**self).print(f);
	}

	fn print_non_ws(&self, f: &mut PrintFmt) {
		(**self).print(f);
	}
}

impl<T: Print> Print for Box<T> {
	fn print(&self, f: &mut PrintFmt) {
		(**self).print(f);
	}

	fn print_non_ws(&self, f: &mut PrintFmt) {
		(**self).print(f);
	}
}

impl<T: Print> Print for Option<T> {
	fn print(&self, f: &mut PrintFmt) {
		if let Some(value) = self {
			value.print(f);
		}
	}

	fn print_non_ws(&self, f: &mut PrintFmt) {
		if let Some(value) = self {
			value.print_non_ws(f);
		}
	}
}

impl<T: Print> Print for Vec<T> {
	fn print(&self, f: &mut PrintFmt) {
		for value in self {
			value.print(f);
		}
	}

	fn print_non_ws(&self, f: &mut PrintFmt) {
		for value in self {
			value.print_non_ws(f);
		}
	}
}

impl Print for ! {
	fn print(&self, _f: &mut PrintFmt) {
		*self
	}

	fn print_non_ws(&self, _f: &mut PrintFmt) {
		*self
	}
}

impl<T> Print for PhantomData<T> {
	fn print(&self, _f: &mut PrintFmt) {}

	fn print_non_ws(&self, _f: &mut PrintFmt) {}
}

impl Print for () {
	fn print(&self, _f: &mut PrintFmt) {}

	fn print_non_ws(&self, _f: &mut PrintFmt) {}
}

macro tuple_impl(
	$N:literal, $($T:ident),* $(,)?
) {
	#[automatically_derived]
	impl< $($T: Print,)* > Print for ( $($T,)* ) {
		#[expect(non_snake_case)]
		fn print(&self, f: &mut PrintFmt) {
			let ( $($T,)* ) = self;
			$(
				$T.print(f);
			)*
		}

		#[expect(non_snake_case)]
		fn print_non_ws(&self, f: &mut PrintFmt) {
			let ( $($T,)* ) = self;
			$(
				$T.print_non_ws(f);
			)*
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }

impl Print for AstStr {
	fn print(&self, f: &mut PrintFmt) {
		self.write(&mut f.output);
	}

	fn print_non_ws(&self, f: &mut PrintFmt) {
		self.write(&mut f.output);
	}
}

impl<T: ArenaData + Print> Print for ArenaIdx<T> {
	fn print(&self, f: &mut PrintFmt) {
		(**self).print(f);
	}

	fn print_non_ws(&self, f: &mut PrintFmt) {
		(**self).print(f);
	}
}

/// Print formatter
pub struct PrintFmt {
	output: String,
}

impl PrintFmt {
	/// Creates a new formatter
	#[must_use]
	pub const fn new() -> Self {
		Self { output: String::new() }
	}

	/// Returns the output
	#[must_use]
	pub fn output(&self) -> &str {
		&self.output
	}
}

impl Default for PrintFmt {
	fn default() -> Self {
		Self::new()
	}
}
