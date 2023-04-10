use serde::{Serialize, Deserialize};

use super::page::*;
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default, Clone, Copy)]
pub struct Eid(pub u32);

impl Eid {
    pub const BITS: u32 = 32;
    pub const GEN_BITS: u32 = 8;
    pub const GEN_COUNT: u32 = 1 << Self::GEN_BITS;
    pub const GEN_MASK: u32 = Self::GEN_COUNT - 1;
    pub const INDEX_BITS: u32 = Self::BITS - Self::GEN_BITS;
    pub const INDEX_COUNT: u32 = 1 << Self::INDEX_BITS;
    pub const INDEX_MASK: u32 = Self::INDEX_COUNT - 1;

    pub fn index(&self) -> usize {
        (self.0 & Self::INDEX_MASK) as usize
    }

    pub fn gen(&self) -> u32 {
        self.0 >> Self::INDEX_BITS
    }

    pub fn with_gen(&self, gen: u32) -> Eid {
        Eid((self.0 & Self::INDEX_MASK) | (gen << Self::INDEX_BITS))
    }

    pub fn increment_gen(&self) -> Eid {
        let next = (self.gen() + 1) & Self::GEN_MASK;
        self.with_gen(next)
    }
}

impl Into<usize> for Eid {
    fn into(self) -> usize {
        self.index()
    }
}

impl Display for Eid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.index(), self.gen())
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct GroupId(pub u32);

pub struct EntityPool {
    page_mask: u64,
    groups: Vec<VecDeque<Eid>>,
    pages: Vec<GroupId>,
}

impl Default for EntityPool {
    fn default() -> Self {
        Self {
            page_mask: u64::MAX,
            groups: Default::default(),
            pages: Default::default()
        }
    }
}

impl EntityPool {
    /// Mask can be used to ensure mutually exclusive IDs generated from
    /// different pools. Mask must have at least one bit present, otherwise
    /// no entities can be created.
    pub fn new(page_mask: u64) -> Option<Self> {
        if page_mask == 0 { None }
        else {
            Some(Self {
                page_mask,
                ..Default::default()
            })
        }
    }
}

impl EntityPool {
    fn is_page_usable(page_mask: u64, page_index: usize) -> bool {
        // Page mask applies to repeating groups of 64 pages
        let page_bit = 1 << (page_index % 64);
        page_mask & page_bit != 0
    }

    pub fn create_in(&mut self, group_id: GroupId) -> Eid {
        // Allocate groups through ID if needed
        let group_index = group_id.0 as usize;
        while self.groups.len() <= group_index {
            self.groups.push(VecDeque::new());
        }
        // Try to get next pooled ID from group
        let group = &mut self.groups[group_index];
        match group.pop_front() {
            Some(eid) => eid,
            None => {
                // If no IDs remain, we need to allocate a new page for group.
                // First add dummy pages until we reach a page allowed by mask
                while !Self::is_page_usable(self.page_mask, self.pages.len()) {
                    self.pages.push(GroupId(u32::MAX));
                }
                // Reserve all IDs of the page for this group (excluding the first
                // ID which we will return from here)
                let base_id = self.pages.len() as u32 * PAGE_SIZE;
                for i in 1..PAGE_SIZE {
                    group.push_back(Eid(i + base_id))
                }
                // Claim page for group
                self.pages.push(group_id);
                Eid(base_id)
            }
        }
    }

    pub fn create(&mut self) -> Eid {
        self.create_in(GroupId(0))
    }

    /// Recycles Eid, adding it to queue with an incremented gen
    pub fn recycle(&mut self, eid: Eid) {
        let i = eid.index();
        let page_index = i >> PAGE_SIZE_POW;
        if Self::is_page_usable(self.page_mask, page_index) {
            let group_id = self.pages[page_index];
            let next_eid = eid.increment_gen();
            self.groups[group_id.0 as usize].push_back(next_eid);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_eid() {
        let mut p = EntityPool::default();
        let eid = p.create();
        assert_eq!(eid, Eid(0));
        let eid = p.create();
        assert_eq!(eid, Eid(1));
    }

    #[test]
    fn create_eid_with_page_mask() {
        let mut p = EntityPool::new(0b10).unwrap();
        let eid = p.create();
        assert_eq!(eid, Eid(PAGE_SIZE));
        let eid = p.create();
        assert_eq!(eid, Eid(PAGE_SIZE + 1));
    }

    #[test]
    fn create_many_eids_with_page_mask() {
        let mut p = EntityPool::new(0b100010010).unwrap();
        for i in 0..PAGE_SIZE {
            let eid = p.create();
            assert_eq!(eid, Eid(i + PAGE_SIZE * 1));
        }
        for i in 0..PAGE_SIZE {
            let eid = p.create();
            assert_eq!(eid, Eid(i + PAGE_SIZE * 4));
        }
        for i in 0..PAGE_SIZE {
            let eid = p.create();
            assert_eq!(eid, Eid(i + PAGE_SIZE * 8));
        }
    }

    #[test]
    fn recycle_eid() {
        let mut p = EntityPool::default();
        for _ in 0..PAGE_SIZE {
            let eid = p.create();
            p.recycle(eid);
        }
        let eid = p.create();
        assert_eq!(eid, Eid(0).with_gen(1));
        let eid = p.create();
        assert_eq!(eid, Eid(1).with_gen(1));
    }

    #[test]
    fn recycle_eid_with_page_mask() {
        let mut p = EntityPool::new(0b10).unwrap();
        for i in 0..PAGE_SIZE {
            p.recycle(Eid(i));
        }
        let eid = p.create();
        assert_eq!(eid, Eid(PAGE_SIZE));
    }

    #[test]
    fn create_eid_in_group() {
        let mut p = EntityPool::default();
        let eid = p.create_in(GroupId(1));
        assert_eq!(eid, Eid(0));
        let eid = p.create_in(GroupId(2));
        assert_eq!(eid, Eid(PAGE_SIZE));
    }

    #[test]
    fn recycle_page_of_eids() {
        let mut p = EntityPool::default();
        let count = Eid::GEN_COUNT * 2;
        for gi in 0..count {
            for i in 0..PAGE_SIZE {
                let eid = p.create();
                assert_eq!(eid, Eid(i).with_gen(gi & Eid::GEN_MASK));
                p.recycle(eid);
            }
        }
    }
}
