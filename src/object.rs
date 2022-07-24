use std::ops::{Deref, DerefMut};
use crate::tag_list::TagList;

pub enum OSMObject {}

impl OSMObject {
    pub fn tags(&self) -> &TagList {
        unsafe { object_tags(self) }
    }
}

extern "C" {
    fn object_tags(object: &OSMObject) -> &TagList;
}

macro_rules! impl_subclass {
    ($class:path) => {
        impl Deref for $class {
            type Target = OSMObject;

            fn deref(&self) -> &Self::Target {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl DerefMut for $class {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { std::mem::transmute(self) }
            }
        }
    }
}
impl_subclass!(crate::area::Area);
impl_subclass!(crate::node::Node);
impl_subclass!(crate::handler::Relation);
impl_subclass!(crate::way::Way);
