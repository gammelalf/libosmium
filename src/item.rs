use std::mem::transmute;

use crate::area::Area;
use crate::handler::{Changeset, ChangesetDiscussion, Relation, RelationMemberList};
use crate::node::Node;
use crate::node_ref_list::{InnerRing, OuterRing, WayNodeList};
use crate::tag_list::TagList;
use crate::way::Way;

#[repr(C)]
pub struct Item {
    _size: ItemSize,
    _type: ItemType,
    _flags_and_padding: u16,
}

impl Item {
    /// Get the item's dynamic size
    pub fn byte_size(&self) -> ItemSize {
        self._size
    }

    /// Convert an item's reference into a reference of its actual subclass
    pub fn parse(&self) -> Option<ItemRef> {
        unsafe {
            Some(match self._type {
                ItemType::Node => ItemRef::Node(transmute(self)),
                ItemType::Way => ItemRef::Way(transmute(self)),
                ItemType::Relation => ItemRef::Relation(transmute(self)),
                ItemType::Area => ItemRef::Area(transmute(self)),
                ItemType::Changeset => ItemRef::Changeset(transmute(self)),
                ItemType::TagList => ItemRef::TagList(transmute(self)),
                ItemType::WayNodeList => ItemRef::WayNodeList(transmute(self)),
                ItemType::RelationMemberList => ItemRef::RelationMemberList(transmute(self)),
                ItemType::RelationMemberListWithFullMembers => {
                    ItemRef::RelationMemberList(transmute(self))
                }
                ItemType::OuterRing => ItemRef::OuterRing(transmute(self)),
                ItemType::InnerRing => ItemRef::InnerRing(transmute(self)),
                ItemType::ChangesetDiscussion => ItemRef::ChangesetDiscussion(transmute(self)),
                _ => return None,
            })
        }
    }
}

/// Enum for converting an item's reference into a reference of its actual subclass.
pub enum ItemRef<'a> {
    Node(&'a Node),
    Way(&'a Way),
    Relation(&'a Relation),
    Area(&'a Area),
    Changeset(&'a Changeset),
    TagList(&'a TagList),
    WayNodeList(&'a WayNodeList),
    RelationMemberList(&'a RelationMemberList),
    OuterRing(&'a OuterRing),
    InnerRing(&'a InnerRing),
    ChangesetDiscussion(&'a ChangesetDiscussion),
}

/// Enum identifying an item's actual subclass
///
/// Because libosmium needs to store different items next to each other in memory, they all store
/// their subclass as an enum at the beginning right after the size.
#[repr(u16)]
#[non_exhaustive]
pub enum ItemType {
    Undefined = 0x00,
    Node = 0x01,
    Way = 0x02,
    Relation = 0x03,
    Area = 0x04,
    Changeset = 0x05,
    TagList = 0x11,
    WayNodeList = 0x12,
    RelationMemberList = 0x13,
    RelationMemberListWithFullMembers = 0x23,
    OuterRing = 0x40,
    InnerRing = 0x41,
    ChangesetDiscussion = 0x80,
}

/// Memory size of an item.
///
/// Libosmium stores an item's collections continuously in one block of memory
/// instead of using std collections which would put them on extra heap allocations.
/// Therefore all items are dynamically sized and store their size at their beginning.
pub type ItemSize = u32;

macro_rules! impl_subclass {
    ($class:path) => {
        impl AsRef<Item> for $class {
            fn as_ref(&self) -> &Item {
                unsafe { transmute(self) }
            }
        }

        impl AsMut<Item> for $class {
            fn as_mut(&mut self) -> &mut Item {
                unsafe { transmute(self) }
            }
        }
    };
}
impl_subclass!(crate::tag_list::TagList);
impl_subclass!(crate::handler::ChangesetDiscussion);
impl_subclass!(crate::handler::RelationMemberList);
impl_subclass!(crate::node_ref_list::NodeRefList);
impl_subclass!(crate::handler::Changeset);
impl_subclass!(crate::object::OSMObject);
