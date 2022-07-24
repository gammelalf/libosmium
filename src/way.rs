use std::ops::{Deref, DerefMut};
use crate::handler::Way;
use crate::node_ref_list::WayNodeList;

pub enum Way {}

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

extern "C" {
    fn way_nodes_const(way: &Way) -> &WayNodeList;
    fn way_nodes(way: &mut Way) -> &mut WayNodeList;
}