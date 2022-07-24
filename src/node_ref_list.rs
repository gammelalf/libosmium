use crate::node::NodeRef;
use std::ops::{Deref, DerefMut};
use crate::location::Location;

pub enum NodeRefList {}

impl Deref for NodeRefList {
    type Target = [NodeRef];
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(
                node_ref_list_begin_const(self) as *const NodeRef,
                node_ref_list_size(self)
            )
        }
    }
}

impl DerefMut for NodeRefList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(
                node_ref_list_begin(self) as *mut NodeRef,
                node_ref_list_size(self)
            )
        }
    }
}

extern "C" {
    fn node_ref_list_begin_const(list: &NodeRefList) -> &NodeRef;
    fn node_ref_list_begin(list: &mut NodeRefList) -> &mut NodeRef;
    fn node_ref_list_size(list: &NodeRefList) -> usize;
}