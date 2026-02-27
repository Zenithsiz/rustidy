macro a() {}
macro a { () => {} }
macro a { () => {}, () => {} }
macro a { () => {}, () => {}, }

macro a { () => () }
macro a { () => [] }
macro a { () => {} }

macro a { () => () }
macro a { [] => () }
macro a { {} => () }

macro a { (a) => () }
macro a { (()) => () }
macro a { ($a:expr) => () }
macro a { ($_:expr) => () }
macro a { ($(a)*) => () }
macro a { ($(a)+) => () }
macro a { ($(a)?) => () }

macro a { ($(a)b*) => () }

macro a { ($a:block) => () }
macro a { ($a:expr) => () }
macro a { ($a:expr_2021) => () }
macro a { ($a:ident) => () }
macro a { ($a:item) => () }
macro a { ($a:lifetime) => () }
macro a { ($a:literal) => () }
macro a { ($a:meta) => () }
macro a { ($a:pat) => () }
macro a { ($a:pat_param) => () }
macro a { ($a:path) => () }
macro a { ($a:stmt) => () }
macro a { ($a:tt) => () }
macro a { ($a:ty) => () }
macro a { ($a:vis) => () }

macro a { ( $($a:expr => $b:expr),* $(,)? ) => () }

macro a( $($a:expr => $b:expr),* $(,)? ) {}
