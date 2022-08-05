use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

pub enum TagList {}

impl TagList {
    pub fn get<'a>(&'a self, key: &'_ str) -> Option<&'a str> {
        for (k, v) in self.into_iter() {
            if k == key {
                return Some(v);
            }
        }
        return None;
    }
}
impl<'a> IntoIterator for &'a TagList {
    type Item = (&'a str, &'a str);
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
    list_lifetime: PhantomData<&'a TagList>,
}
impl<'a> Iterator for TagIterator<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < self.end {
            let key_start = self.current;
            unsafe {
                self.current = after_null(self.current);
            }

            let value_start = self.current;
            unsafe {
                self.current = after_null(self.current);
            }

            let (key, value) = unsafe { (CStr::from_ptr(key_start), CStr::from_ptr(value_start)) };

            if let (Ok(key), Ok(value)) = (key.to_str(), value.to_str()) {
                return Some((key, value));
            } else {
                eprint!("[libosmium]: got invalid utf-8 in tag -> skipping...");
                continue;
            }
        }
        None
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
    fn tag_list_begin(list: &TagList) -> *const c_char;
    fn tag_list_end(list: &TagList) -> *const c_char;
}
