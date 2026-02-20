fn a() {
	let _ = [ 1 ];
	let _ = [
		1
	];

	let _ = [
		// Comment
		1
	];
	let _ = [
		1
		// Comment
	];

	let _ = [ 1 , 2 , 3 , 4 , 5 , 6 , ];

	#[rustidy::config(max_array_expr_len = 6)]
	let _ = [ 1 , 2 , 3 , 4 , 5 , 6 , ];

	#[rustidy::config(array_expr_cols = 1)]
	let _ = [ 1 , 2 , 3 , 4 , 5 , 6 , ];
	#[rustidy::config(array_expr_cols = 2)]
	let _ = [ 1 , 2 , 3 , 4 , 5 , 6 , ];
	#[rustidy::config(array_expr_cols = 5)]
	let _ = [ 1 , 2 , 3 , 4 , 5 , 6 , ];
	#[rustidy::config(array_expr_cols = 10)]
	let _ = [ 1 , 2 , 3 , 4 , 5 , 6 , ];


	let _ = [ A { a } , B { b } , C { c } , D { d } , E { e } , F { f } , ];
	#[rustidy::config(array_expr_cols = 1)]
	let _ = [ A { a } , B { b } , C { c } , D { d } , E { e } , F { f } , ];
	#[rustidy::config(array_expr_cols = 2)]
	let _ = [ A { a } , B { b } , C { c } , D { d } , E { e } , F { f } , ];
	#[rustidy::config(array_expr_cols = 5)]
	let _ = [ A { a } , B { b } , C { c } , D { d } , E { e } , F { f } , ];
	#[rustidy::config(array_expr_cols = 10)]
	let _ = [ A { a } , B { b } , C { c } , D { d } , E { e } , F { f } , ];
	
	#[rustidy::config(array_expr_cols = 2)]
	let _ = [ 1 , A { a } , 2 , B { b } , 3 , C { c } , 4 , D { d } , 5 , E { e } , 6 , F { f } , ];
}
