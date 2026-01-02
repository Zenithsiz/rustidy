const _: () = match () {
	// Top-level `|`
	| a => (),

	// Literal
	1 => (),
	-1 => (),
	b'_' => (),

	// Identifier
	a => (),
	ref a => (),
	mut a => (),
	a @ b => (),
	ref mut a @ b => (),

	// Wildcard
	_ => (),

	// Rest
	.. => (),

	// Reference
	&a => (),
	&&a => (),
	&mut a => (),

	// Struct
	A {} => (),
	A { a } => (),
	A { ref a } => (),
	A { mut a } => (),
	A { ref mut a } => (),
	A { 0: a } => (),
	A { a: a } => (),
	A { a, } => (),
	A { a, .. } => (),
	A { .. } => (),

	// Tuple struct
	A() => (),
	A(a) => (),
	A(a,) => (),
	A(a, b) => (),

	// Tuple
	() => (),
	(a,) => (),
	(..) => (),
	(a, b) => (),
	(a, b,) => (),

	// Grouped
	(a) => (),

	// Slice
	[] => (),
	[a] => (),
	[a,] => (),
	[a,b] => (),

	// Path
	a::b::c => (),
	<a as b>::c => (),

	// Macro
	a! {} => (),
};
