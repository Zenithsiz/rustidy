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
}
