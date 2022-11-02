//! defines the heart of this crate, the [Handler].
//!
//! It also contains not yet implemented osm types
//! which are already declared in order for the handler trait to be complete.

use std::ffi::{c_void, CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int};
use std::ptr;

use crate::area::Area;
use crate::node::Node;
use crate::node_ref_list::{InnerRing, OuterRing, WayNodeList};
use crate::object::OSMObject;
use crate::tag_list::TagList;
use crate::way::Way;

/// Not implemented yet
pub enum Changeset {}
/// Not implemented yet
pub enum ChangesetDiscussion {}
/// Not implemented yet
pub enum Relation {}
/// Not implemented yet
pub enum RelationMemberList {}

extern "C" {
    /// error_buffer is expected to be 256 bytes in size
    /// and c++ will only write 255, leaving the last one NUL.
    fn apply(handler: HandlerTable, file: *const c_char, error_buffer: *mut c_char);
    fn apply_with_ways(handler: HandlerTable, file: *const c_char, error_buffer: *mut c_char);
    fn apply_with_areas(
        handler: HandlerTable,
        file: *const c_char,
        error_buffer: *mut c_char,
        config: AreaAssemblerConfig,
    );
}

/// Macro to wrap ffi's apply functions
macro_rules! impl_apply {
    ($function:ident, $handler:expr, $file:expr) => {
        impl_apply!($function, $handler, $file, )
    };
    ($function:ident, $handler:expr, $file:expr, $($args:expr),*) => {{
        let file = CString::new($file).expect("File can't contain a NUL character");

        let mut error: [c_char; 256] = [0; 256];
        unsafe { $function($handler.as_table(), file.as_ptr(), error.as_mut_ptr(), $($args),*) };
        let error = error;

        // Empty error message -> no error
        if error[0] == 0 {
            Ok(())
        } else {
            // Safe, because error is existing memory on stack
            // and c++ only writes to 255 of the 256 bytes, so the last one will always stay a NUL.
            let cstr = unsafe { CStr::from_ptr(error.as_ptr()) };
            Err(CString::from(cstr))
        }
    }};
}

/// A handler is the entry interface for processing OSM files.
/// It is something which takes a stream of osm items.
///
/// For every item kind there is an associated method which takes a single item of this kind.
/// When implementing a handler to process OSM data, these are the methods to implement.
/// There is a default implementation for everything which just nothing i.e. ignores its kind.
///
/// The trait also has a few other methods. [`flush`](Handler::flush) will be called at least once,
/// after the final item has been processed to finalize a potentially lazy process.
///
/// The methods [`as_table`](Handler::as_table) and [`apply...`](Handler::apply) are not intended to be overwritten.
/// Instead they implement reading and processing PBF files using the handler instance.
///
/// This trait roughly mimics [`osmium::handler::Handler`](https://osmcode.org/libosmium/manual.html#handlers)
/// and can be converted into the subclass `RustHandler` (see `src/libosmium.cpp`) using [`as_table`](Handler::as_table).
pub trait Handler {
    /// Process an [Area]
    fn area(&mut self, _area: &Area) {}

    /// Process a [Changeset]
    fn changeset(&mut self, _changeset: &Changeset) {}

    /// Process a [ChangesetDiscussion]
    fn changeset_discussion(&mut self, _changeset_discussion: &ChangesetDiscussion) {}

    /// Process a [InnerRing]
    fn inner_ring(&mut self, _inner_ring: &InnerRing) {}

    /// Process a [Node]
    fn node(&mut self, _node: &Node) {}

    /// Process a [OSMObject]
    fn osm_object(&mut self, _object: &OSMObject) {}

    /// Process a [OuterRing]
    fn outer_ring(&mut self, _outer_ring: &OuterRing) {}

    /// Process a [Relation]
    fn relation(&mut self, _relation: &Relation) {}

    /// Process a [RelationMemberList]
    fn relation_member_list(&mut self, _relation_member_list: &RelationMemberList) {}

    /// Process a [TagList]
    fn tag_list(&mut self, _tag_list: &TagList) {}

    /// Process a [Way]
    fn way(&mut self, _way: &Way) {}

    /// Process a [WayNodeList]
    fn way_node_list(&mut self, _way_node_list: &WayNodeList) {}

    /// Finalize temporary ore lazy data
    fn flush(&mut self) {}

    /// Convert the handler into a [HandlerTable] which the c++ shim can interpret as a `RustHandler`
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

    /// Read a PBF file and process its items using this handler without any preprocessing.
    ///
    /// If you'd like c++ to do some preprocessing you might consider:
    /// - [`apply_with_ways`](Handler::apply_with_ways) populates a way's nodes' locations.
    /// - [`apply_with_areas`](Handler::apply_with_areas) assembles areas from ways and relations.
    fn apply(&mut self, file: &str) -> Result<(), CString> {
        impl_apply!(apply, self, file)
    }

    /// Read a PBF file, populates a way's nodes' locations and process the items using this handler.
    ///
    /// The preprocessing step of populating ways works by storing all already seen nodes' locations
    /// in a map and copying them into a way's node refs.
    fn apply_with_ways(&mut self, file: &str) -> Result<(), CString> {
        impl_apply!(apply_with_ways, self, file)
    }
    /// Read a PBF file, assemble areas and process the items using this handler.
    ///
    /// Since areas are [not actual osm items](Area#way-or-relation) stored in the file, you need to you this method,
    /// when you need areas at all. (Unless you write your own preprocessor.)
    ///
    /// This method also populates ways, like [`apply_with_ways`](Handler::apply_with_ways) does.
    ///
    /// Assembling areas is actually more involved than populating ways
    /// and requires an additional pass through the entire file increasing time and memory cost.
    fn apply_with_areas(&mut self, file: &str, config: AreaAssemblerConfig) -> Result<(), CString> {
        impl_apply!(apply_with_areas, self, file, config)
    }
}

/// The handler table is a virtual function table, comparable to `dyn Handler`.
///
/// Unlike `dyn Handler` it is FFI safe
/// and therefore used to represent an implementation of [Handler] on the c++ side.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HandlerTable<'a> {
    self_pointer: *const c_void,
    _self_lifetime: PhantomData<&'a ()>,
    area: *const (),
    changeset: *const (),
    changeset_discussion: *const (),
    inner_ring: *const (),
    node: *const (),
    osm_object: *const (),
    outer_ring: *const (),
    relation: *const (),
    relation_member_list: *const (),
    tag_list: *const (),
    way: *const (),
    way_node_list: *const (),
    flush: *const (),
}

/// This struct holds the varies parameter controlling how areas are assembled.
///
/// ## WIP
/// For now it is just a copy from the c++ code. Better abstraction might come in the future.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AreaAssemblerConfig {
    /// Optional pointer to problem reporter.
    pub problem_reporter: *const c_void,

    /// Debug level. If this is greater than zero, debug messages will be printed to stderr.
    /// Available levels are 1 to 3.
    /// Note that level 2 and above will generate a lot of messages!
    pub debug_level: c_int,

    /// The roles of multipolygon members are ignored when assembling multipolygons, because they are often missing or wrong.
    /// If this is set, the roles are checked after the multipolygons are built against what the assembly process decided where the inner and outer rings are.
    /// This slows down the processing, so it only makes sense if you want to get the problem reports.
    pub check_roles: bool,

    /// When the assembler can't create an area, usually because its geometry would be invalid, it will create an "empty" area object without rings.
    /// This allows you to detect where an area was invalid.
    ///
    /// If this is set to false, invalid areas will simply be discarded.
    pub create_empty_areas: bool,

    /// Create areas for (multi)polygons where the tags are on the relation.
    ///
    /// If this is set to false, those areas will simply be discarded.
    pub create_new_style_polygons: bool,

    /// Create areas for (multi)polygons where the tags are on the outer way(s).
    /// This is ignored by the area::Assembler which doesn't support old-style multipolygons any more.
    /// Use the area::AssemblerLegacy if you need this.
    ///
    /// If this is set to false, those areas will simply be discarded.
    pub create_old_style_polygons: bool,

    /// Create areas for polygons created from ways.
    ///
    /// If this is set to false, those areas will simply be discarded.
    pub create_way_polygons: bool,

    /// Keep the type tag from multipolygon relations on the area object. By default this is false, and the type tag will be removed.
    pub keep_type_tag: bool,

    /// If there is an invalid location in any of the ways needed for assembling the multipolygon, the assembler will normally fail.
    /// If this is set, the assembler will silently ignore the invalid locations pretending them to be not referenced from the ways.
    /// This will allow some areas to be built, others will now be incorrect.
    /// This can sometimes be useful to assemble areas crossing the boundary of an extract, but you will also get geometrically valid but wrong (multi)polygons.
    pub ignore_invalid_locations: bool,
}

impl Default for AreaAssemblerConfig {
    /// A copy of the c++'s default constructor
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
