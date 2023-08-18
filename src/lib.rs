#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
//! ## Where to start
//!
//! As stated above this crate is for reading osm objects from a file.
//! Therefore, you should start with the [Handler] trait which does exactly that.

mod area;
pub use area::Area;

pub mod handler;
pub use handler::Handler;

mod item;
pub use item::{Item, ItemRef, ItemType};

mod location;
pub use location::{Location, PRECISION};

mod node;
pub use node::{Node, NodeRef};

pub mod node_ref_list;

mod object;
pub use object::OSMObject;

pub mod tag_list;

mod way;
pub use way::Way;

mod buffer;
pub use buffer::ItemBuffer;

mod impl_subclass {
    macro_rules! impl_as_ref {
        ($class:path as $base:path) => {
            impl AsRef<$base> for $class {
                #[doc = concat!("Cast to an [", stringify!($base) ,"] reference")]
                fn as_ref(&self) -> &$base {
                    unsafe { std::mem::transmute(self) }
                }
            }

            impl AsMut<$base> for $class {
                #[doc = concat!("Cast to an [", stringify!($base) ,"] reference")]
                fn as_mut(&mut self) -> &mut $base {
                    unsafe { std::mem::transmute(self) }
                }
            }
        };
    }
    macro_rules! impl_subclass {
        ($class:path as Item) => {
            impl_as_ref!($class as crate::item::Item);
        };

        ($class:path as OSMObject) => {
            impl std::ops::Deref for $class {
                type Target = $crate::object::OSMObject;

                #[doc = concat!("Cast to an [", stringify!($crate::object::OSMObject) ,"] reference")]
                fn deref(&self) -> &Self::Target {
                    unsafe { std::mem::transmute(self) }
                }
            }

            impl std::ops::DerefMut for $class {
                #[doc = concat!("Cast to an [", stringify!($crate::object::OSMObject) ,"] reference")]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    unsafe { std::mem::transmute(self) }
                }
            }

            impl_as_ref!($class as $crate::object::OSMObject);
            impl_as_ref!($class as $crate::item::Item);
        }
    }
    impl_subclass!(crate::area::Area as OSMObject);
    impl_subclass!(crate::node::Node as OSMObject);
    impl_subclass!(crate::handler::Relation as OSMObject);
    impl_subclass!(crate::way::Way as OSMObject);
    impl_subclass!(crate::tag_list::TagList as Item);
    impl_subclass!(crate::handler::ChangesetDiscussion as Item);
    impl_subclass!(crate::handler::RelationMemberList as Item);
    impl_subclass!(crate::node_ref_list::NodeRefList as Item);
    impl_subclass!(crate::handler::Changeset as Item);
    impl_subclass!(crate::object::OSMObject as Item);
}
