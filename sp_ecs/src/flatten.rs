use super::iter::*;
use super::mask::*;
use std::iter::Flatten;

// Wrap PageIntoIter with the intent of flattening iteration over pages and options (2x flatten),
// and also map the resulting nested tuple to a flattened tuple.
pub struct FlatPageIntoIter<I> {
    into_iter: PageIntoIter<I>,
}

impl<I> FlatPageIntoIter<I> {
    pub fn new(iter: I) -> Self {
        Self {
            into_iter: PageIntoIter::new(iter),
        }
    }

    pub fn join<
        MB: Iterator<Item = u64>,
        VB: Iterator,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
    >(
        self,
        rhs: FlatPageIntoIter<IB>,
    ) -> FlatPageIntoIter<PageJoinIter<I, IB>> {
        FlatPageIntoIter {
            into_iter: self.into_iter.join(rhs.into_iter),
        }
    }

    pub fn left_join<
        MB: Iterator<Item = u64>,
        VB: Iterator,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
    >(
        self,
        rhs: FlatPageIntoIter<IB>,
    ) -> FlatPageIntoIter<PageLeftJoinIter<I, IB>> {
        FlatPageIntoIter {
            into_iter: self.into_iter.left_join(rhs.into_iter),
        }
    }
}

impl<M: Iterator<Item = u64>, V: Iterator, I: Iterator<Item = Option<IntoMaskIter<M, V>>>>
    IntoIterator for FlatPageIntoIter<I>
{
    type Item = V::Item;
    type IntoIter = Flatten<Flatten<I>>;
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter.into_iter().flatten().flatten()
    }
}

impl<
        MA: Iterator<Item = u64>,
        VA: Iterator,
        IA: Iterator<Item = Option<IntoMaskIter<MA, VA>>>,
        MB: Iterator<Item = u64>,
        VB: Iterator,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
    > std::ops::BitAnd<FlatPageIntoIter<IB>> for FlatPageIntoIter<IA>
{
    type Output = FlatPageIntoIter<PageJoinIter<IA, IB>>;

    fn bitand(self, rhs: FlatPageIntoIter<IB>) -> Self::Output {
        self.join(rhs)
    }
}
