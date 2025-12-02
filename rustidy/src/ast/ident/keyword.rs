//! Keyword

/// Strict/Reserved keywords
#[rustfmt::skip]
pub const STRICT_OR_RESERVED_KEYWORDS: [&str; 52] = [
	// Strict (2015)
	"as",
	"break",
	"const",
	"continue",
	"crate",
	"else",
	"enum",
	"extern",
	"false",
	"fn",
	"for",
	"if",
	"impl",
	"in",
	"let",
	"loop",
	"match",
	"mod",
	"move",
	"mut",
	"pub",
	"ref",
	"return",
	"self",
	"Self",
	"static",
	"struct",
	"super",
	"trait",
	"true",
	"type",
	"unsafe",
	"use",
	"where",
	"while",

	// Strict (2015)
	"async",
	"await",
	"dyn",

	// Reserved (2015)
	"abstract",
	"become",
	"box",
	"do",
	"final",
	"macro",
	"override",
	"priv",
	"typeof",
	"unsized",
	"virtual",
	"yield",

	// Reserved (2018)
	"try",

	// Reserved (2024)
	"try",
];
