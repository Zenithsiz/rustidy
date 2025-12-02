macro_rules ! a ( () => () );
macro_rules ! a [ () => () ];
macro_rules ! a { () => () }

macro_rules! a { () => (); () => (); }

macro_rules! a { () => () }
macro_rules! a { () => [] }
macro_rules! a { () => {} }

macro_rules! a { () => () }
macro_rules! a { [] => () }
macro_rules! a { {} => () }

macro_rules! a { (a) => () }
macro_rules! a { (()) => () }
macro_rules! a { ($a:expr) => () }
macro_rules! a { ($_:expr) => () }
macro_rules! a { ($(a)*) => () }
macro_rules! a { ($(a)+) => () }
macro_rules! a { ($(a)?) => () }

macro_rules! a { ($(a)b*) => () }

macro_rules! a { ($a:block) => () }
macro_rules! a { ($a:expr) => () }
macro_rules! a { ($a:expr_2021) => () }
macro_rules! a { ($a:ident) => () }
macro_rules! a { ($a:item) => () }
macro_rules! a { ($a:lifetime) => () }
macro_rules! a { ($a:literal) => () }
macro_rules! a { ($a:meta) => () }
macro_rules! a { ($a:pat) => () }
macro_rules! a { ($a:pat_param) => () }
macro_rules! a { ($a:path) => () }
macro_rules! a { ($a:stmt) => () }
macro_rules! a { ($a:tt) => () }
macro_rules! a { ($a:ty) => () }
macro_rules! a { ($a:vis) => () }

macro_rules! a { ( $($a:expr => $b:expr),* $(,)? ) => () }