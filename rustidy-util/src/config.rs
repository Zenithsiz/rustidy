//! Formatter configuration

// Imports
use std::sync::Arc;

macro decl_config($Config:ident; $($field:ident : $T:ty = $default:expr),* $(,)?) {
	/// Formatter configuration
	#[derive(Clone, Debug)]
	pub struct $Config {
		$(
			pub $field: $T,
		)*
	}

	impl Default for $Config {
		fn default() -> Self {
			Self {
				$(
					$field: $default,
				)*
			}
		}
	}

	impl serde::Serialize for Config {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer
		{
			#[derive(serde::Serialize)]
			struct ${ concat($Config, Repr) }<'a> {
				$(
					$field: Option<&'a $T>,
				)*
			}

			let repr = ${ concat($Config, Repr) } {
				$(
					$field: (self.$field != $default).then(|| &self.$field),
				)*
			};

			repr.serialize(serializer)
		}
	}

	impl<'de> serde::Deserialize<'de> for Config {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: serde::Deserializer<'de>
		{
			#[derive(serde::Deserialize)]
			struct ${ concat($Config, Repr) } {
				$(
					$field: Option<$T>,
				)*
			}

			let repr = ${ concat($Config, Repr) }::deserialize(deserializer)?;

			Ok(Self {
				$(
					$field: repr.$field.unwrap_or_else(|| $default),
				)*
			})

		}
	}
}

decl_config! {
	Config;

	indent:             Arc<str>         = "\t".into(),
	empty_line_spacing: EmptyLineSpacing = EmptyLineSpacing { min: 0, max: 2 },
	max_use_tree_len:   usize            = 75,
	array_expr_cols:    Option<usize>    = None,
	max_array_expr_len: usize            = 80,
}

/// Empty line spacing.
///
/// Keeps at least `min` empty lines in between consecutive things,
/// and at most `max` (inclusive).
// TODO: Should we allow this being different for items and statements?
// TODO: Remove this and just flatten it into the config
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmptyLineSpacing {
	pub min: usize,
	pub max: usize,
}
