fn a() {
	let _ = [ 1 , 2 , 3 , 4 , 5 , 6 , ];
	#[rustidy::config(array_expr_rows = 2)]
	let _ = [ 1 , 2 , 3 , 4 , 5 , 6 , ];
	#[rustidy::config(array_expr_rows = 5)]
	let _ = [ 1 , 2 , 3 , 4 , 5 , 6 , ];
	#[rustidy::config(array_expr_rows = 10)]
	let _ = [ 1 , 2 , 3 , 4 , 5 , 6 , ];


	let _ = [ A { a } , B { b } , C { c } , D { d } , E { e } , F { f } , ];
	#[rustidy::config(array_expr_rows = 2)]
	let _ = [ A { a } , B { b } , C { c } , D { d } , E { e } , F { f } , ];
	#[rustidy::config(array_expr_rows = 5)]
	let _ = [ A { a } , B { b } , C { c } , D { d } , E { e } , F { f } , ];
	#[rustidy::config(array_expr_rows = 10)]
	let _ = [ A { a } , B { b } , C { c } , D { d } , E { e } , F { f } , ];
}
