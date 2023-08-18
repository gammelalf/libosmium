use std::mem::MaybeUninit;

use crate::location::Location;

/// Reference to a [Node]
///
/// This is basically just the node's id so you could look it up if you had them all cached.
/// But it also contains a second field to be able to store a location, if it so happens to be known.
#[repr(C)]
pub struct NodeRef {
    /// The [Node]'s id
    pub id: i64,

    /// The [Node]'s location if known
    pub location: MaybeUninit<Location>,
}

impl NodeRef {
    /// Check if a valid location is set and return it.
    pub fn get_location(&self) -> Option<Location> {
        let loc = unsafe { self.location.assume_init_read() };
        if loc.is_valid() {
            Some(loc)
        } else {
            None
        }
    }
}

/// A Node is some point on the map with associated tags.
pub enum Node {}

impl Node {
    /// Get the node's location
    pub fn location(&self) -> Location {
        unsafe { node_location(self) }
    }

    /// set the node's location
    pub fn set_location(&mut self, location: &Location) {
        unsafe {
            set_node_location(self, location);
        }
    }
}

extern "C" {
    fn node_location(node: &Node) -> Location;
    fn set_node_location(node: &mut Node, location: &Location);
}
