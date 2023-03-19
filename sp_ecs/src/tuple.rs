use super::flatten::*;
use super::iter::*;
use super::mask::*;

fn flatten3<A, B, C>(t: ((A, B), C)) -> (A, B, C) {
    (t.0 .0, t.0 .1, t.1)
}

fn flatten4<A, B, C, D>(t: (((A, B), C), D)) -> (A, B, C, D) {
    (t.0 .0 .0, t.0 .0 .1, t.0 .1, t.1)
}

fn flatten5<A, B, C, D, E>(t: ((((A, B), C), D), E)) -> (A, B, C, D, E) {
    (t.0 .0 .0 .0, t.0 .0 .0 .1, t.0 .0 .1, t.0 .1, t.1)
}

fn flatten6<A, B, C, D, E, F>(t: (((((A, B), C), D), E), F)) -> (A, B, C, D, E, F) {
    (
        t.0 .0 .0 .0 .0,
        t.0 .0 .0 .0 .1,
        t.0 .0 .0 .1,
        t.0 .0 .1,
        t.0 .1,
        t.1,
    )
}

fn flatten7<A, B, C, D, E, F, G>(t: ((((((A, B), C), D), E), F), G)) -> (A, B, C, D, E, F, G) {
    (
        t.0 .0 .0 .0 .0 .0,
        t.0 .0 .0 .0 .0 .1,
        t.0 .0 .0 .0 .1,
        t.0 .0 .0 .1,
        t.0 .0 .1,
        t.0 .1,
        t.1,
    )
}

fn flatten8<A, B, C, D, E, F, G, H>(
    t: (((((((A, B), C), D), E), F), G), H),
) -> (A, B, C, D, E, F, G, H) {
    (
        t.0 .0 .0 .0 .0 .0 .0,
        t.0 .0 .0 .0 .0 .0 .1,
        t.0 .0 .0 .0 .0 .1,
        t.0 .0 .0 .0 .1,
        t.0 .0 .0 .1,
        t.0 .0 .1,
        t.0 .1,
        t.1,
    )
}

fn flatten9<A, B, C, D, E, F, G, H, I>(
    t: ((((((((A, B), C), D), E), F), G), H), I),
) -> (A, B, C, D, E, F, G, H, I) {
    (
        t.0 .0 .0 .0 .0 .0 .0 .0,
        t.0 .0 .0 .0 .0 .0 .0 .1,
        t.0 .0 .0 .0 .0 .0 .1,
        t.0 .0 .0 .0 .0 .1,
        t.0 .0 .0 .0 .1,
        t.0 .0 .0 .1,
        t.0 .0 .1,
        t.0 .1,
        t.1,
    )
}

// These wrap iterators yieling nested tuples, mapping each result to
// a flattened tuple. This kind of nesting occurs when repeatedly joining
// tables left to right (versus arbitrary tuple groupings which we don't
// support here).
pub struct FlatIter3<I>(I);
pub struct FlatIter4<I>(I);
pub struct FlatIter5<I>(I);
pub struct FlatIter6<I>(I);
pub struct FlatIter7<I>(I);
pub struct FlatIter8<I>(I);
pub struct FlatIter9<I>(I);

impl<A, B, C, I: Iterator<Item = ((A, B), C)>> Iterator for FlatIter3<I> {
    type Item = (A, B, C);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(flatten3)
    }
}

impl<A, B, C, D, I: Iterator<Item = (((A, B), C), D)>> Iterator for FlatIter4<I> {
    type Item = (A, B, C, D);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(flatten4)
    }
}

impl<A, B, C, D, E, I: Iterator<Item = ((((A, B), C), D), E)>> Iterator for FlatIter5<I> {
    type Item = (A, B, C, D, E);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(flatten5)
    }
}

impl<A, B, C, D, E, F, I: Iterator<Item = (((((A, B), C), D), E), F)>> Iterator for FlatIter6<I> {
    type Item = (A, B, C, D, E, F);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(flatten6)
    }
}

impl<A, B, C, D, E, F, G, I: Iterator<Item = ((((((A, B), C), D), E), F), G)>> Iterator
    for FlatIter7<I>
{
    type Item = (A, B, C, D, E, F, G);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(flatten7)
    }
}

impl<A, B, C, D, E, F, G, H, I: Iterator<Item = (((((((A, B), C), D), E), F), G), H)>> Iterator
    for FlatIter8<I>
{
    type Item = (A, B, C, D, E, F, G, H);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(flatten8)
    }
}

impl<A, B, C, D, E, F, G, H, I, J: Iterator<Item = ((((((((A, B), C), D), E), F), G), H), I)>>
    Iterator for FlatIter9<J>
{
    type Item = (A, B, C, D, E, F, G, H, I);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(flatten9)
    }
}

/// Substitute for IteratorInto since it's already implemented for tuples.
pub trait Join {
    type JoinItem;
    type JoinIter: Iterator<Item = Self::JoinItem>;
    fn join(self) -> Self::JoinIter;
}

impl<
        TA,
        MA: Iterator<Item = u64>,
        VA: Iterator<Item = TA>,
        IA: Iterator<Item = Option<IntoMaskIter<MA, VA>>>,
        TB,
        MB: Iterator<Item = u64>,
        VB: Iterator<Item = TB>,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
    > Join for (FlatPageIntoIter<IA>, FlatPageIntoIter<IB>)
{
    type JoinItem = (TA, TB);
    type JoinIter = <FlatPageIntoIter<PageJoinIter<IA, IB>> as IntoIterator>::IntoIter;
    fn join(self) -> Self::JoinIter {
        self.0.join(self.1).into_iter()
    }
}

impl<
        TA,
        MA: Iterator<Item = u64>,
        VA: Iterator<Item = TA>,
        IA: Iterator<Item = Option<IntoMaskIter<MA, VA>>>,
        TB,
        MB: Iterator<Item = u64>,
        VB: Iterator<Item = TB>,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
        TC,
        MC: Iterator<Item = u64>,
        VC: Iterator<Item = TC>,
        IC: Iterator<Item = Option<IntoMaskIter<MC, VC>>>,
    > Join
    for (
        FlatPageIntoIter<IA>,
        FlatPageIntoIter<IB>,
        FlatPageIntoIter<IC>,
    )
{
    type JoinItem = (TA, TB, TC);
    type JoinIter = FlatIter3<
        <FlatPageIntoIter<PageJoinIter<PageJoinIter<IA, IB>, IC>> as IntoIterator>::IntoIter,
    >;
    fn join(self) -> Self::JoinIter {
        FlatIter3(self.0.join(self.1).join(self.2).into_iter())
    }
}

impl<
        TA,
        MA: Iterator<Item = u64>,
        VA: Iterator<Item = TA>,
        IA: Iterator<Item = Option<IntoMaskIter<MA, VA>>>,
        TB,
        MB: Iterator<Item = u64>,
        VB: Iterator<Item = TB>,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
        TC,
        MC: Iterator<Item = u64>,
        VC: Iterator<Item = TC>,
        IC: Iterator<Item = Option<IntoMaskIter<MC, VC>>>,
        TD,
        MD: Iterator<Item = u64>,
        VD: Iterator<Item = TD>,
        ID: Iterator<Item = Option<IntoMaskIter<MD, VD>>>,
    > Join
    for (
        FlatPageIntoIter<IA>,
        FlatPageIntoIter<IB>,
        FlatPageIntoIter<IC>,
        FlatPageIntoIter<ID>,
    )
{
    type JoinItem = (TA, TB, TC, TD);
    type JoinIter = FlatIter4<<FlatPageIntoIter<PageJoinIter<PageJoinIter<PageJoinIter<IA, IB>, IC>, ID>> as IntoIterator>::IntoIter>;
    fn join(self) -> Self::JoinIter {
        FlatIter4(self.0.join(self.1).join(self.2).join(self.3).into_iter())
    }
}

impl<
        TA,
        MA: Iterator<Item = u64>,
        VA: Iterator<Item = TA>,
        IA: Iterator<Item = Option<IntoMaskIter<MA, VA>>>,
        TB,
        MB: Iterator<Item = u64>,
        VB: Iterator<Item = TB>,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
        TC,
        MC: Iterator<Item = u64>,
        VC: Iterator<Item = TC>,
        IC: Iterator<Item = Option<IntoMaskIter<MC, VC>>>,
        TD,
        MD: Iterator<Item = u64>,
        VD: Iterator<Item = TD>,
        ID: Iterator<Item = Option<IntoMaskIter<MD, VD>>>,
        TE,
        ME: Iterator<Item = u64>,
        VE: Iterator<Item = TE>,
        IE: Iterator<Item = Option<IntoMaskIter<ME, VE>>>,
    > Join
    for (
        FlatPageIntoIter<IA>,
        FlatPageIntoIter<IB>,
        FlatPageIntoIter<IC>,
        FlatPageIntoIter<ID>,
        FlatPageIntoIter<IE>,
    )
{
    type JoinItem = (TA, TB, TC, TD, TE);
    type JoinIter = FlatIter5<
        <FlatPageIntoIter<
            PageJoinIter<PageJoinIter<PageJoinIter<PageJoinIter<IA, IB>, IC>, ID>, IE>,
        > as IntoIterator>::IntoIter,
    >;
    fn join(self) -> Self::JoinIter {
        FlatIter5(
            self.0
                .join(self.1)
                .join(self.2)
                .join(self.3)
                .join(self.4)
                .into_iter(),
        )
    }
}

impl<
        TA,
        MA: Iterator<Item = u64>,
        VA: Iterator<Item = TA>,
        IA: Iterator<Item = Option<IntoMaskIter<MA, VA>>>,
        TB,
        MB: Iterator<Item = u64>,
        VB: Iterator<Item = TB>,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
        TC,
        MC: Iterator<Item = u64>,
        VC: Iterator<Item = TC>,
        IC: Iterator<Item = Option<IntoMaskIter<MC, VC>>>,
        TD,
        MD: Iterator<Item = u64>,
        VD: Iterator<Item = TD>,
        ID: Iterator<Item = Option<IntoMaskIter<MD, VD>>>,
        TE,
        ME: Iterator<Item = u64>,
        VE: Iterator<Item = TE>,
        IE: Iterator<Item = Option<IntoMaskIter<ME, VE>>>,
        TF,
        MF: Iterator<Item = u64>,
        VF: Iterator<Item = TF>,
        IF: Iterator<Item = Option<IntoMaskIter<MF, VF>>>,
    > Join
    for (
        FlatPageIntoIter<IA>,
        FlatPageIntoIter<IB>,
        FlatPageIntoIter<IC>,
        FlatPageIntoIter<ID>,
        FlatPageIntoIter<IE>,
        FlatPageIntoIter<IF>,
    )
{
    type JoinItem = (TA, TB, TC, TD, TE, TF);
    type JoinIter = FlatIter6<
        <FlatPageIntoIter<
            PageJoinIter<
                PageJoinIter<PageJoinIter<PageJoinIter<PageJoinIter<IA, IB>, IC>, ID>, IE>,
                IF,
            >,
        > as IntoIterator>::IntoIter,
    >;
    fn join(self) -> Self::JoinIter {
        FlatIter6(
            self.0
                .join(self.1)
                .join(self.2)
                .join(self.3)
                .join(self.4)
                .join(self.5)
                .into_iter(),
        )
    }
}

impl<
        TA,
        MA: Iterator<Item = u64>,
        VA: Iterator<Item = TA>,
        IA: Iterator<Item = Option<IntoMaskIter<MA, VA>>>,
        TB,
        MB: Iterator<Item = u64>,
        VB: Iterator<Item = TB>,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
        TC,
        MC: Iterator<Item = u64>,
        VC: Iterator<Item = TC>,
        IC: Iterator<Item = Option<IntoMaskIter<MC, VC>>>,
        TD,
        MD: Iterator<Item = u64>,
        VD: Iterator<Item = TD>,
        ID: Iterator<Item = Option<IntoMaskIter<MD, VD>>>,
        TE,
        ME: Iterator<Item = u64>,
        VE: Iterator<Item = TE>,
        IE: Iterator<Item = Option<IntoMaskIter<ME, VE>>>,
        TF,
        MF: Iterator<Item = u64>,
        VF: Iterator<Item = TF>,
        IF: Iterator<Item = Option<IntoMaskIter<MF, VF>>>,
        TG,
        MG: Iterator<Item = u64>,
        VG: Iterator<Item = TG>,
        IG: Iterator<Item = Option<IntoMaskIter<MG, VG>>>,
    > Join
    for (
        FlatPageIntoIter<IA>,
        FlatPageIntoIter<IB>,
        FlatPageIntoIter<IC>,
        FlatPageIntoIter<ID>,
        FlatPageIntoIter<IE>,
        FlatPageIntoIter<IF>,
        FlatPageIntoIter<IG>,
    )
{
    type JoinItem = (TA, TB, TC, TD, TE, TF, TG);
    type JoinIter = FlatIter7<
        <FlatPageIntoIter<
            PageJoinIter<
                PageJoinIter<
                    PageJoinIter<PageJoinIter<PageJoinIter<PageJoinIter<IA, IB>, IC>, ID>, IE>,
                    IF,
                >,
                IG,
            >,
        > as IntoIterator>::IntoIter,
    >;
    fn join(self) -> Self::JoinIter {
        FlatIter7(
            self.0
                .join(self.1)
                .join(self.2)
                .join(self.3)
                .join(self.4)
                .join(self.5)
                .join(self.6)
                .into_iter(),
        )
    }
}

impl<
        TA,
        MA: Iterator<Item = u64>,
        VA: Iterator<Item = TA>,
        IA: Iterator<Item = Option<IntoMaskIter<MA, VA>>>,
        TB,
        MB: Iterator<Item = u64>,
        VB: Iterator<Item = TB>,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
        TC,
        MC: Iterator<Item = u64>,
        VC: Iterator<Item = TC>,
        IC: Iterator<Item = Option<IntoMaskIter<MC, VC>>>,
        TD,
        MD: Iterator<Item = u64>,
        VD: Iterator<Item = TD>,
        ID: Iterator<Item = Option<IntoMaskIter<MD, VD>>>,
        TE,
        ME: Iterator<Item = u64>,
        VE: Iterator<Item = TE>,
        IE: Iterator<Item = Option<IntoMaskIter<ME, VE>>>,
        TF,
        MF: Iterator<Item = u64>,
        VF: Iterator<Item = TF>,
        IF: Iterator<Item = Option<IntoMaskIter<MF, VF>>>,
        TG,
        MG: Iterator<Item = u64>,
        VG: Iterator<Item = TG>,
        IG: Iterator<Item = Option<IntoMaskIter<MG, VG>>>,
        TH,
        MH: Iterator<Item = u64>,
        VH: Iterator<Item = TH>,
        IH: Iterator<Item = Option<IntoMaskIter<MH, VH>>>,
    > Join
    for (
        FlatPageIntoIter<IA>,
        FlatPageIntoIter<IB>,
        FlatPageIntoIter<IC>,
        FlatPageIntoIter<ID>,
        FlatPageIntoIter<IE>,
        FlatPageIntoIter<IF>,
        FlatPageIntoIter<IG>,
        FlatPageIntoIter<IH>,
    )
{
    type JoinItem = (TA, TB, TC, TD, TE, TF, TG, TH);
    type JoinIter = FlatIter8<
        <FlatPageIntoIter<
            PageJoinIter<
                PageJoinIter<
                    PageJoinIter<
                        PageJoinIter<PageJoinIter<PageJoinIter<PageJoinIter<IA, IB>, IC>, ID>, IE>,
                        IF,
                    >,
                    IG,
                >,
                IH,
            >,
        > as IntoIterator>::IntoIter,
    >;
    fn join(self) -> Self::JoinIter {
        FlatIter8(
            self.0
                .join(self.1)
                .join(self.2)
                .join(self.3)
                .join(self.4)
                .join(self.5)
                .join(self.6)
                .join(self.7)
                .into_iter(),
        )
    }
}

impl<
        TA,
        MA: Iterator<Item = u64>,
        VA: Iterator<Item = TA>,
        IA: Iterator<Item = Option<IntoMaskIter<MA, VA>>>,
        TB,
        MB: Iterator<Item = u64>,
        VB: Iterator<Item = TB>,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
        TC,
        MC: Iterator<Item = u64>,
        VC: Iterator<Item = TC>,
        IC: Iterator<Item = Option<IntoMaskIter<MC, VC>>>,
        TD,
        MD: Iterator<Item = u64>,
        VD: Iterator<Item = TD>,
        ID: Iterator<Item = Option<IntoMaskIter<MD, VD>>>,
        TE,
        ME: Iterator<Item = u64>,
        VE: Iterator<Item = TE>,
        IE: Iterator<Item = Option<IntoMaskIter<ME, VE>>>,
        TF,
        MF: Iterator<Item = u64>,
        VF: Iterator<Item = TF>,
        IF: Iterator<Item = Option<IntoMaskIter<MF, VF>>>,
        TG,
        MG: Iterator<Item = u64>,
        VG: Iterator<Item = TG>,
        IG: Iterator<Item = Option<IntoMaskIter<MG, VG>>>,
        TH,
        MH: Iterator<Item = u64>,
        VH: Iterator<Item = TH>,
        IH: Iterator<Item = Option<IntoMaskIter<MH, VH>>>,
        TI,
        MI: Iterator<Item = u64>,
        VI: Iterator<Item = TI>,
        II: Iterator<Item = Option<IntoMaskIter<MI, VI>>>,
    > Join
    for (
        FlatPageIntoIter<IA>,
        FlatPageIntoIter<IB>,
        FlatPageIntoIter<IC>,
        FlatPageIntoIter<ID>,
        FlatPageIntoIter<IE>,
        FlatPageIntoIter<IF>,
        FlatPageIntoIter<IG>,
        FlatPageIntoIter<IH>,
        FlatPageIntoIter<II>,
    )
{
    type JoinItem = (TA, TB, TC, TD, TE, TF, TG, TH, TI);
    type JoinIter = FlatIter9<
        <FlatPageIntoIter<
            PageJoinIter<
                PageJoinIter<
                    PageJoinIter<
                        PageJoinIter<
                            PageJoinIter<
                                PageJoinIter<PageJoinIter<PageJoinIter<IA, IB>, IC>, ID>,
                                IE,
                            >,
                            IF,
                        >,
                        IG,
                    >,
                    IH,
                >,
                II,
            >,
        > as IntoIterator>::IntoIter,
    >;
    fn join(self) -> Self::JoinIter {
        FlatIter9(
            self.0
                .join(self.1)
                .join(self.2)
                .join(self.3)
                .join(self.4)
                .join(self.5)
                .join(self.6)
                .join(self.7)
                .join(self.8)
                .into_iter(),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::Eid;

    use super::super::table::*;
    use super::*;

    #[derive(Default)]
    struct World {
        a: Table<i32>,
        b: Table<i32>,
        c: Table<i32>,
    }

    fn create() -> World {
        let mut w = World::default();
        w.a.add(Eid(9), 10);
        w.a.add(Eid(10), 20);
        w.b.add(Eid(10), 100);
        w.b.add(Eid(11), 200);
        w.c.add(Eid(10), 1000);
        w.c.add(Eid(12), 2000);
        w
    }

    #[test]
    fn join_tables_tuple2() {
        let mut w = create();
        for (a, b) in (w.a.iter_mut(), w.b.iter()).join() {
            *a += b;
        }
        let items: Vec<i32> = w.a.iter().into_iter().cloned().collect();
        assert_eq!(&items, &[10, 120]);
    }

    #[test]
    fn join_tables_tuple3() {
        let mut w = create();
        for (a, b, c) in (w.a.iter_mut(), w.b.iter(), w.c.iter()).join() {
            *a += b + c;
        }
        let items: Vec<i32> = w.a.iter().into_iter().cloned().collect();
        assert_eq!(&items, &[10, 1120]);
    }

    #[test]
    fn join_tables_nested_tuples3() {
        let mut w = create();
        for ((a, b), c) in w.a.iter_mut().join(w.b.iter()).join(w.c.iter()) {
            *a += b + c;
        }
        let items: Vec<i32> = w.a.iter().into_iter().cloned().collect();
        assert_eq!(&items, &[10, 1120]);
    }
}
