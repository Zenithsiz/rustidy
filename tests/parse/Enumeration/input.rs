enum A {}

enum A<T> {}
enum A where T: B {}

enum A { A }
enum A { A(u32) }
enum A { A() }
enum A { A { a: u32 } }
enum A { A {} }

enum A { #[a] A }
enum A { pub A }
enum A { A = 5 }

enum A { A, B(u32), C { a: u32 }, }

enum A<T> where T: B { #[a] pub A = 1, #[b] pub B(u32) = 2, #[c] pub C { a: u32 } = 3, }
