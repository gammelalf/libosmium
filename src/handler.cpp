#include "osmium/handler.hpp"      // Handler and all object types
#include "osmium/handler/node_locations_for_ways.hpp"
#include "osmium/index/map/flex_mem.hpp"
#include "osmium/io/pbf_input.hpp" // osmium::io::Reader for pbf files
#include "osmium/visitor.hpp"      // osmium::apply
#include "osmium/area/assembler.hpp"
#include "osmium/area/multipolygon_manager.hpp"

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