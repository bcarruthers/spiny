/// This should be enhanced to support nth()
/// Iterator over mask and value iterators
pub struct MaskIter<IM, IV> {
    mask: u64,
    rem: usize,
    masks: IM,
    values: IV,
}

impl<IM, IV> MaskIter<IM, IV> {
    pub fn new(masks: IM, values: IV) -> Self {
        Self {
            mask: 0,
            rem: 0,
            masks,
            values,
        }
    }
}

impl<M: Into<u64>, IM: Iterator<Item = M>, IV: Iterator> Iterator for MaskIter<IM, IV> {
    type Item = IV::Item;

    // This calls nth() to skip over zero bits. For vec iterator, nth() is
    // a single seek operation, but for others it may involve repeatedly calling next().
    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 {
            // Try to scan to next nonzero mask
            while let Some(mask) = self.masks.next() {
                let mask: u64 = mask.into();
                // Found populated mask
                if mask != 0 {
                    // Advance to first bit
                    let i = mask.trailing_zeros() as usize;
                    let skip = self.rem + i;
                    // Doing two shifts to avoid overflow when 63 trailing zeros
                    self.mask = (mask >> i) >> 1;
                    self.rem = 63 - i;
                    return self.values.nth(skip);
                }
                self.rem += 64;
            }
            // If none remain
            if self.mask == 0 {
                return None;
            }
        }
        // Find lowest set bit
        let i = self.mask.trailing_zeros() as usize;
        let adv = i + 1;
        self.mask >>= adv;
        self.rem -= adv;
        self.values.nth(i)
    }
}

pub struct BitAndIter<A, B>(pub A, pub B);

impl<A: Iterator<Item = u64>, B: Iterator<Item = u64>> Iterator for BitAndIter<A, B> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if let (Some(ma), Some(mb)) = (self.0.next(), self.1.next()) {
            Some(ma & mb)
        } else {
            None
        }
    }
}

/// This should be enhanced to support nth()
/// This is really only needed if there are more mask bits than values, meaning the
/// value iterator ends early. Otherwise, Bitand is sufficient.
pub struct JoinIter<A, B>(pub A, pub B);

impl<A: Iterator, B: Iterator> Iterator for JoinIter<A, B> {
    type Item = (A::Item, B::Item);
    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(ma), Some(mb)) = (self.0.next(), self.1.next()) {
            Some((ma, mb))
        } else {
            None
        }
    }
}

/// This should be enhanced to support nth()
/// Assume lhs masks are already iterated separately (via MaskIter), and this
/// iterator just needs to progress through lhs values and rhs masks/values.
pub struct LeftJoinIter<IA, IMB, IB> {
    mask: u64,
    i: usize,
    lhs_values: IA,
    rhs: Option<(IMB, IB)>,
}

impl<IA, IMB, IB> LeftJoinIter<IA, IMB, IB> {
    pub fn new(lhs_values: IA, rhs: Option<(IMB, IB)>) -> Self {
        Self {
            mask: 0,
            i: 0,
            lhs_values,
            rhs,
        }
    }
}

impl<IA: Iterator, IMB: Iterator<Item = u64>, IB: Iterator> Iterator for LeftJoinIter<IA, IMB, IB> {
    type Item = (IA::Item, Option<IB::Item>);
    fn next(&mut self) -> Option<Self::Item> {
        // First, try to advance in lhs, halting entire iteration if there
        // are no remaining lhs values.
        if let Some(ma) = self.lhs_values.next() {
            let mb = if let Some((rhs_masks, rhs_values)) = &mut self.rhs {
                if self.i % 64 == 0 {
                    // Get the next mask, or use empty mask if mask iter is complete
                    self.mask = rhs_masks.next().unwrap_or(0);
                } else {
                    self.mask >>= 1;
                }
                self.i += 1;
                // Get next rhs value
                let mb = rhs_values.next();
                // Discard the value if masked out
                if self.mask & 1 != 0 {
                    mb
                } else {
                    None
                }
            } else {
                None
            };
            Some((ma, mb))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_masks(expected: i32, masks: &[u64]) {
        let values = 1i32..1000;
        let sum: i32 = MaskIter::new(
            masks.into_iter().cloned().collect::<Vec<_>>().into_iter(),
            values.into_iter(),
        )
        .sum();
        assert_eq!(sum, expected);
    }

    #[test]
    fn iter_masks() {
        test_masks(0, &[0]);
        test_masks(1, &[1]);
        test_masks(64, &[1 << 63]);
        test_masks(18727, &[0b111u64, 0b1, 1 << 63, 0, u64::MAX]);
    }
}
