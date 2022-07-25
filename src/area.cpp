#include "osmium/memory/item_iterator.hpp"
#include "osmium/osm/area.hpp"

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