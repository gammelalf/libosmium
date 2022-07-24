//! Types storing a list of nodes:
//! - `NodeRefList`: is the base type for all list of nodes
//! - `WayNodeList`: a [`Way`]'s list of nodes
//! - `InnerRing` / `OuterRing`: the two typed of rings defining an [`Area`]
//!
//! From looking at [libosmium's code](https://docs.osmcode.org/libosmium/latest/node__ref__list_8hpp_source.html)
//! it seems that `NodeRefList` is the only class of real importance since the other three just extend it and overwrite
//! a single class attribute as well as a check method depending on it.
//!
//! Since this binding doesn't care about libosmium's internal memory layout
//! and there was no need to implement this check, the subclasses are just type aliases on rust's side.
//!
//! [`Area`]: crate::area::Area
//! [`Way`]: crate::way::Way
use crate::node::NodeRef;
use std::ops::{Deref, DerefMut};

pub enum NodeRefList {}

pub type WayNodeList = NodeRefList;
pub type InnerRing = NodeRefList;
pub type OuterRing = NodeRefList;

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