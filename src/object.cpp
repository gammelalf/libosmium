#include "osmium/osm/tag.hpp"
#include "osmium/osm/object.hpp"

extern "C" {
    const osmium::TagList &object_tags(const osmium::OSMObject &object) {
        return object.tags();
    }
}
