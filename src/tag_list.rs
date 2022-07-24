use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

pub enum TagList {}

impl TagList {
    pub(crate) fn get_value_by_key(&self, key: &CStr) -> Option<&CStr> {
        let key = key.as_ptr();
        let value = unsafe { tag_list_get_value_by_key(self, key) };
        if value.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(value) })
        }
    }
}
impl<'a> IntoIterator for &'a TagList {
    type Item = (&'a CStr, &'a CStr);
    type IntoIter = TagIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TagIterator {
            current: unsafe { tag_list_begin(self) },
            end: unsafe { tag_list_end(self) },
            list_lifetime: PhantomData::default(),
        }
    }
}

#[repr(C)]
pub struct TagIterator<'a> {
    current: *const c_char,
    end: *const c_char,
    list_lifetime: PhantomData<&'a ()>,
}
impl<'a> Iterator for TagIterator<'a> {
    type Item = (&'a CStr, &'a CStr);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let key_start = self.current;
            unsafe {self.current = after_null(self.current);}
            let value_start = self.current;
            unsafe {self.current = after_null(self.current);}
            Some(
                unsafe {
                    (CStr::from_ptr(key_start), CStr::from_ptr(value_start))
                }
            )
        }
    }
}

/// Increment a char pointer past the next null character
unsafe fn after_null(mut ptr: *const c_char) -> *const c_char {
    while *ptr != 0 {
        ptr = ptr.add(1);
    }
    ptr.add(1)
}

extern "C" {
    fn tag_list_get_value_by_key(list: &TagList, key: *const c_char) -> *const c_char;
    fn tag_list_begin(list: &TagList) -> *const c_char;
    fn tag_list_end(list: &TagList) -> *const c_char;
}