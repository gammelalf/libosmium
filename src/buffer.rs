use crate::item::Item;

/// Buffer to clone OSM items into
pub struct ItemBuffer {
    buffer: Vec<u8>,
}

impl ItemBuffer {
    /// Construct a new, empty buffer.
    pub const fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    /// Constructs a new, empty buffer with at least the specified capacity.
    ///
    /// Note the capacity is in bytes not number of items, since they are all dynamically sized.
    pub fn with_capacity(size: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(size),
        }
    }

    /// Appends an item to the back of the buffer.
    pub fn push(&mut self, item: &impl AsRef<Item>) {
        self.buffer.extend_from_slice(item.as_ref().as_bytes());
    }

    /// Returns an iterator over the buffer.
    ///
    /// The iterator yields all items from start to end.
    pub fn iter(&self) -> ItemBufferIterator {
        ItemBufferIterator {
            buffer: self,
            index: 0,
        }
    }

    /// Clears the whole buffer, removing all items.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    pub fn clear(&mut self) {
        self.buffer.clear()
    }
}

/// Immutable [ItemBuffer] iterator
///
/// This struct is created by the [`iter`](ItemBuffer::iter) method.
pub struct ItemBufferIterator<'b> {
    buffer: &'b ItemBuffer,
    index: usize,
}

impl<'b> Iterator for ItemBufferIterator<'b> {
    type Item = &'b Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item: &u8 = self.buffer.buffer.get(self.index)?;
        let item: &Item = unsafe {std::mem::transmute(item)};
        self.index += item.aligned_size();
        Some(item)
    }
}

impl<'b> IntoIterator for &'b ItemBuffer {
    type Item = &'b Item;
    type IntoIter = ItemBufferIterator<'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}