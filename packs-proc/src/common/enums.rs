use syn::Attribute;
use crate::common::get_tag_attr;

/// A thin wrapper around `Vec<u8>` which keeps track of the `tags` provided by the `tag` attribute
/// in variants of an enum.
pub struct Tags {
    tags: Vec<u8>,
}

impl Tags {
    fn add_tag(&mut self, tag: u8) {
        if self.tags.contains(&tag) {
            panic!("Tag {:X} is not unique!", tag)
        }

        self.tags.push(tag);
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut tags = Vec::with_capacity(cap);
        tags.push(0x00);
        Tags {
            tags,
        }
    }

    /// Adds a tag from attributes. Panics, if none is present.
    pub fn add_from_attr(&mut self, attrs: &Vec<Attribute>) {
        if let Some(t) = get_tag_attr(attrs) {
            self.add_tag(t)
        } else {
            panic!("No #[tag = u8] attribute found.")
        }
    }

    pub fn last_tag(&self) -> u8 {
        *self.tags.last().expect("No tag in Tags")
    }
}