//! Printing

// Features
#![feature(never_type, decl_macro, coverage_attribute, macro_metavar_expr_concat)]

// Exports
pub use rustidy_macros::Print;

// Imports
use {
	core::marker::PhantomData,
	rustidy_format::{Config, Replacements},
	rustidy_util::{ArenaData, ArenaIdx, AstStr},
};

/// Printable types
pub trait Print: Sized {
	/// Prints this type onto a writer
	fn print(&self, f: &mut PrintFmt);
}

impl<T: Print> Print for &'_ T {
	fn print(&self, f: &mut PrintFmt) {
		(**self).print(f);
	}
}

impl<T: Print> Print for Box<T> {
	fn print(&self, f: &mut PrintFmt) {
		(**self).print(f);
	}
}

impl<T: Print> Print for Option<T> {
	fn print(&self, f: &mut PrintFmt) {
		if let Some(value) = self {
			value.print(f);
		}
	}
}

impl<T: Print> Print for Vec<T> {
	fn print(&self, f: &mut PrintFmt) {
		for value in self {
			value.print(f);
		}
	}
}

impl Print for ! {
	fn print(&self, _f: &mut PrintFmt) {
		*self
	}
}

impl<T> Print for PhantomData<T> {
	fn print(&self, _f: &mut PrintFmt) {}
}

impl Print for () {
	fn print(&self, _f: &mut PrintFmt) {}
}

macro tuple_impl($N:literal, $($T:ident),* $(,)?) {
	#[automatically_derived]
	impl< $($T: Print,)* > Print for ( $($T,)* ) {
		#[expect(non_snake_case)]
		fn print(&self, f: &mut PrintFmt) {
			let ( $($T,)* ) = self;
			$(
				$T.print(f);
			)*
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }

impl Print for AstStr {
	fn print(&self, f: &mut PrintFmt) {
		match f.replacements.get(self) {
			Some(replacement) => replacement.write(f.config, &mut f.output),
			None => f.output.push_str(self.str(f.input)),
		}
	}
}

impl<T: ArenaData<Data: Print>> Print for ArenaIdx<T> {
	fn print(&self, f: &mut PrintFmt) {
		T::ARENA.get(self).print(f);
	}
}

/// Print formatter
pub struct PrintFmt<'a, 'input> {
	input:        &'input str,
	output:       String,
	config:       &'a Config,
	replacements: &'a Replacements,
}

impl<'a, 'input> PrintFmt<'a, 'input> {
	/// Creates a new formatter
	#[must_use]
	pub const fn new(input: &'input str, config: &'a Config, replacements: &'a Replacements) -> Self {
		Self {
			input,
			output: String::new(),
			config,
			replacements,
		}
	}

	/// Returns the output
	#[must_use]
	pub fn output(&self) -> &str {
		&self.output
	}
}
