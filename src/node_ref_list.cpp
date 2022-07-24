#include "osmium/osm/node_ref_list.hpp"
#include "osmium/osm/node_ref.hpp"

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