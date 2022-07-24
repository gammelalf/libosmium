#include "osmium/osm/way.hpp"

extern "C" {
    const osmium::WayNodeList &way_nodes_const(const osmium::Way &way) {
        return way.nodes();
    }

    osmium::WayNodeList &way_nodes(osmium::Way &way) {
        return way.nodes();
    }
}