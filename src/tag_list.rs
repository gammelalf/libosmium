//! defines the [TagList] as well as its [iterator](TagIterator).
//!
//! It also provides an [owned version](OwnedTagList) for mostly for testing purposes.

use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

/// A tag list is a map from tag names to their values.
///
/// To be memory efficient it is stored as a slice of key value pairs.
/// Keys and Values are separated using a `NUL` and the slice is terminated with a double `NUL`.
pub enum TagList {}

impl TagList {
    /// Lookup a key's value
    pub fn get<'a>(&'a self, key: &'_ str) -> Option<&'a str> {
        for (k, v) in self.into_iter() {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    /// Are there any tags at all?
    pub fn is_empty(&self) -> bool {
        unsafe { tag_list_begin(self) == tag_list_end(self) }
    }

    /// Get the raw underlying memory as slice.
    ///
    /// See [TagList] for a description of this memory's layout.
    ///
    /// This method is also exposed via an [AsRef<\[u8\]>] implementation.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let begin = tag_list_begin(self) as *const u8;
            let end = tag_list_end(self);
            std::slice::from_raw_parts(begin, end as usize - begin as usize)
        }
    }
}

impl AsRef<[u8]> for TagList {
    /// See [TagList] for a description of this memory's layout.
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> IntoIterator for &'a TagList {
    type Item = (&'a str, &'a str);
    type IntoIter = TagIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TagIterator {
            current: unsafe { tag_list_begin(self) },
            end: unsafe { tag_list_end(self) },
            list_lifetime: PhantomData,
        }
    }
}

impl std::fmt::Debug for TagList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug(self, f)
    }
}

/// Iterator over [TagList]'s key value pairs
///
/// This iterator handles the `NUL` terminator and converts c strings into rust strings.
/// If a c string doesn't not contain valid utf-8 it will be skipped and logged to stderr.
/// However by osm specification everything _should_ be utf-8.
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

/// An owned version of the [`TagList`], implemented purely in rust.
///
/// The actual data is stored in the same way as in [`TagList`]
/// and therefore uses the same accessing logic. But it manages its own memory using a [`Vec<u8>`]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OwnedTagList(pub Vec<u8>);

impl OwnedTagList {
    /// Constructs a new, empty `OwnedTagList`.
    ///
    /// The list will not allocate until elements are pushed onto it. (It just calls [`Vec::new`])
    pub const fn new() -> OwnedTagList {
        OwnedTagList(Vec::new())
    }

    /// Internal helper method to push a single string and a trailing `NUL` character
    ///
    /// See [`push_pair`] for panic explanation
    ///
    /// [`push_pair`]: OwnedTagList::push_pair
    fn push_str(&mut self, string: &str) {
        assert!(!string.as_bytes().contains(&0));
        self.0.extend_from_slice(string.as_bytes());
        self.0.push(0);
    }

    /// Add a new key-value pair to the list
    ///
    /// This method does not check duplicate keys. It simply pushes the new pair.
    ///
    /// # Panics
    ///
    /// Panics if any of the arguments contains a `NUL` character.
    /// This could have been handled by returning an error, but for now I chose not to.
    pub fn push_pair(&mut self, key: &str, value: &str) {
        self.push_str(key);
        self.push_str(value);
    }
}

impl From<&TagList> for OwnedTagList {
    /// Copy a TagList's memory into an owned `Vec`
    fn from(tag_list: &TagList) -> Self {
        let bytes = tag_list.as_bytes();
        let mut tag_list = OwnedTagList::new();
        tag_list.0.extend_from_slice(bytes);
        tag_list
    }
}

impl<'a> IntoIterator for &'a OwnedTagList {
    type Item = (&'a str, &'a str);
    type IntoIter = TagIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let begin = self.0.as_ptr();
        let len = self.0.len();
        TagIterator {
            current: begin as *const c_char,
            end: (begin as usize + len) as *const c_char,
            list_lifetime: PhantomData,
        }
    }
}

impl std::fmt::Debug for OwnedTagList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug(self, f)
    }
}

fn debug<'t>(
    tags: impl IntoIterator<Item = (&'t str, &'t str)>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    let mut map = f.debug_map();
    for (key, value) in tags {
        map.key(&key);
        map.value(&value);
    }
    map.finish()
}

/// Nice syntax for creating an [`OwnedTagList`] using literals.
///
/// Mainly useful to test which would work on [`TagList`] with hardcoded data.
///
/// ```
/// use libosmium::tag_list;
///
/// let tags = tag_list! {
///     "amenity": "bar",
///     "name": "Hacker's pub",
///     "opening_hours": "Mo-Su 00:00-06:00,14:00-24:00"
/// };
/// ```
#[macro_export]
macro_rules! tag_list {
    ($($key:literal: $value:literal),* $(,)?) => {
        {
            let string = concat!(
                $($key, "\x00", $value, "\x00"),*
            );
            let mut bytes = Vec::new();
            bytes.extend_from_slice(string.as_bytes());
            $crate::tag_list::OwnedTagList(bytes)
        }
    };
}
