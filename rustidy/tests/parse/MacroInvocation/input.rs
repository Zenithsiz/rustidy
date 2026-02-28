fn trailing_macro_invocation() {
	let _ = { a!{} };
	let _ = { a!{} 0 };
	let _ = { a!{} b!{} };
	let _ = { a!{}.f() };
	let _ = { a!{}? };
}