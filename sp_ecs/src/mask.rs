use std::slice::{Iter, IterMut};

use serde::{Serialize, Deserialize};

use super::join::*;

pub const MASK_SIZE_POW: u32 = 6;
pub const MASK_SIZE: u32 = 1 << MASK_SIZE_POW;
pub const MASK_MASK: u32 = MASK_SIZE - 1;

pub struct BitArray<const SIZE: usize> {
    masks: [u64; SIZE],
}

impl<const SIZE: usize> BitArray<SIZE> {
    pub fn new() -> Self {
        Self {
            masks: [0u64; SIZE],
        }
    }

    /// Number of set bits
    pub fn count(&self) -> usize {
        self.masks.iter().map(|m| m.count_ones()).sum::<u32>() as usize
    }

    pub fn is_empty(&self) -> bool {
        !self.masks.iter().any(|m| *m != 0)
    }

    pub fn add_mask(&mut self, i: usize, mask: u64) {
        self.masks[i] |= mask;
    }

    pub fn remove_mask(&mut self, i: usize, mask: u64) -> u64 {
        let current = &mut self.masks[i];
        let new_mask = *current & !mask;
        let removed = new_mask ^ new_mask;
        *current = new_mask;
        removed
    }

    pub fn get_mask(&self, i: usize) -> u64 {
        self.masks[i]
    }

    pub fn get_mask_mut(&mut self, i: usize) -> &mut u64 {
        &mut self.masks[i]
    }

    pub fn iter_masks(&self) -> Iter<u64> {
        self.masks.iter()
    }

    pub fn iter_masks_mut(&mut self) -> IterMut<u64> {
        self.masks.iter_mut()
    }

    pub fn add(&mut self, i: usize) -> bool {
        let mask_index = i >> MASK_SIZE_POW;
        let bit_index = i & MASK_MASK as usize;
        let mask = &mut self.masks[mask_index];
        let new_mask = *mask | (1 << bit_index);
        let added = *mask != new_mask;
        *mask = new_mask;
        added
    }

    pub fn remove(&mut self, i: usize) -> bool {
        let mask_index = i >> MASK_SIZE_POW;
        let bit_index = i & MASK_MASK as usize;
        let mask = &mut self.masks[mask_index];
        let new_mask = *mask & !(1 << bit_index);
        let removed = *mask != new_mask;
        *mask = new_mask;
        removed
    }

    pub fn get(&self, i: usize) -> bool {
        let mask_index = i >> MASK_SIZE_POW;
        let bit_index = i & MASK_MASK as usize;
        let mask = self.masks[mask_index];
        let bit = (mask >> bit_index) & 1;
        bit != 0
    }

    pub fn clear(&mut self) {
        self.masks.fill(0);
    }
}

/// Abstract iterable over masked data. The reason of using this instead of
/// directly creating an iterator is to abstract over mutability so we can
/// use this as a primitive for joining.
pub struct IntoMaskIter<M, V> {
    pub masks: M,
    pub values: V,
}

impl<M, V> IntoMaskIter<M, V> {
    pub fn new(masks: M, values: V) -> Self {
        Self { masks, values }
    }

    pub fn join<MB: Iterator<Item = u64>, VB: Iterator>(
        self,
        rhs: IntoMaskIter<MB, VB>,
    ) -> IntoMaskIter<BitAndIter<M, MB>, JoinIter<V, VB>> {
        IntoMaskIter {
            // Masks are joined independently from values, but value join is
            // actually redundant since mask should already have checked for
            // presence, assuming masks and values are consistent.
            masks: BitAndIter(self.masks, rhs.masks),
            values: JoinIter(self.values, rhs.values),
        }
    }

    pub fn left_join<IM: Iterator<Item = u64>, IV: Iterator>(
        self,
        rhs: Option<IntoMaskIter<IM, IV>>,
    ) -> IntoMaskIter<M, LeftJoinIter<V, IM, IV>> {
        IntoMaskIter {
            masks: self.masks,
            values: LeftJoinIter::new(self.values, rhs.map(|rhs| (rhs.masks, rhs.values))),
        }
    }
}

impl<M: Iterator<Item = u64>, V: Iterator> IntoIterator for IntoMaskIter<M, V> {
    type Item = V::Item;
    type IntoIter = MaskIter<M, V>;
    fn into_iter(self) -> MaskIter<M, V> {
        MaskIter::new(self.masks, self.values)
    }
}

/// Unbounded vec of masked values supported random access.
#[derive(Default)]
pub struct MaskedVec<T> {
    pub masks: Vec<u64>,
    pub values: Vec<T>,
}

impl<T> MaskedVec<T> {
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        let mask_index = i >> MASK_SIZE_POW;
        let bit_index = i & MASK_MASK as usize;
        let mask = self.masks[mask_index];
        let bit = (mask >> bit_index) & 1;
        if bit != 0 {
            Some(&self.values[i])
        } else {
            None
        }
    }

    pub fn push_some(&mut self, value: T) {
        let i = self.values.len();
        let mi = i / 64;
        self.values.push(value);
        if self.masks.len() <= mi {
            self.masks.push(1);
        } else {
            let bi = i % 64;
            self.masks[mi] |= 1 << bi;
        }
    }

    pub fn clear(&mut self) {
        self.masks.clear();
        self.values.clear();
    }

    pub fn iter(&self) -> impl IntoIterator<Item = &T> {
        IntoMaskIter::new(self.masks.iter().cloned(), self.values.iter())
    }

    pub fn iter_mut(&mut self) -> impl IntoIterator<Item = &mut T> {
        IntoMaskIter::new(self.masks.iter().cloned(), self.values.iter_mut())
    }
}

impl<T: Default> MaskedVec<T> {
    pub fn push_none(&mut self) {
        let i = self.values.len();
        let mi = i / 64;
        self.values.push(Default::default());
        if self.masks.len() <= mi {
            self.masks.push(0);
        }
    }

    pub fn push(&mut self, value: Option<T>) {
        match value {
            Some(v) => self.push_some(v),
            None => self.push_none(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitStream {
    len: usize,
    masks: Vec<u64>,
}

impl BitStream {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn count_ones(&self) -> usize {
        self.masks.iter().map(|m| m.count_ones() as usize).sum()
    }

    pub fn get(&self, i: usize) -> bool {
        let mask_index = i >> MASK_SIZE_POW;
        let bit_index = i & MASK_MASK as usize;
        let mask = self.masks[mask_index];
        let bit = (mask >> bit_index) & 1;
        bit != 0
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.masks.clear();
    }

    pub fn push_true(&mut self) {
        let bi = self.len % 64;
        if bi == 0 {
            self.masks.push(1);
        } else {
            self.masks[self.len / 64] |= 1 << bi;
        }
        self.len += 1;
    }

    pub fn push_false(&mut self) {
        if self.len % 64 == 0 {
            self.masks.push(0);
        }
        self.len += 1;
    }

    pub fn push(&mut self, value: bool) {
        if value {
            self.push_true()
        } else {
            self.push_false()
        }
    }
}
