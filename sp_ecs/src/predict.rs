use hashbrown::{HashMap, hash_map::Entry};

use crate::{Eid, Table, WriteTable, table::{TableIter, TableIterMut}};

// This is limited to 32 entries because that's the limit of the
// default implementation for arrays, which is needed to avoid
// requiring the Copy trait on delta types (which is an issue if
// they include Vec<> etc)
const BUFFER_SIZE_POW: i64 = 5;
const BUFFER_SIZE: i64 = 1 << BUFFER_SIZE_POW;
const BUFFER_MASK: i64 = BUFFER_SIZE - 1;

/// Does not support adding/removing, only changing existing value
pub trait Delta<T> {
    fn apply_delta_to(&self, value: &mut T);
    fn add_delta(&mut self, rhs: &Self);
}

#[derive(Clone)]
pub struct ReplaceDelta<T>(pub T);

impl<T: Clone> Delta<T> for ReplaceDelta<T> {
    fn apply_delta_to(&self, value: &mut T) {
        *value = self.0.clone();
    }

    fn add_delta(&mut self, rhs: &Self) {
        *self = rhs.clone()
    }
}

#[derive(Clone)]
pub struct AddDelta<T>(pub T);

impl<T: std::ops::Add<Output = T> + Clone + Copy> Delta<T> for AddDelta<T> {
    fn apply_delta_to(&self, value: &mut T) {
        *value = *value + self.0;
    }

    fn add_delta(&mut self, rhs: &Self) {
        self.0 = self.0 + rhs.0
    }
}

pub struct PredictBuffer<T, D = ReplaceDelta<T>> {
    base_value: T,
    base_tick: i64,
    mask: u32,
    deltas: [D; BUFFER_SIZE as usize],
}

impl<T, D> PredictBuffer<T, D> {
    /// Called when writing values from base packet. Assume other code is ensuring
    /// later packets don't overwrite earlier.
    pub fn set_base_value(&mut self, value: T, tick: i64) {
        //println!("Set base value on tick {}", tick);
        //if tick >= self.base_tick 
        {
            self.base_value = value;
            self.base_tick = tick;
        }
    }
}

impl<T, D> PredictBuffer<T, D> {
    pub fn initial(&self) -> &T {
        &self.base_value
    }

    pub fn mask(&self) -> u32 {
        self.mask
    }

    pub fn is_empty(&self) -> bool {
        self.mask == 0
    }

    pub fn get_delta(&self, tick: i64) -> Option<&D> {
        let index = (tick & BUFFER_MASK) as usize;
        let bit = 1 << index;
        if self.mask & bit != 0 { Some(&self.deltas[index]) } else { None }
    }
}

impl<T: Clone, D: Delta<T> + Default> PredictBuffer<T, D> {
    pub fn new(base_value: T, base_tick: i64) -> Self {
        //println!("Create on tick {}", base_tick);
        Self {
            base_value,
            base_tick,
            mask: 0,
            deltas: Default::default()
        }
    }

    fn clear_entry(&mut self, tick: i64) {
        let index = (tick & BUFFER_MASK) as usize;
        let bit = 1 << index;
        self.mask &= !bit;
        self.deltas[index] = Default::default();
    }

    /// Called on every fixed update (after receiving base packet and setting base values).
    /// Clears client tick slot in preparation for sim logic
    pub fn predict(&mut self, predict_tick: i64, value: &mut T) {
        // Clear all ticks from earliest tick up to (but not including) base tick
        // Note the earliest tick is the same as the new client tick
        let min_tick = predict_tick - BUFFER_SIZE;
        // Drag base tick forward
        let base_tick = self.base_tick.max(min_tick);
        for tick in min_tick..base_tick {
            self.clear_entry(tick);
        }
        // Clear entry for next delta
        self.clear_entry(predict_tick);
        // Apply deltas from base tick to client tick to get new predicted value
        let mut new_value = self.base_value.clone();
        if self.mask != 0 {
            for tick in base_tick..predict_tick {
                if let Some(delta) = self.get_delta(tick) {
                    delta.apply_delta_to(&mut new_value);
                }
            }
        }
        *value = new_value;
    }

    /// Called during sim logic
    pub fn write_delta(&mut self, predict_tick: i64, delta: D) {
        let index = (predict_tick & BUFFER_MASK) as usize;
        let bit = 1 << index;
        if self.mask & bit != 0 {
            self.deltas[index].add_delta(&delta);
        } else {
            self.deltas[index] = delta;
            self.mask |= bit;
        }
    }
}

pub struct PredictTable<T, D = ReplaceDelta<T>> {
    enable: bool,
    table: Table<T>,
    buffers: HashMap<Eid, PredictBuffer<T, D>>,
}

impl<T, D> Default for PredictTable<T, D> {
    fn default() -> Self {
        Self {
            enable: false,
            table: Default::default(),
            buffers: Default::default()
        }
    }
}

impl<T, D> PredictTable<T, D> {
    pub fn new_enabled() -> Self {
        Self { enable: true, ..Default::default() }
    }

    pub fn as_writer(&mut self, tick: i64) -> PredictTableWriter<'_, T, D> {
        PredictTableWriter {
            predict: self,
            tick,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enable
    }

    pub fn table(&self) -> &Table<T> {
        &self.table
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    pub fn predict_len(&self) -> usize {
        self.buffers.len()
    }

    pub fn contains(&self, id: Eid) -> bool {
        self.table.contains(id)
    }

    pub fn try_get(&self, id: Eid) -> Option<&T> {
        self.table.try_get(id)
    }

    pub fn get(&self, id: Eid) -> &T {
        self.table.get(id)
    }

    pub fn iter(&self) -> TableIter<T> {
        self.table.iter()
    }

    /// This should only be called on the server
    pub fn iter_base_mut(&mut self) -> TableIterMut<T> {
        self.table.iter_mut()
    }

    pub fn clear(&mut self) {
        self.table.clear();
        self.buffers.clear();
    }

    pub fn get_buffer(&self, eid: Eid) -> Option<&PredictBuffer<T, D>> {
        self.buffers.get(&eid)
    }
}

/// For writing values received from server. Directly writes to table if no 
/// prediction deltas exist or if value is no longer presenp.
impl<T: Clone + Copy + Default, D> PredictTable<T, D> {
    pub fn add(&mut self, eid: Eid, tick: i64, value: T) {
        if self.enable {
            if let Some(buffer) = self.buffers.get_mut(&eid) {
                buffer.set_base_value(value, tick)
            } else {
                self.table.add(eid, value)
            }            
        } else {
            self.table.add(eid, value)
        }
    }

    pub fn remove(&mut self, eid: Eid) -> Option<T> {
        if self.enable {
            self.buffers.remove(&eid);
        }
        self.table.remove(eid)
    }

    pub fn set(&mut self, eid: Eid, tick: i64, value: Option<T>) {
        match value {
            Some(value) => self.add(eid, tick, value),
            None => {
                self.remove(eid);
            },
        }
    }
}

impl<T: Clone + Copy + Default, D: Delta<T> + Default> PredictTable<T, D> {
    /// Copy-on-write by accumulating deltas on top of base value
    pub fn apply_delta(&mut self, eid: Eid, tick: i64, delta: D) {
        if self.enable {
            if let Some(value) = self.table.try_get_mut(eid) {
                match self.buffers.entry(eid) {
                    Entry::Occupied(entry) => {
                        delta.apply_delta_to(value);
                        entry.into_mut().write_delta(tick, delta)
                    },
                    Entry::Vacant(entry) => {
                        let mut buffer = PredictBuffer::new(value.clone(), tick);
                        delta.apply_delta_to(value);
                        buffer.write_delta(tick, delta);
                        entry.insert(buffer);
                    }
                }
            }
        } else {
            delta.apply_delta_to(self.table.get_mut(eid))
        }
    }

    pub fn remove_join(&mut self, source: &Table<Eid>) {
        if self.enable {
            for eid in source.iter() {
                self.buffers.remove(eid);
            }
        }
        self.table.remove_iter(source.iter_pages());
    }

    /// For updating predictions after receiving a packet from base and setting 
    /// values received from base. This should be called to prepare for a fixed
    /// update (by clearing delta slot of client tick).
    pub fn predict(&mut self, predict_tick: i64) {
        if self.enable {
            // Clear any client tick delta and calculate predicted values
            for (&eid, buffer) in self.buffers.iter_mut() {
                let value = self.table.get_mut(eid);
                buffer.predict(predict_tick, value);
            }
            // Remove any buffers which no longer have deltas
            self.buffers.retain(|_, buffer| !buffer.is_empty());
        }
    }

    pub fn move_value(&mut self, from: Eid, to: Eid) {
        if self.enable {
            if let Some(buffer) = self.buffers.remove(&from) {
                self.buffers.insert(to, buffer);
            }
        }
        self.table.move_value(from, to)
    }
}

pub struct PredictTableWriter<'a, T, D> {
    tick: i64,
    predict: &'a mut PredictTable<T, D>,
}

impl<'a, T: Clone + Copy + Default, D: Delta<T> + Default> WriteTable<T> for PredictTableWriter<'a, T, D> {
    fn add(&mut self, eid: Eid, value: T) {
        self.predict.add(eid, self.tick, value)
    }

    fn remove(&mut self, eid: Eid) -> Option<T> {
        self.predict.remove(eid)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Delta<i32> for i32 {
        fn apply_delta_to(&self, value: &mut i32) {
            *value += *self;
        }

        fn add_delta(&mut self, rhs: &Self) {
            *self += rhs;
        }
    }

    #[test]
    fn predict_with_deltas() {
        let mut p = PredictTable::<i32, i32>::new_enabled();
        p.add(Eid(1), 0, 10);
        // base msg, no deltas
        p.add(Eid(1), 10, 100);
        assert_eq!(*p.get(Eid(1)), 100);
        assert_eq!(p.predict_len(), 0);
        // Delta
        p.apply_delta(Eid(1), 15, 200);
        assert_eq!(*p.get(Eid(1)), 300);
        assert_eq!(p.predict_len(), 1);
        // No new base msg
        p.predict(16);
        assert_eq!(*p.get(Eid(1)), 300);
        assert_eq!(p.predict_len(), 1);
        // base msg
        p.add(Eid(1), 11, 105);
        assert_eq!(*p.get(Eid(1)), 300);
        assert_eq!(p.predict_len(), 1);
        p.predict(17);
        assert_eq!(*p.get(Eid(1)), 305);
        assert_eq!(p.predict_len(), 1);
        // Delta
        p.apply_delta(Eid(1), 16, -50);
        assert_eq!(*p.get(Eid(1)), 255);
        // No new base msg
        p.predict(18);
        assert_eq!(*p.get(Eid(1)), 255);
        // No new base msg, deltas cleared
        p.predict(28);
        // assert_eq!(*p.get(Eid(1)), 105);
        // // Two deltas
        // p.apply_delta(Eid(1), 28, 1);
        // p.apply_delta(Eid(1), 28, 2);
        // assert_eq!(*p.get(Eid(1)), 108);
        // // Another deltas in next tick
        // p.predict(29);
        // p.apply_delta(Eid(1), 29, 3);
        // assert_eq!(*p.get(Eid(1)), 111);
        // // Advance tick
        // for i in 0..5 {
        //     p.predict(30 + i);
        //     assert_eq!(*p.get(Eid(1)), 111);
        // }
    }
}
