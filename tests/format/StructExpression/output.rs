fn a() {
	let _ = A { };

	let _ = A { a };
	let _ = A { a, b };

	let _ = A {
		a,
		loooooooooooooooooooooooooooooooooooooooooooooooong
	};
}
