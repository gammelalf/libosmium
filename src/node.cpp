#include "osmium/osm/node.hpp"

extern "C" {
    osmium::Location node_location(const osmium::Node &node) {
        return node.location();
    }

    void node_set_location(osmium::Node &node, const osmium::Location &location) {
        node.set_location(location);
    }
}