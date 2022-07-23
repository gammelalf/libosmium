#include "osmium/handler.hpp"      // Handler and all object types
#include "osmium/io/pbf_input.hpp" // osmium::io::Reader for pbf files
#include "osmium/visitor.hpp"      // osmium::apply

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

extern "C" {
    void test(RustHandler handler, char *file) {
        osmium::io::Reader reader{file};
        osmium::apply(reader, handler);
        reader.close();
    }
}