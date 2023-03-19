use super::join::*;
use super::mask::*;
use super::page::*;
use std::iter::Cloned;
use std::slice::{Iter, IterMut};

/// Combines two page iterators (returning Option<IntoMaskIter<>>) to iterate over joined mask
/// iterators
pub struct PageJoinIter<A, B>(A, B);

impl<
        IMA: Iterator<Item = u64>,
        IMB: Iterator<Item = u64>,
        IVA: Iterator,
        IVB: Iterator,
        IA: Iterator<Item = Option<IntoMaskIter<IMA, IVA>>>,
        IB: Iterator<Item = Option<IntoMaskIter<IMB, IVB>>>,
    > Iterator for PageJoinIter<IA, IB>
{
    type Item = Option<IntoMaskIter<BitAndIter<IMA, IMB>, JoinIter<IVA, IVB>>>;
    fn next(&mut self) -> Option<Self::Item> {
        // First, check whether iter is completed
        if let (Some(ma), Some(mb)) = (self.0.next(), self.1.next()) {
            // Next, check whether both values are Some
            let result = {
                if let (Some(ma), Some(mb)) = (ma, mb) {
                    // Here, we have two boxed pages to iterate over
                    Some(ma.join(mb))
                } else {
                    None
                }
            };
            Some(result)
        } else {
            None
        }
    }
}

pub struct PageLeftJoinIter<A, B>(A, B);

/// Combines two page iterators (returning Option<IntoMaskIter<>>) to iterate over joined mask
/// iterators
impl<
        IMA: Iterator<Item = u64>,
        IMB: Iterator<Item = u64>,
        IVA: Iterator,
        IVB: Iterator,
        IA: Iterator<Item = Option<IntoMaskIter<IMA, IVA>>>,
        IB: Iterator<Item = Option<IntoMaskIter<IMB, IVB>>>,
    > Iterator for PageLeftJoinIter<IA, IB>
{
    type Item = Option<IntoMaskIter<IMA, LeftJoinIter<IVA, IMB, IVB>>>;
    fn next(&mut self) -> Option<Self::Item> {
        let mb = self.1.next();
        // Check whether left iter is completed
        if let Some(ma) = self.0.next() {
            // Next, check whether left value is Some
            let result = {
                if let Some(ma) = ma {
                    // Try to get page from rhs
                    let mb = match mb {
                        Some(into_iter) => into_iter,
                        None => None,
                    };
                    Some(ma.left_join(mb))
                } else {
                    None
                }
            };
            Some(result)
        } else {
            None
        }
    }
}

/// Iterator over a slice of mask references, yielding cloned masks
type SliceMaskIter<'a> = Cloned<Iter<'a, u64>>;

/// Iterator over immutable pages, with each page item yielded as an iterator
/// over immutable masked data.
pub struct PageIter<I>(pub I);

impl<'a, T: 'a, I: Iterator<Item = &'a PageOption<T>>> Iterator for PageIter<I> {
    type Item = Option<IntoMaskIter<SliceMaskIter<'a>, Iter<'a, T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.0.next() {
            let result = match entry {
                Some(page) => Some(IntoMaskIter::new(
                    page.masks.iter_masks().cloned(),
                    page.values.iter(),
                )),
                None => None,
            };
            Some(result)
        } else {
            None
        }
    }
}

/// Iterator over mutable pages, with each page item yielded as an iterator
/// over mutable masked data.
pub struct PageIterMut<I>(pub I);

impl<'a, T: 'a, I: Iterator<Item = &'a mut PageOption<T>>> Iterator for PageIterMut<I> {
    type Item = Option<IntoMaskIter<SliceMaskIter<'a>, IterMut<'a, T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.0.next() {
            let result = match entry {
                Some(page) => Some(IntoMaskIter::new(
                    page.masks.iter_masks().cloned(),
                    page.values.iter_mut(),
                )),
                None => None,
            };
            Some(result)
        } else {
            None
        }
    }
}

/// Abstract iterable over pages, allowing joins with other pagess
pub struct PageIntoIter<I> {
    iter: I,
}

impl<I> PageIntoIter<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }

    /// Joins two iterators over pages of mask data, producing a new iterator
    /// over joined pages
    pub fn join<
        MB: Iterator<Item = u64>,
        VB: Iterator,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
    >(
        self,
        rhs: PageIntoIter<IB>,
    ) -> PageIntoIter<PageJoinIter<I, IB>> {
        PageIntoIter {
            iter: PageJoinIter(self.iter, rhs.iter),
        }
    }

    /// Joins two iterators over pages of mask data, producing a new iterator
    /// over joined pages
    pub fn left_join<
        IM: Iterator<Item = u64>,
        IV: Iterator,
        IB: Iterator<Item = Option<IntoMaskIter<IM, IV>>>,
    >(
        self,
        rhs: PageIntoIter<IB>,
    ) -> PageIntoIter<PageLeftJoinIter<I, IB>> {
        PageIntoIter {
            iter: PageLeftJoinIter(self.iter, rhs.iter),
        }
    }
}

impl<I: Iterator> IntoIterator for PageIntoIter<I> {
    type Item = I::Item;
    type IntoIter = I;
    fn into_iter(self) -> I {
        self.iter
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_masks_mut() {
        fn test(m: Vec<u64>) {
            let mut a = vec![1; m.len() * 64];
            let iter = IntoMaskIter::new(m.iter().cloned(), a.iter_mut());
            let mut count = 0;
            for x in iter {
                *x += 1;
                count += 1;
            }
            for i in 0..a.len() {
                let mi = i / 64;
                let bi = i % 64;
                let expected = if (m[mi] >> bi) & 1 != 0 { 2 } else { 1 };
                assert_eq!(a[i], expected);
            }
            let expected: u32 = m.iter().map(|x| x.count_ones()).sum();
            assert_eq!(count, expected);
        }
        test(Vec::new());
        test(vec![0]);
        test(vec![1]);
        test(vec![0, 1]);
        test(vec![u64::MAX]);
        test(vec![1, 0, 0b111111]);
        test(vec![
            0b0,
            0b110101010010,
            0b0,
            0b1111010111,
            0b0,
            0b0,
            u64::MAX,
            u64::MAX,
        ]);
    }

    #[test]
    fn iter_joined() {
        let mut a1 = vec![1; 300];
        let mut a2 = vec![10; 300];
        let m1 = vec![1u64, 1, 1];
        let m2 = vec![1u64, 1, 1];
        let iter1 = IntoMaskIter::new(m1.iter().cloned(), a1.iter_mut());
        let iter2 = IntoMaskIter::new(m2.iter().cloned(), a2.iter_mut());
        for (x, y) in iter1.join(iter2) {
            *x += 1;
            *y += 2;
        }
    }

    #[test]
    fn add_remove_items() {}
}
