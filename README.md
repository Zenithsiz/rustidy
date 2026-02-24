# rustidy

A rust source code formatter

## Features

- Full support for all stable syntax (except for non-derive macros, which are only parsed, not formatted).

- Support for some nightly-only syntax (such as `try` blocks, `do yeet` expressions, and more).

- Formatting derive macro attributes (formats expressions inside of attributes)

- Ability to change the configuration for a block with attributes (both outer and inner attributes are supported):

```rust
// Change the threshold for splitting an array into multi-line.
#[rustidy::config(max_array_expr_len = 100)]
const ARRAY: [u32; 25] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
#[rustidy::config(max_array_expr_len = 0)]
const ARRAY: [u32; 2] = [
	1,
	2,
];

// Format an array with columns
#[rustidy::config(array_expr_cols = 3)]
const ARRAY: [u32; 8] = [
	1, 2, 3,
	4, 5, 6,
	7, 8,
]

// Change the indentation on a part of the code
#[rustidy::config(indent = "  ")]
fn main() {
  println!("Hello world!");
}
#[rustidy::config(indent = "\t\t")]
fn main() {
		println!("Hello world!");
}

// And of course, you can skip formatting altogether
#[rustidy::skip]
fn main() {
	1      +      6      +
	9      +      9;
}
```

## Installation

### From source

You can install `rustidy` using `cargo install` on a nightly toolchain:

```sh
cargo +nightly install --locked rustidy
```

You should also install the cargo extension `cargo rustidy`:

```sh
cargo +nightly install --locked cargo-rustidy
```

### From binaries

- Binaries are distributed through [github releases](https://github.com/Zenithsiz/rustidy/releases)

## Usage

### Using `cargo rustidy`

The most common way to use `rustidy` is to use it to format all files in a cargo package or workspace using `cargo rustidy`:

```sh
cargo rustidy
```

This will try to use the `rustidy` binary in your `$PATH`. You can also directly specify the binary to use using the `RUSTIDY` environment variable:

```sh
RUSTIDY=.../rustidy cargo rustidy
```

For pre-commit hooks or CI servers, you can also check that code is formatted properly using the `--check` flag:

```sh
# Exits with 0 if all files are formatted properly, and 1 otherwise
cargo rustidy --check
```

### Using `rustidy` directly

You can directly format a file in-place (and all of it's modules) using `rustidy` directly:

```sh
rustidy file1.rs file2.rs ...
```

`rustidy` also has the ability to read input from stdin and output it to stdout. This is the default mode when no arguments are passed:

```sh
printf "fn main() {}" | rustidy > output.rs
```

Similarly to `cargo rustidy`, `rustidy` has a `--check` argument to ensure that the specified files are all formatted properly:

```sh
rustidy --check file1.rs file2.rs ...
```

## Editors

To run rustidy in your editors, you can typically specify it as a replacement for `rustfmt`.

In VSCode using `rust-analyzer`, you can achieve this with the following settings:

```json
"rust-analyzer.rustfmt.overrideCommand": ["rustidy"]
```

## Configuration

Rustidy can be configured using a `.rustidy.toml` file in any parent directory from where it was invoked.
Only the first file found will be used as the configuration.

See [an example configuration file](.rustidy.toml).

Currently the configuration options are not documented, but you can check a list of them, including their default values, on [the source](rustidy-util/src/config.rs).

## License

This project is dual-licensed under either of the following licenses:

- [Apache license Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)
