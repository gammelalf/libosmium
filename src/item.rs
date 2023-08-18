use std::mem::transmute;

use crate::area::Area;
use crate::handler::{Changeset, ChangesetDiscussion, Relation, RelationMemberList};
use crate::node::Node;
use crate::node_ref_list::{InnerRing, OuterRing, WayNodeList};
use crate::tag_list::TagList;
use crate::way::Way;

/// This type is the base class responsible for libosmium's custom memory management.
///
/// It stores an object's dynamic size and its actual subclass as an enum.
#[repr(C)]
pub struct Item {
    _size: ItemSize,
    _type: ItemType,
    _flags_and_padding: u16,
}

/// Align items to this many bytes
const ALIGN_BYTES: usize = 8;

impl Item {
    /// Get the item's data as byte slice padded to [`aligned_size`](Self::aligned_size)
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const Item as *const u8, self.aligned_size()) }
    }

    /// Get the item's dynamic size aligned to `ALIGN_BYTES`
    pub fn aligned_size(&self) -> usize {
        (self._size as usize + ALIGN_BYTES - 1) & !(ALIGN_BYTES - 1)
    }

    /// Get the item's dynamic size
    pub fn byte_size(&self) -> ItemSize {
        self._size
    }

    /// Get the item's type
    pub fn item_type(&self) -> ItemType {
        self._type
    }

    /// Convert an item's reference into a reference of its actual subclass
    pub fn cast(&self) -> Option<ItemRef> {
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

/// Enum for converting an [Item] reference into a reference of its actual subclass.
#[derive(Copy, Clone)]
pub enum ItemRef<'a> {
    /// A [Node] reference
    Node(&'a Node),

    /// A [Way] reference
    Way(&'a Way),

    /// A [Relation] reference
    Relation(&'a Relation),

    /// An [Area] reference
    Area(&'a Area),

    /// A [Changeset] reference
    Changeset(&'a Changeset),

    /// A [TagList] reference
    TagList(&'a TagList),

    /// A [WayNodeList] reference
    WayNodeList(&'a WayNodeList),

    /// A [RelationMemberList] reference
    RelationMemberList(&'a RelationMemberList),

    /// An [OuterRing] reference
    OuterRing(&'a OuterRing),

    /// An [InnerRing] reference
    InnerRing(&'a InnerRing),

    /// A [ChangesetDiscussion] reference
    ChangesetDiscussion(&'a ChangesetDiscussion),
}

/// Enum identifying an item's actual subclass
///
/// Because libosmium needs to store different items next to each other in memory, they all store
/// their subclass as an enum at the beginning right after the size.
#[repr(u16)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[allow(missing_docs)]
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
