#include "osmium/osm/tag.hpp"
#include "osmium/osm/object.hpp"

#define OSMObject(return_type, method_name) return_type OSMObject_##method_name(const osmium::OSMObject &object) { return object.method_name(); }
extern "C" {
    OSMObject(osmium::object_id_type, id)
    OSMObject(osmium::unsigned_object_id_type, positive_id)
    OSMObject(bool, deleted)
    OSMObject(bool, visible)
    OSMObject(osmium::object_version_type, version)
    OSMObject(osmium::user_id_type, uid)
    OSMObject(bool, user_is_anonymous)
    OSMObject(osmium::Timestamp, timestamp)
    OSMObject(const char *, user)
    OSMObject(const osmium::TagList &, tags)
}
