use std::marker::PhantomData;
use std::os::raw::c_char;

use crate::node_ref_list::{InnerRing, OuterRing};
use crate::object::ObjectId;

/// An area, as the name suggests, is some mapped area with tags.
///
/// ## Way or Relation?
/// An "area" is actually not a type of object defined by osm. It is more like a meta object.
/// In osm an "area" is ether a single way or a relation of ways with appropriated tags.
///
/// Use [`is_from_way`](Area::is_from_way) to check what a given area is actually saved as
/// and [`original_id`](Area::original_id) to get the underlying object's is.
///
/// ## Multipolygon
/// A multipolygon consists of multiple polygons named "outer rings" which contain "inner rings".
/// These are polygons as well, which are "cut out" from their outer ring.
/// Use [`outer_rings`](Area::outer_rings) and [`inner_rings`](Area::inner_rings) to access the area's rings.
///
/// Each ring i.e. polygon is just a [list of points](crate::node_ref_list::NodeRefList).
///
/// While the general shape of an area is a multipolygon, most areas are just a single outer ring without any inner ones.
/// For example most residential buildings are just a single polygon.
/// Use [`is_multipolygon`](Area::is_multipolygon) and [`num_rings`](Area::num_rings) to check an area's shape.
pub enum Area {}

impl Area {
    /// Was this area created from a way?
    /// (In contrast to areas created from a relation and their members.)
    pub fn is_from_way(&self) -> bool {
        // See original_id for the why
        self.positive_id() & 1 == 0
    }

    /// Return the Id of the way or relation this area was created from.
    pub fn original_id(&self) -> ObjectId {
        // libosmium uses the inline functions `object_id_to_area_id` and `area_id_to_object_id`
        // defined in <osmium/osm/area.hpp> to do this kind of conversion.
        //
        // From their source code its apparent, that the original id is simply shifted left once
        // and the newly free least significant bit is used as flag whether the original object
        // was a way or relation (0 = way, 1 = relation). So to convert back simply shift right once
        // dumping the flag bit.
        self.id() >> 1
    }

    /// Count the number of outer and inner rings of this area.
    pub fn num_rings(&self) -> (usize, usize) {
        let rings = unsafe { area_num_rings(self) };
        (rings.outer, rings.inner)
    }

    /// Check whether this area is a multipolygon, ie. whether it has more than one outer ring.
    pub fn is_multipolygon(&self) -> bool {
        self.num_rings().0 > 1
    }

    /// Return an iterator over all outer rings.
    pub fn outer_rings(&self) -> impl Iterator<Item = &OuterRing> {
        unsafe { area_outer_rings(self) }
    }

    /// Return an iterator over all inner rings in the given outer ring.
    // TODO consider making this a method on OuterRing?
    pub fn inner_rings<'a>(
        &'a self,
        outer: &'a OuterRing,
    ) -> impl Iterator<Item = &'a InnerRing> + 'a {
        unsafe { area_inner_rings(self, outer) }
    }
}

/// Iterator with same memory layout as c++'s `osmium::memory::ItemIterator`
///
/// It uses `T: Ring` to be generic over the [rust type](Ring::Target) to cast pointers into
/// and over the c++ [method](Ring::increment) to call to increment the iterator.
#[repr(C)]
struct ItemIterator<'a, T: Ring> {
    current: *const c_char,
    end: *const c_char,
    area_lifetime: PhantomData<&'a T>,
}
impl<'a, T: Ring> Iterator for ItemIterator<'a, T> {
    type Item = &'a T::Target;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        }

        unsafe { T::increment(self) };

        if self.current == self.end {
            None
        } else {
            Some(unsafe { self.current.cast::<T::Target>().as_ref().unwrap() })
        }
    }
}

trait Ring: Sized {
    type Target;
    unsafe fn increment(_iter: &mut ItemIterator<Self>);
}
struct Outer;
impl Ring for Outer {
    type Target = OuterRing;
    #[inline]
    unsafe fn increment(iter: &mut ItemIterator<Self>) {
        item_iterator_outer_ring_increment(iter);
    }
}
struct Inner;
impl Ring for Inner {
    type Target = InnerRing;
    #[inline]
    unsafe fn increment(iter: &mut ItemIterator<Self>) {
        item_iterator_inner_ring_increment(iter);
    }
}

// This type alias just makes the naming consistent with c++'s names.
type ItemIteratorRange<'a, T> = ItemIterator<'a, T>;

extern "C" {
    fn area_num_rings(area: &Area) -> NumRings;
    fn area_outer_rings(area: &Area) -> ItemIteratorRange<Outer>;
    fn area_inner_rings<'a>(area: &'a Area, outer: &'a OuterRing) -> ItemIteratorRange<'a, Inner>;
    fn item_iterator_outer_ring_increment(iter: &mut ItemIterator<Outer>);
    fn item_iterator_inner_ring_increment(iter: &mut ItemIterator<Inner>);
}

/// Boilerplate to ensure well defined layout for `(usize, usize)`
#[repr(C)]
struct NumRings {
    outer: usize,
    inner: usize,
}
