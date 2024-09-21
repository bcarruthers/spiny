use super::mask::*;

// 16 masks * 64 bits = 1024 components per page
pub const PAGE_MASK_SIZE_POW: u32 = 4;
pub const PAGE_MASK_COUNT: usize = 1 << PAGE_MASK_SIZE_POW;
pub const PAGE_SIZE_POW: u32 = PAGE_MASK_SIZE_POW + MASK_SIZE_POW;
pub const PAGE_SIZE: u32 = 1 << PAGE_SIZE_POW;
pub const PAGE_MASK: u32 = PAGE_SIZE - 1;

/// Fixed size page storing masked values. Vec is used instead of array
/// to avoid stack overflow for large components (array would be allocated
/// on the stack first).
pub struct Page<T> {
    pub masks: BitArray<PAGE_MASK_COUNT>,
    pub values: Vec<T>,
}

impl<T> Page<T> {
    /// Number of defined values present
    pub fn len(&self) -> usize {
        self.masks.count()
    }

    pub fn is_empty(&self) -> bool {
        self.masks.is_empty()
    }

    pub fn get_mask(&self, i: usize) -> u64 {
        self.masks.get_mask(i)
    }

    pub fn contains(&self, i: usize) -> bool {
        self.masks.get(i)
    }

    pub fn get(&self, i: usize) -> &T {
        assert!(self.contains(i));
        &self.values[i]
    }

    pub fn get_mut(&mut self, i: usize) -> &mut T {
        assert!(self.contains(i));
        &mut self.values[i]
    }

    pub fn try_get(&self, i: usize) -> Option<&T> {
        if self.masks.get(i) {
            Some(&self.values[i])
        } else {
            None
        }
    }

    pub fn try_get_mut(&mut self, i: usize) -> Option<&mut T> {
        if self.masks.get(i) {
            Some(&mut self.values[i])
        } else {
            None
        }
    }

    /// Adds only if not already present
    pub fn try_add(&mut self, i: usize, value: T) -> bool {
        let added = self.masks.add(i);
        if added {
            self.values[i] = value;
        }
        added
    }

    /// Returns added value if not already present
    pub fn try_add_mut(&mut self, i: usize) -> Option<&mut T> {
        let added = self.masks.add(i);
        if added { Some(&mut self.values[i]) } else { None }
    }

    pub fn get_or_add_mut(&mut self, i: usize) -> &mut T {
        self.masks.add(i);
        &mut self.values[i]
    }

    /// Adds or sets value, overwriting any existing
    pub fn add(&mut self, i: usize, value: T) {
        self.masks.add(i);
        self.values[i] = value;
    }

    pub fn iter(&self) -> impl IntoIterator<Item = &T> {
        IntoMaskIter::new(self.masks.iter_masks().cloned(), self.values.iter())
    }

    pub fn iter_mut(&mut self) -> impl IntoIterator<Item = &mut T> {
        IntoMaskIter::new(self.masks.iter_masks().cloned(), self.values.iter_mut())
    }
}

impl<T: Default> Page<T> {
    pub fn new() -> Self {
        let mut values = Vec::with_capacity(PAGE_SIZE as usize);
        for _ in 0..PAGE_SIZE {
            values.push(T::default());
        }
        Self {
            masks: BitArray::new(),
            values,
        }
    }

    pub fn clear(&mut self) {
        self.masks.clear();
        for i in 0..PAGE_SIZE as usize {
            self.values[i] = Default::default();
        }
    }
    
    /// Returns true if the element was removed
    pub fn remove(&mut self, i: usize) -> Option<T> {
        let removed = self.masks.remove(i);
        if removed {
            Some(std::mem::take(&mut self.values[i]))
        } else {
            None
        }
    }

    /// Returns mask with bits set indicating each element removed
    pub fn remove_mask(&mut self, i: usize, mask: u64) -> u64 {
        let removed = self.masks.remove_mask(i, mask);
        let mut mask = removed;
        while mask != 0 {
            let t = mask & mask.wrapping_neg();
            let i = mask.trailing_zeros() as usize;
            self.values[i] = Default::default();
            mask ^= t;
        }
        removed
    }

    /// Iterates over masks, removing elements when bits are set
    pub fn remove_mask_iter<I: Iterator<Item = u64>>(&mut self, iter: I) {
        let mut i = 0;
        for mask in iter {
            self.remove_mask(i, mask);
            i += 1;
        }
    }
}

pub type PageOption<T> = Option<Box<Page<T>>>;
