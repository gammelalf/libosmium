use std::ffi::CStr;
use std::os::raw::c_char;

use crate::tag_list::TagList;

/// Base class for OSM 's objects:
/// - [Node](crate::node::Node)
/// - [Way](crate::way::Way)
/// - Relation
/// - [Area](crate::Area)
///
/// Since the above types are c++ subclasses, a (for example) Way pointer is also a valid OSMObject.
/// To reflect this these types implement [Deref](std::ops::Deref) and [DerefMut](std::ops::DerefMut) which just uses an pointer cast.
pub enum OSMObject {}

impl OSMObject {
    /// Get ID of this object.
    pub fn id(&self) -> ObjectId {
        unsafe { OSMObject_id(self) }
    }

    /// Get absolute value of the ID of this object.
    pub fn positive_id(&self) -> UnsignedObjectId {
        unsafe { OSMObject_positive_id(self) }
    }

    /// Is this object marked as deleted?
    pub fn deleted(&self) -> bool {
        unsafe { OSMObject_deleted(self) }
    }

    /// Is this object marked visible (ie not deleted)?
    pub fn visible(&self) -> bool {
        unsafe { OSMObject_visible(self) }
    }

    /// Get version of this object.
    pub fn version(&self) -> ObjectVersion {
        unsafe { OSMObject_version(self) }
    }

    /// Get user id of this object.
    pub fn uid(&self) -> UserId {
        unsafe { OSMObject_uid(self) }
    }

    /// Is this user anonymous?
    pub fn user_is_anonymous(&self) -> bool {
        unsafe { OSMObject_user_is_anonymous(self) }
    }

    /// Get timestamp when this object last changed.
    pub fn timestamp(&self) -> Timestamp {
        unsafe { OSMObject_timestamp(self) }
    }

    /// Get user name for this object.
    pub fn user(&self) -> &CStr {
        unsafe { CStr::from_ptr(OSMObject_user(self)) }
    }

    /// Get the list of tags for this object.
    pub fn tags(&self) -> &TagList {
        unsafe { OSMObject_tags(self) }
    }
}

/// Type for OSM user IDs.
pub type UserId = u32;

/// Type for OSM object (node, way, or relation) IDs.
pub type ObjectId = i64;

/// Type for OSM object (node, way, or relation) IDs where we only allow positive IDs.
pub type UnsignedObjectId = u64;

/// Type for OSM object version number.
pub type ObjectVersion = u32;

/// A timestamp. Internal representation is an unsigned 32bit integer holding seconds
/// since epoch (1970-01-01T00:00:00Z), so this will overflow in 2106.
/// We can use an unsigned integer here, because the OpenStreetMap project was started
/// long after 1970, so there will never be dates before that.
pub type Timestamp = u32;

extern "C" {
    fn OSMObject_id(object: &OSMObject) -> ObjectId;
    fn OSMObject_positive_id(object: &OSMObject) -> UnsignedObjectId;
    fn OSMObject_deleted(object: &OSMObject) -> bool;
    fn OSMObject_visible(object: &OSMObject) -> bool;
    fn OSMObject_version(object: &OSMObject) -> ObjectVersion;
    fn OSMObject_uid(object: &OSMObject) -> UserId;
    fn OSMObject_user_is_anonymous(object: &OSMObject) -> bool;
    fn OSMObject_timestamp(object: &OSMObject) -> Timestamp;
    fn OSMObject_user(object: &OSMObject) -> *const c_char;
    fn OSMObject_tags(object: &OSMObject) -> &TagList;
}
