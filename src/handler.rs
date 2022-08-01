use std::ffi::c_void;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int};
use std::ptr;

use crate::area::Area;
use crate::node::Node;
use crate::node_ref_list::{InnerRing, OuterRing, WayNodeList};
use crate::object::OSMObject;
use crate::tag_list::TagList;
use crate::way::Way;

extern "C" {
    pub fn apply(handler: HandlerTable, file: *const c_char);
    pub fn apply_with_ways(handler: HandlerTable, file: *const c_char);
    pub fn apply_with_areas(
        handler: HandlerTable,
        file: *const c_char,
        config: AreaAssemblerConfig,
    );
}

pub enum Changeset {}
pub enum ChangesetDiscussion {}
pub enum Relation {}
pub enum RelationMemberList {}

/// Implement this trait to define a [Handler](https://osmcode.org/libosmium/manual.html#handlers)
pub trait Handler {
    fn area(&mut self, _area: &Area) {}
    fn changeset(&mut self, _changeset: &Changeset) {}
    fn changeset_discussion(&mut self, _changeset_discussion: &ChangesetDiscussion) {}
    fn inner_ring(&mut self, _inner_ring: &InnerRing) {}
    fn node(&mut self, _node: &Node) {}
    fn osm_object(&mut self, _object: &OSMObject) {}
    fn outer_ring(&mut self, _outer_ring: &OuterRing) {}
    fn relation(&mut self, _relation: &Relation) {}
    fn relation_member_list(&mut self, _relation_member_list: &RelationMemberList) {}
    fn tag_list(&mut self, _tag_list: &TagList) {}
    fn way(&mut self, _way: &Way) {}
    fn way_node_list(&mut self, _way_node_list: &WayNodeList) {}
    fn flush(&mut self) {}

    fn as_table(&mut self) -> HandlerTable {
        HandlerTable {
            self_pointer: self as *mut _ as *mut c_void,
            _self_lifetime: Default::default(),
            area: Self::area as *const (),
            changeset: Self::changeset as *const (),
            changeset_discussion: Self::changeset_discussion as *const (),
            inner_ring: Self::inner_ring as *const (),
            node: Self::node as *const (),
            osm_object: Self::osm_object as *const (),
            outer_ring: Self::outer_ring as *const (),
            relation: Self::relation as *const (),
            relation_member_list: Self::relation_member_list as *const (),
            tag_list: Self::tag_list as *const (),
            way: Self::way as *const (),
            way_node_list: Self::way_node_list as *const (),
            flush: Self::flush as *const (),
        }
    }
}

/// Representation for a [`Handler`] instance which is can be passed to cpp.
///
/// It is a function table in combination with a pointer to the [`Handler`] instance.
/// So it's basically a `dyn Handler` which was manually written to be FFI compatible.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HandlerTable<'a> {
    pub self_pointer: *const c_void,
    pub _self_lifetime: PhantomData<&'a ()>,
    pub area: *const (),
    pub changeset: *const (),
    pub changeset_discussion: *const (),
    pub inner_ring: *const (),
    pub node: *const (),
    pub osm_object: *const (),
    pub outer_ring: *const (),
    pub relation: *const (),
    pub relation_member_list: *const (),
    pub tag_list: *const (),
    pub way: *const (),
    pub way_node_list: *const (),
    pub flush: *const (),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AreaAssemblerConfig {
    pub problem_reporter: *const c_void,
    pub debug_level: c_int,
    pub check_roles: bool,
    pub create_empty_areas: bool,
    pub create_new_style_polygons: bool,
    pub create_old_style_polygons: bool,
    pub create_way_polygons: bool,
    pub keep_type_tag: bool,
    pub ignore_invalid_locations: bool,
}

impl Default for AreaAssemblerConfig {
    fn default() -> Self {
        AreaAssemblerConfig {
            problem_reporter: ptr::null(),
            debug_level: 0,
            check_roles: false,
            create_empty_areas: true,
            create_new_style_polygons: true,
            create_old_style_polygons: true,
            create_way_polygons: true,
            keep_type_tag: false,
            ignore_invalid_locations: false,
        }
    }
}
