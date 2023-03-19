use serde::{Serialize, Deserialize};

use crate::Eid;

use super::{mask::*, table::*};

/// Utility for iterating over entities that were modified
pub fn iter_modified<'a, Id: Clone>(
    eids: &'a Table<Id>,
    any_mod: &'a Table<()>,
) -> impl Iterator<Item = Id> + 'a {
    eids.iter()
        .join(any_mod.iter())
        .into_iter()
        .map(|(id, _any_mod)| id.clone())
}

/// Compact stream to replicate components
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaskedStream<T> {
    masks: BitStream,
    values: Vec<T>,
}

impl<T> MaskedStream<T> {
    pub fn new() -> Self {
        Self {
            masks: BitStream::new(),
            values: Vec::new(),
        }
    }

    pub fn present_len(&self) -> usize {
        self.masks.count_ones()
    }

    pub fn value_len(&self) -> usize {
        self.values.len()
    }

    pub fn is_present(&self, present_index: usize) -> bool {
        self.masks.get(present_index)
    }

    pub fn get_value(&self, value_index: usize) -> &T {
        &self.values[value_index]
    }

    pub fn push_some(&mut self, value: T) {
        self.masks.push_true();
        self.values.push(value);
    }

    pub fn push_none(&mut self) {
        self.masks.push_false();
    }

    pub fn push(&mut self, value: Option<T>) {
        match value {
            Some(v) => self.push_some(v),
            None => self.push_none(),
        }
    }

    pub fn clear(&mut self) {
        self.masks.clear();
        self.values.clear();
    }
}

/// Compact stream to replicate component changes
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeltaStream<T> {
    modified: BitStream,
    values: MaskedStream<T>,
}

impl<T> DeltaStream<T> {
    pub fn new() -> Self {
        Self {
            modified: BitStream::new(),
            values: MaskedStream::new(),
        }
    }

    pub fn modified_len(&self) -> usize {
        self.modified.count_ones()
    }

    pub fn present_len(&self) -> usize {
        self.values.present_len()
    }

    pub fn value_len(&self) -> usize {
        self.values.value_len()
    }

    pub fn is_modified(&self, mod_index: usize) -> bool {
        self.modified.get(mod_index)
    }

    pub fn is_present(&self, present_index: usize) -> bool {
        self.values.is_present(present_index)
    }

    pub fn get_value(&self, value_index: usize) -> &T {
        self.values.get_value(value_index)
    }

    pub fn push_unmodified(&mut self) {
        self.modified.push_false();
    }

    pub fn push_modified_some(&mut self, value: T) {
        self.values.push_some(value);
    }

    pub fn push_modified_none(&mut self) {
        self.values.push_none();
    }

    pub fn push_modified(&mut self, value: Option<T>) {
        self.modified.push_true();
        self.values.push(value);
    }

    pub fn clear(&mut self) {
        self.modified.clear();
        self.values.clear();
    }
}

impl<T: Default + Copy> DeltaStream<T> {
    pub fn apply_to<W: WriteTable<T>>(&self, eids: &Vec<Eid>, dest: &mut W) {
        let mut mi = 0;
        let mut pi = 0;
        let mut vi = 0;
        while mi < eids.len() {
            if self.is_modified(mi) {
                let eid = eids[mi].clone();
                if self.is_present(pi) {
                    let value = self.get_value(vi);
                    dest.add(eid, value.clone());
                    vi += 1;
                } else {
                    dest.remove(eid);
                }
                pi += 1;
            }
            mi += 1;
        }
    }
}

/// Tracks modified state of each value
#[derive(Default)]
pub struct DeltaTable<T> {
    pub modified: Table<()>,
    pub values: Table<T>,
}

impl<T> DeltaTable<T> {
    pub fn new() -> Self {
        Self {
            modified: Table::new(),
            values: Table::new(),
        }
    }
}

impl<T: Default> DeltaTable<T> {
    pub fn clear(&mut self) {
        self.modified.clear();
        self.values.clear();
    }
}

impl<T: Default + Clone + Copy + PartialEq> DeltaTable<T> {
    pub fn remove(&mut self, id: Eid) {
        self.modified.remove(id.clone());
        self.values.remove(id);
    }

    pub fn write(&mut self, id: Eid, new_value: Option<&T>) -> bool {
        let dest_value = self.values.try_get_mut(id);
        // Compare new and old value
        let modified = match (dest_value, new_value) {
            (Some(dest_value), Some(new_value)) => {
                // Both are present, check whether value changed
                let modified = dest_value != new_value;
                if modified {
                    *dest_value = new_value.clone();
                }
                modified
            }
            (Some(_), None) => {
                self.values.remove(id);
                true
            }
            (None, Some(new_value)) => {
                self.values.add(id, new_value.clone());
                true
            }
            (None, None) => false,
        };
        if modified {
            self.modified.add(id, ());
        }
        modified
    }

    /// Input is a table of entities with any component modified, not necessarily
    /// the one here.
    pub fn flush(&mut self, modified: &Table<()>, output: &mut DeltaStream<T>) {
        // The result of this:
        // - Bit for any modified
        // - For each modified bit set, bit for value present or not
        // - For each present bit set, value
        // Two levels of nesting:
        // - Left join: component modified and component
        // - Left join again: any modified and inner left join result
        for (_any_mod, rhs) in modified
            .iter()
            .left_join(self.modified.iter().left_join(self.values.iter()))
        {
            match rhs {
                Some((_comp_mod, value)) => output.push_modified(value.cloned()),
                None => output.push_unmodified(),
            }
        }
        self.modified.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Eid;

    #[derive(Debug, PartialEq, Default, Clone, Copy)]
    pub struct Cmp {
        pub data: [u16; 32],
    }

    #[derive(Default)]
    struct Tables {
        eids: Table<Eid>,
        values: Table<char>,
    }

    #[derive(Default)]
    struct Test {
        src: Tables,
        any_mod: Table<()>,
        modified: DeltaTable<char>,
        deltas: DeltaStream<char>,
        dest: Tables,
    }

    impl Test {
        fn replicate(&mut self) {
            let eids = iter_modified(&self.src.eids, &self.any_mod).collect::<Vec<_>>();
            for eid in eids.iter() {
                self.modified.write(*eid, self.src.values.try_get(*eid));
            }
            self.modified.flush(&self.any_mod, &mut self.deltas);
            self.deltas.apply_to(&eids, &mut self.dest.values);
        }
    }

    #[test]
    fn replicate_empty() {
        let mut t = Test::default();
        t.src.eids.add(Eid(1), Eid(1));
        t.any_mod.add(Eid(1), ());
        t.replicate();
    }

    #[test]
    fn replicate_added() {
        let mut t = Test::default();
        t.src.eids.add(Eid(1), Eid(1));
        t.src.eids.add(Eid(2), Eid(2));
        t.src.values.add(Eid(1), 'a');
        t.any_mod.add(Eid(1), ());
        t.any_mod.add(Eid(2), ());
        t.replicate();
        assert_eq!(t.deltas.modified_len(), 1);
        assert_eq!(t.deltas.present_len(), 1);
        assert_eq!(t.deltas.value_len(), 1);
        let values = t.dest.values.iter().into_iter().collect::<Vec<_>>();
        assert_eq!(values.len(), 1);
        assert_eq!(*values[0], 'a');
    }

    #[test]
    fn write_deltas() {
        let mut d = DeltaTable::<Cmp>::new();
        let mut t = DeltaStream::new();
        let mut any_mod = Table::new();
        any_mod.add(Eid(1), ());

        // Add none
        assert_eq!(d.write(Eid(1), None), false);
        d.flush(&any_mod, &mut t);
        assert_eq!(t.modified_len(), 0);
        assert_eq!(t.present_len(), 0);
        assert_eq!(t.value_len(), 0);
        t.clear();

        // Add some
        assert_eq!(d.write(Eid(1), Some(&Cmp::default())), true);
        d.flush(&any_mod, &mut t);
        assert_eq!(t.modified_len(), 1);
        assert_eq!(t.present_len(), 1);
        assert_eq!(t.value_len(), 1);
        t.clear();

        // Add equal value
        assert_eq!(d.write(Eid(1), Some(&Cmp::default())), false);
        d.flush(&any_mod, &mut t);
        assert_eq!(t.modified_len(), 0);
        assert_eq!(t.present_len(), 0);
        assert_eq!(t.value_len(), 0);
        t.clear();

        // Add different value
        let mut cmp = Cmp::default();
        cmp.data[10] = 10;
        assert_eq!(d.write(Eid(1), Some(&cmp)), true);
        d.flush(&any_mod, &mut t);
        assert_eq!(t.modified_len(), 1);
        assert_eq!(t.present_len(), 1);
        assert_eq!(t.value_len(), 1);
        t.clear();

        // Add none
        assert_eq!(d.write(Eid(1), None), true);
        d.flush(&any_mod, &mut t);
        assert_eq!(t.modified_len(), 1);
        assert_eq!(t.present_len(), 0);
        assert_eq!(t.value_len(), 0);
        t.clear();

        // Add some but without mod flag
        any_mod.remove(Eid(1));
        assert_eq!(d.write(Eid(1), Some(&Cmp::default())), true);
        d.flush(&any_mod, &mut t);
        assert_eq!(t.modified_len(), 0);
        assert_eq!(t.present_len(), 0);
        assert_eq!(t.value_len(), 0);
        t.clear();
    }
}
