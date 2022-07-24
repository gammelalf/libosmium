use std::ops::{Deref, DerefMut};
use crate::handler::{Way, WayNodeList};
use crate::node_ref_list::NodeRefList;

impl Deref for Way {
    type Target = WayNodeList;
    fn deref(&self) -> &Self::Target {
        unsafe { way_nodes_const(self) }
    }
}

impl DerefMut for Way {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { way_nodes(self) }
    }
}

impl Deref for WayNodeList {
    type Target = NodeRefList;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl DerefMut for WayNodeList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

extern "C" {
    fn way_nodes_const(way: &Way) -> &WayNodeList;
    fn way_nodes(way: &mut Way) -> &mut WayNodeList;
}