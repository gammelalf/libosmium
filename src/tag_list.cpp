#include "osmium/osm/tag.hpp"
#include "osmium/memory/collection.hpp"

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