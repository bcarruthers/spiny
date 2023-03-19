use crate::Eid;
use super::flatten::*;
use super::iter::*;
use super::mask::*;
use super::page::*;
use std::slice::{Iter, IterMut};

// Iterators over values in table
pub type TableIter<'a, T> = FlatPageIntoIter<PageIter<Iter<'a, PageOption<T>>>>;
pub type TableIterMut<'a, T> = FlatPageIntoIter<PageIterMut<IterMut<'a, PageOption<T>>>>;

pub trait WriteTable<T> {
    fn add(&mut self, id: Eid, value: T);
    fn remove(&mut self, id: Eid) -> bool;

    fn set(&mut self, id: Eid, value: Option<T>) {
        if let Some(value) = value {
            self.add(id, value);
        } else {
            self.remove(id);
        }
    }

    /// Adds or sets values, overwriting any existing
    fn add_iter<I: IntoIterator<Item = (Eid, T)>>(&mut self, entries: I) {
        for (id, value) in entries {
            self.add(id, value);
        }
    }
}

/// Table of components stored in pages
pub struct Table<T> {
    pages: Vec<PageOption<T>>,
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Self {
            pages: Default::default(),
        }
    }
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Self { pages: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.pages.iter().flatten().map(|page| page.len()).sum()
    }

    /// Removes all pages
    pub fn clear(&mut self) {
        self.pages.clear();
    }

    pub fn is_empty(&self) -> bool {
        !self.pages.iter().flatten().any(|page| !page.is_empty())
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn try_get_page(&self, i: usize) -> Option<&Page<T>> {
        if i >= self.pages.len() {
            None
        } else {
            match &self.pages[i] {
                Some(page) => Some(page.as_ref()),
                None => None,
            }
        }
    }

    pub fn try_get_page_mut(&mut self, i: usize) -> Option<&mut Page<T>> {
        if i >= self.pages.len() {
            None
        } else {
            match &mut self.pages[i] {
                Some(page) => Some(page.as_mut()),
                None => None,
            }
        }
    }

    pub fn get_page(&self, i: usize) -> &Page<T> {
        self.try_get_page(i).unwrap()
    }

    pub fn get_page_mut(&mut self, i: usize) -> &mut Page<T> {
        self.try_get_page_mut(i).unwrap()
    }

    pub fn contains(&self, id: Eid) -> bool {
        let i: usize = id.into();
        let page_index = i >> PAGE_SIZE_POW;
        match self.try_get_page(page_index) {
            Some(page) => {
                let index_in_page = i & PAGE_MASK as usize;
                page.contains(index_in_page)
            }
            None => false,
        }
    }

    pub fn try_get(&self, id: Eid) -> Option<&T> {
        let i: usize = id.into();
        let page_index = i >> PAGE_SIZE_POW;
        match self.try_get_page(page_index) {
            Some(page) => {
                let index_in_page = i & PAGE_MASK as usize;
                page.try_get(index_in_page)
            }
            None => None,
        }
    }

    pub fn try_get_mut(&mut self, id: Eid) -> Option<&mut T> {
        let i: usize = id.into();
        let page_index = i >> PAGE_SIZE_POW;
        match self.try_get_page_mut(page_index) {
            Some(page) => {
                let index_in_page = i & PAGE_MASK as usize;
                page.try_get_mut(index_in_page)
            }
            None => None,
        }
    }

    pub fn get(&self, id: Eid) -> &T {
        let i: usize = id.into();
        let page_index = i >> PAGE_SIZE_POW;
        let index_in_page = i & PAGE_MASK as usize;
        let page = self.get_page(page_index);
        page.get(index_in_page)
    }

    pub fn get_mut(&mut self, id: Eid) -> &mut T {
        let i: usize = id.into();
        let page_index = i >> PAGE_SIZE_POW;
        let index_in_page = i & PAGE_MASK as usize;
        let page = self.get_page_mut(page_index);
        page.get_mut(index_in_page)
    }
}

impl<T: Default> Table<T> {
    /// Clears each page but keeps them present
    pub fn clear_pages(&mut self) {
        for page in self.pages.iter_mut() {
            if let Some(page) = page {
                page.clear();
            }
        }
    }

    pub fn remove_page(&mut self, i: usize) -> bool {
        let removed = i < self.pages.len() && self.pages[i].is_some();
        if removed {
            self.pages[i] = None;
        }
        removed
    }
    
    pub fn add_page(&mut self, i: usize) -> &mut Page<T> {
        while self.pages.len() <= i {
            self.pages.push(None);
        }
        let entry = &mut self.pages[i];
        if entry.is_none() {
            let page = Page::<T>::new();
            *entry = Some(Box::new(page));
        }
        entry.as_mut().unwrap()
    }

    pub fn try_add(&mut self, id: Eid, value: T) -> bool {
        let i: usize = id.into();
        let page_index = i >> PAGE_SIZE_POW;
        let index_in_page = i & PAGE_MASK as usize;
        let page = self.add_page(page_index);
        page.try_add(index_in_page, value)
    }

    pub fn try_add_mut(&mut self, id: Eid) -> Option<&mut T> {
        let i: usize = id.into();
        let page_index = i >> PAGE_SIZE_POW;
        let index_in_page = i & PAGE_MASK as usize;
        let page = self.add_page(page_index);
        page.try_add_mut(index_in_page)
    }

    pub fn get_or_add_mut(&mut self, id: Eid) -> &mut T {
        let i: usize = id.into();
        let page_index = i >> PAGE_SIZE_POW;
        let index_in_page = i & PAGE_MASK as usize;
        let page = self.add_page(page_index);
        page.get_or_add_mut(index_in_page)
    }
}

impl<T: Default> WriteTable<T> for Table<T> {
    /// Adds or sets value, overwriting any existing
    fn add(&mut self, id: Eid, value: T) {
        let i: usize = id.into();
        let page_index = i >> PAGE_SIZE_POW;
        let index_in_page = i & PAGE_MASK as usize;
        let page = self.add_page(page_index);
        page.add(index_in_page, value)
    }

    fn remove(&mut self, id: Eid) -> bool {
        let i: usize = id.into();
        let page_index = i >> PAGE_SIZE_POW;
        let index_in_page = i & PAGE_MASK as usize;
        match self.try_get_page_mut(page_index) {
            Some(page) => page.remove(index_in_page),
            None => false,
        }
    }
}

impl<T> Table<T> {
    pub fn iter_pages(&self) -> PageIter<Iter<PageOption<T>>> {
        PageIter(self.pages.iter())
    }

    pub fn iter_pages_mut(&mut self) -> PageIterMut<IterMut<PageOption<T>>> {
        PageIterMut(self.pages.iter_mut())
    }

    pub fn iter(&self) -> TableIter<T> {
        FlatPageIntoIter::new(self.iter_pages())
    }

    pub fn iter_mut(&mut self) -> TableIterMut<T> {
        FlatPageIntoIter::new(self.iter_pages_mut())
    }
}

impl<T: Default> Table<T> {
    pub fn remove_iter<
        MB: Iterator<Item = u64>,
        VB: Iterator,
        IB: Iterator<Item = Option<IntoMaskIter<MB, VB>>>,
    >(
        &mut self,
        iter: IB,
    ) {
        let into_iter = PageIntoIter::new(iter);
        for entry in self.pages.iter_mut().zip(into_iter.into_iter()) {
            if let (Some(page), Some(masked)) = entry {
                page.remove_mask_iter(masked.masks);
            }
        }
    }

    pub fn remove_join<U>(&mut self, source: &Table<U>) {
        self.remove_iter(source.iter_pages());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_empty() {
        let mut t = Table::<i32>::new();
        assert_eq!(t.is_empty(), true);
        t.add(Eid(10), 100);
        assert_eq!(t.is_empty(), false);
        t.remove(Eid(10));
        assert_eq!(t.is_empty(), true);
    }

    #[test]
    fn clear() {
        let mut t = Table::<i32>::new();
        t.add(Eid(10), 100);
        t.clear();
        assert_eq!(t.is_empty(), true);
        assert_eq!(t.contains(Eid(10)), false);
    }

    #[test]
    fn add_remove_items() {
        let mut t = Table::<i32>::new();
        assert_eq!(t.contains(Eid(0)), false);
        assert_eq!(t.iter().into_iter().count(), 0);
        t.add(Eid(10), 100);
        assert_eq!(t.contains(Eid(0)), false);
        assert_eq!(t.contains(Eid(10)), true);
        assert_eq!(*t.get(Eid(10)), 100);
        assert_eq!(*t.get_mut(Eid(10)), 100);
        let items: Vec<i32> = t.iter().into_iter().cloned().collect();
        assert_eq!(&items, &[100]);
        t.add(Eid(20), 200);
        assert_eq!(t.contains(Eid(20)), true);
        assert_eq!(*t.get(Eid(20)), 200);
        assert_eq!(*t.get_mut(Eid(20)), 200);
        let items: Vec<i32> = t.iter().into_iter().cloned().collect();
        assert_eq!(&items, &[100, 200]);
        t.remove(Eid(20));
        assert_eq!(t.contains(Eid(20)), false);
        assert_eq!(t.try_get(Eid(20)), None);
        assert_eq!(t.try_get_mut(Eid(20)), None);
        let items: Vec<i32> = t.iter().into_iter().cloned().collect();
        assert_eq!(&items, &[100]);
    }

    #[test]
    fn add_multiple_times() {
        let mut t = Table::<i32>::new();
        t.add(Eid(1), 10);
        assert_eq!(*t.get(Eid(1)), 10);
        t.add(Eid(1), 11);
        assert_eq!(*t.get(Eid(1)), 11);
    }

    #[test]
    fn remove_not_present() {
        let mut t = Table::<i32>::new();
        assert_eq!(t.remove(Eid(1)), false);
        assert_eq!(t.try_get(Eid(1)), None);
    }

    #[test]
    fn left_join() {
        let mut a = Table::<i32>::new();
        let mut b = Table::<i32>::new();
        a.add(Eid(1), 10);
        a.add(Eid(2), 20);
        a.add(Eid(2000), 30);
        b.add(Eid(2), 200);
        b.add(Eid(2000), 300);
        let mut r = Vec::new();
        for (a, b) in a.iter().left_join(b.iter()) {
            r.push((*a, b.cloned()));
        }
        assert_eq!(r.len(), 3);
        assert_eq!(r[0], (10, None));
        assert_eq!(r[1], (20, Some(200)));
        assert_eq!(r[2], (30, Some(300)));
    }

    #[test]
    fn iter_pages() {
        let mut t = Table::<i32>::new();
        t.add(Eid(20), 200);
        let mut sum = 0;
        for page in t.iter_pages().flatten() {
            for x in page {
                sum += x;
            }
        }
        assert_eq!(sum, 200);
    }
}
