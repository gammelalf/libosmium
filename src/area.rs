use std::marker::PhantomData;
use std::os::raw::c_char;
use crate::node_ref_list::{InnerRing, OuterRing};

pub enum Area {}

impl Area {
    pub fn outer_rings(&self) -> impl Iterator<Item=&OuterRing> {
        unsafe { area_outer_rings(self) }
    }
    pub fn inner_rings<'a>(&'a self, outer: &'a OuterRing) -> impl Iterator<Item=&'a InnerRing> + 'a {
        unsafe { area_inner_rings(self, outer) }
    }
}

trait Ring: Sized {
    type Target;
    unsafe fn increment(_iter: &mut ItemIterator<Self>);
}
struct Outer; impl Ring for Outer {
    type Target = OuterRing;
    #[inline]
    unsafe fn increment(iter: &mut ItemIterator<Self>) {
        item_iterator_outer_ring_increment(iter);
    }
}
struct Inner; impl Ring for Inner {
    type Target = InnerRing;
    #[inline]
    unsafe fn increment(iter: &mut ItemIterator<Self>) {
        item_iterator_inner_ring_increment(iter);
    }
}

type ItemIteratorRange<'a, T> = ItemIterator<'a, T>;
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

extern "C" {
    fn area_outer_rings(area: &Area) -> ItemIteratorRange<Outer>;
    fn area_inner_rings<'a>(area: &'a Area, outer: &'a OuterRing) -> ItemIteratorRange<'a, Inner>;
    fn item_iterator_outer_ring_increment(iter: &mut ItemIterator<Outer>);
    fn item_iterator_inner_ring_increment(iter: &mut ItemIterator<Inner>);
}