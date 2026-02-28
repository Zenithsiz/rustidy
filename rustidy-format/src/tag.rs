//! Formatter tags

// Imports
use core::mem;

/// Formatter tag
pub trait FormatTag {
	type Data;

	fn field(tags: &FormatTags) -> &Option<Self::Data>;
	fn field_mut(tags: &mut FormatTags) -> &mut Option<Self::Data>;
}

macro decl_tags(
	$FormatTags:ident;
	$new:ident;
	$add:ident;
	$remove:ident;
	$set:ident;
	$contains:ident;

	$(
		$( #[$doc:meta] )*
		$Tag:ident: $TagData:ty,
	)*
) {
	$(
		$( #[$doc] )*
		pub struct $Tag;

		impl FormatTag for $Tag {
			type Data = $TagData;

			fn field(tags: &FormatTags) -> &Option<Self::Data> {
				&tags.$Tag
			}

			fn field_mut(tags: &mut FormatTags) -> &mut Option<Self::Data> {
				&mut tags.$Tag
			}
		}
	)*

	/// Formatter tags
	#[derive(Clone, Copy, Debug)]
	#[expect(non_snake_case, reason = "Macro-generated")]
	pub struct $FormatTags {
		$(
			$Tag: Option<$TagData>,
		)*
	}

	impl $FormatTags {
		/// Creates new, empty, tags
		#[must_use]
		pub const fn $new() -> Self {
			Self {
				$( $Tag: None, )*
			}
		}

		/// Adds a tag.
		///
		/// Returns the previous tag data, if any
		pub fn $add<Tag: FormatTag>(&mut self, data: Tag::Data) -> Option<Tag::Data> {
			self.$set::<Tag>(Some(data))
		}

		/// Removes a tag.
		///
		/// Returns the previous tag data, if any
		pub fn $remove<Tag: FormatTag>(&mut self) -> Option<Tag::Data> {
			self.$set::<Tag>(None)
		}

		/// Sets whether a tag is present or not.
		///
		/// Returns the previous tag data, if any.
		pub fn $set<Tag: FormatTag>(&mut self, data: Option<Tag::Data>) -> Option<Tag::Data> {
			mem::replace(Tag::field_mut(self), data)
		}

		/// Returns a tag's data, if it exists
		#[must_use]
		pub fn $contains<Tag: FormatTag>(&self) -> Option<&Tag::Data> {
			Tag::field(self).as_ref()
		}
	}

	impl Default for FormatTags {
		fn default() -> Self {
			Self::$new()
		}
	}
}

decl_tags! {
	FormatTags;
	new;
	add;
	remove;
	set;
	contains;

	/// Inside chain
	InsideChain: InsideChainData,

	/// After newline
	// Note: This attribute only works because every time
	//       we apply it, there's always whitespace directly
	//       after to remove it, otherwise it would stay for
	//       too long and be applied when it's no longer relevant.
	// TODO: Ideally, we'd assign some "position" to this, but
	//       during formatting, we no longer necessarily have
	//       the input ranges.
	AfterNewline: (),
}

#[derive(Clone, Copy, Debug)]
pub struct InsideChainData {
	pub indent: bool
}
