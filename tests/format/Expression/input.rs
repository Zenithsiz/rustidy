fn chain() {
	let _ = a.b.c().d;

	#[rustidy::config(max_chain_len = 0)]
	let _ = a.b.c().d;

	#[rustidy::config(max_chain_len = 10)]
	let _ = a.long_field_a.long_method_b().long_field_c.long_method_d();

	#[rustidy::config(max_chain_len = 20)]
	let _ = a.short.long_method_a().short.long_field_b.short.long_method_c().short;

	#[rustidy::config(max_chain_len = 6)]
	let _ = a.b.c.d(a.b.c).e.f;

	#[rustidy::config(max_chain_len = 0)]
	let _ = a.b.c.d(|| {
		let _ = 5;
	}).e.f;

	#[rustidy::config(max_chain_len = 6)]
	let _ = f(a.b).c;
}


fn closure_inside_call() {
	let _ = a.f(|| {
		let _ = a;
	});

	let _ = a.f(a, b, c, d, || {
		let _ = a;
	});
}

fn doc_comment_newline() {
	fn a() {
		//! A1
		#![a]
		//! A2
		#![a]
		let _ = a;
	}
	
	fn b() {
		//! B1
		#![b]
		//! B2
		let _ = b;
	}

	fn c() {
		//! C1
		#![c]
		//! C2
	}

	pub enum A {
		/// B1
		#[b]
		/// B2
		#[c]
		B,

		/// C1
		#[c]
		/// C2
		#[c]
		/// C3
		C,
	}
}
