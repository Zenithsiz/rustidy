fn literal() {
	let _ = 5;
	let _ = 5u8;
	let _ = 5_u8;
}

fn path() {
	let _ = ::a::b::<g>::c;
	let _ = self;

	let _ = <a as b>::c;
}

fn operator() {
	let _ = &a;
	let _ = &&a;

	let _ = &mut a;
	let _ = &&mut a;

	let _ = &raw const a;
	let _ = &&raw const a;

	let _ = &raw mut a;
	let _ = &&raw mut a;

	let _ = *a;

	let _ = a?;

	let _ = -a;
	let _ = !a;

	let _ = a + b;
	let _ = a - b;
	let _ = a / b;
	let _ = a % b;
	let _ = a & b;
	let _ = a | b;
	let _ = a ^ b;
	let _ = a << b;
	let _ = a >> b;

	let _ = a == b;
	let _ = a != b;
	let _ = a > b;
	let _ = a < b;
	let _ = a >= b;
	let _ = a <= b;

	let _ = a || b;
	let _ = a && b;

	let _ = a as T;

	let _ = a = b;

	let _ = a += b;
	let _ = a -= b;
	let _ = a *= b;
	let _ = a /= b;
	let _ = a %= b;
	let _ = a &= b;
	let _ = a |= b;
	let _ = a ^= b;
	let _ = a <<= b;
	let _ = a >>= b;
}

fn grouped() {
	let _ = (5);
}

fn array() {
	let _ = [];
	let _ = [a];
	let _ = [a,];
	let _ = [a, b];
	let _ = [a; b];
}

fn await_() {
	let _ = a.await;
}

fn index() {
	let _ = a[5];
}

fn tuple() {
	let _ = ();
	let _ = (1,);
	let _ = (1,2);
	let _ = (1,2,);
}

fn tuple_index() {
	let _ = a.0;
	let _ = a.0.0;
	let _ = a.0.0.0;
}

fn struct_() {
	let _ = A {};
	let _ = A { a };
	let _ = A { a: b };
	let _ = A { a, b, };
	let _ = A { a, ..b };
	let _ = A { ..b };
}

fn call() {
	let _ = a();
	let _ = a(1);
	let _ = a(1,);
	let _ = a(1,2);
	let _ = a(1,2,);

	let _ = a(return);
}

fn method_call() {
	let _ = a.b();
	let _ = a.b(1);
	let _ = a.b(1,);
	let _ = a.b(1,2);
	let _ = a.b(1,2,);
}

fn field() {
	let _ = a.b;
}

fn closure() {
	let _ = || a;
	let _ = async || a;
	let _ = move || a;
	let _ = |a| a;
	let _ = |A(a)| a;
	let _ = |#[a] a| a;
	let _ = |a,| a;
	let _ = |a, b| a;
	let _ = |a, b,| a;
	let _ = |a: u32| a;
	let _ = |A(a): A| a;
	let _ = || -> u32 {};

	let _ = async move |#[a] A(a): u32, b, C(c): i32,| -> u32 {};
}

fn async_block() {
	let _ = async {};
	let _ = async move {};
}

fn continue_() {
	let _ = continue;
	let _ = continue 'a;
}

fn break_() {
	let _ = break;
	let _ = break 'a;
}

fn range() {
	let _ = a..b;
	let _ = a..;
	let _ = ..b;
	let _ = ..;
	let _ = a..=b;
	let _ = ..=b;
	let _ = .. .. ..;
	let _ = .. ..= ..;
	let _ = a..{};
}

fn return_() {
	let _ = return;
	let _ = return a;
}

fn underscore() {
	let _ = _;
}

fn macro_() {
	let _ = a!();
}

fn block() {
	let _ = {};
	let _ = { #![a] };
	let _ = { let a; };
	let _ = { let a; let b; };
	let _ = { a };
	let _ = { #![a] let a; let b; c };
}

fn const_block() {
	let _ = const {};
}

fn unsafe_block() {
	let _ = unsafe {};
}

fn loop_() {
	let _ = loop {};
	let _ = while a {};
	let _ = for A(a) in b {};
	let _ = 'a: {};

	let _ = for a in 0.. {};
}

fn if_() {
	let _ = if a {};
	let _ = if let a = b {};
	let _ = if let A(a) = b {};
	let _ = if #[a] let a = b {};
	let _ = if let a = b && let c = d {};
	let _ = if let a = b && c {};
	let _ = if let a = b && c && let d = e {};
	let _ = if let a = (b && c) && let d = e {};
	let _ = if a || b {};

	let _ = if a {} else {};
	let _ = if a {} else if b {};
	let _ = if a {} else if b {} else {};

	let _ = if if true {} {};

	let _ = if break {};
	let _ = if ({}) {};
	let _ = if (break {}) {};
	let _ = if 1 + break {};
	// Note: Although we can't parse the following case, the
	//       compiler *also* can't parse it, so we ought to be fine.
	//let _ = if break break {};
	let _ = if {} {};
	let _ = if !{} {};

	let _ = if 0 + {} + 0 {};
}

fn match_() {
	let _ = match a {};
	let _ = match a { #![a] };
	let _ = match a { a => b };
	let _ = match a { a => b, };
	let _ = match a { #[a] a => b };
	let _ = match a { a => b, c => d, };
	let _ = match a { a => {} c => d, };
	let _ = match a { a => {}, c => d, };
	let _ = match a { a if b => c };
	let _ = match a { () => {} () => {} };
	let _ = match a { () => {}?(), () => {} };
}

fn with_attr() {
	let _ = #[a] a;
}

fn complex() {
	&**a.b?.c + &**a.b?.c + &**a.b?.c;
	*self = match () {};
	match () {} + const {};
}

fn do_yeet() {
	do yeet 5;
	do yeet return 7;
}
