// Construct areas from relations and ways
#include "osmium/area/assembler.hpp"
#include "osmium/area/multipolygon_manager.hpp"

// Handler class
#include "osmium/handler.hpp"

// A handler to populate all ways' nodes' locations
#include "osmium/handler/node_locations_for_ways.hpp"
#include "osmium/index/map/flex_mem.hpp"

// Support reading of pbf files
#include "osmium/io/pbf_input.hpp"

// Iterators for a TagList's tags and an Area's rings
#include "osmium/memory/collection.hpp"
#include "osmium/memory/item_iterator.hpp"

// Area class
#include "osmium/osm/area.hpp"

// Node class
#include "osmium/osm/node.hpp"

// Simplified node representation used as children in Areas and Ways
#include "osmium/osm/node_ref.hpp"
#include "osmium/osm/node_ref_list.hpp"

// OSMObject class
#include "osmium/osm/object.hpp"

// Way class
#include "osmium/osm/way.hpp"

// TagList class
#include "osmium/osm/tag.hpp"

// Function for reading a file and applying handlers on all items
#include "osmium/visitor.hpp"


// area.rs
extern "C" {
    struct NumRings {
        size_t outer, inner;
    };
    NumRings area_num_rings(osmium::Area &area) {
        std::pair<size_t, size_t> rings = area.num_rings();
        return {.outer = rings.first, .inner = rings.second};
    }
    osmium::memory::ItemIteratorRange<const osmium::OuterRing> area_outer_rings(const osmium::Area &area) {
        return area.outer_rings();
    }
    osmium::memory::ItemIteratorRange<const osmium::InnerRing> area_inner_rings(const osmium::Area &area, const osmium::OuterRing &outer) {
        return area.inner_rings(outer);
    }
    void item_iterator_outer_ring_increment(osmium::memory::ItemIterator<const osmium::OuterRing> &iter) {
        iter++;
    }
    void item_iterator_inner_ring_increment(osmium::memory::ItemIterator<const osmium::InnerRing> &iter) {
        iter++;
    }
}

// node.rs
extern "C" {
    osmium::Location node_location(const osmium::Node &node) {
        return node.location();
    }

    void node_set_location(osmium::Node &node, const osmium::Location &location) {
        node.set_location(location);
    }
}

// node_ref_list.rs
extern "C" {
    const osmium::NodeRef &node_ref_list_begin_const(const osmium::NodeRefList &list) {
        return *list.cbegin();
    }
    osmium::NodeRef &node_ref_list_begin(osmium::NodeRefList &list) {
        return *list.begin();
    }
    size_t node_ref_list_size(const osmium::NodeRefList &list) {
        return list.size();
    }
}

// object.rs
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

// tag_list.rs
extern "C" {
    const char* tag_list_get_value_by_key(const osmium::TagList &list, const char *key) {
        return list.get_value_by_key(key);
    }
    osmium::memory::CollectionIterator<const osmium::Tag> tag_list_begin(const osmium::TagList &list) {
        return list.begin();
    }
    osmium::memory::CollectionIterator<const osmium::Tag> tag_list_end(const osmium::TagList &list) {
        return list.end();
    }
}

// way.rs
extern "C" {
    const osmium::WayNodeList &way_nodes_const(const osmium::Way &way) {
        return way.nodes();
    }

    osmium::WayNodeList &way_nodes(osmium::Way &way) {
        return way.nodes();
    }
}

// handler.rs
template <class T>
using HandlerFunc = void (void *, const T &);

class RustHandler : public osmium::handler::Handler {
private:
    void *self_pointer;
    HandlerFunc<osmium::Area>*                area_handler;
    HandlerFunc<osmium::Changeset>*           changeset_handler;
    HandlerFunc<osmium::ChangesetDiscussion>* changeset_discussion_handler;
    HandlerFunc<osmium::InnerRing>*           inner_ring_handler;
    HandlerFunc<osmium::Node>*                node_handler;
    HandlerFunc<osmium::OSMObject>*           osm_object_handler;
    HandlerFunc<osmium::OuterRing>*           outer_ring_handler;
    HandlerFunc<osmium::Relation>*            relation_handler;
    HandlerFunc<osmium::RelationMemberList>*  relation_member_list_handler;
    HandlerFunc<osmium::TagList>*             tag_list_handler;
    HandlerFunc<osmium::Way>*                 way_handler;
    HandlerFunc<osmium::WayNodeList>*         way_node_list_handler;
    void (*flush_handler)(void *);

public:
    void area                 (const osmium::Area&                arg) { area_handler                 (self_pointer, arg); }
    void changeset            (const osmium::Changeset&           arg) { changeset_handler            (self_pointer, arg); }
    void changeset_discussion (const osmium::ChangesetDiscussion& arg) { changeset_discussion_handler (self_pointer, arg); }
    void inner_ring           (const osmium::InnerRing&           arg) { inner_ring_handler           (self_pointer, arg); }
    void node                 (const osmium::Node&                arg) { node_handler                 (self_pointer, arg); }
    void osm_object           (const osmium::OSMObject&           arg) { osm_object_handler           (self_pointer, arg); }
    void outer_ring           (const osmium::OuterRing&           arg) { outer_ring_handler           (self_pointer, arg); }
    void relation             (const osmium::Relation&            arg) { relation_handler             (self_pointer, arg); }
    void relation_member_list (const osmium::RelationMemberList&  arg) { relation_member_list_handler (self_pointer, arg); }
    void tag_list             (const osmium::TagList&             arg) { tag_list_handler             (self_pointer, arg); }
    void way                  (const osmium::Way&                 arg) { way_handler                  (self_pointer, arg); }
    void way_node_list        (const osmium::WayNodeList&         arg) { way_node_list_handler        (self_pointer, arg); }
    void flush() { flush_handler(self_pointer); }
};

using way_creator_map = osmium::index::map::FlexMem<osmium::unsigned_object_id_type, osmium::Location>;
using way_creator_type = osmium::handler::NodeLocationsForWays<way_creator_map>;
using area_creator_type = osmium::area::MultipolygonManager<osmium::area::Assembler>;

extern "C" {
    void apply(RustHandler handler, char *path) {
        osmium::io::File file{path};

        osmium::io::Reader reader{file};
        osmium::apply(
            reader,
            handler
        );
        reader.close();
    }

    void apply_with_ways(RustHandler handler, char *path) {
        osmium::io::File file{path};

        way_creator_map map;
        way_creator_type way_creator{map};
        way_creator.ignore_errors();

        osmium::io::Reader reader{file};
        osmium::apply(
            reader,
            way_creator,
            handler
        );
        reader.close();
    }

    void apply_with_areas(RustHandler handler, char *path, osmium::area::AssemblerConfig config) {
        const osmium::io::File file{path};

        way_creator_map map;
        way_creator_type way_creator{map};
        way_creator.ignore_errors();

        area_creator_type area_creator{config};
        osmium::relations::read_relations(file, area_creator);

        osmium::io::Reader reader{file, osmium::io::read_meta::no};
        osmium::apply(
            reader,
            way_creator,
            handler,
            area_creator.handler(
                [&handler](const osmium::memory::Buffer &area_buffer) {
                    osmium::apply(area_buffer, handler);
                }
            )
        );
        reader.close();
    }
}
