#![forbid(unsafe_code)]

pub mod delta;
pub mod entity;
mod flatten;
pub mod iter;
mod join;
pub mod mask;
pub mod page;
pub mod predict;
pub mod table;
pub mod tuple;

pub use entity::{Eid, EntityPool};
pub use predict::*;
pub use table::{Table, WriteTable};
pub use tuple::Join;

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialEq, Default, Clone, Copy)]
    struct Pos {
        x: i32,
        y: i32,
    }

    #[derive(Debug, PartialEq, Default, Clone, Copy)]
    struct Vel {
        x: i32,
        y: i32,
    }

    #[derive(Default, Clone, Copy)]
    struct Created;

    #[derive(Default)]
    struct World {
        pool: EntityPool,
        eids: Table<Eid>,
        pos: Table<Pos>,
        vel: Table<Vel>,
        destroyed: Table<()>,
    }

    impl World {
        fn commit(&mut self) {
            if !self.destroyed.is_empty() {
                for (eid, _) in (self.eids.iter(), self.destroyed.iter()).join() {
                    self.pool.recycle(*eid);
                }
                self.eids.remove_join(&self.destroyed);
                self.pos.remove_join(&self.destroyed);
                self.vel.remove_join(&self.destroyed);
                self.destroyed.clear();
            }
        }
    }

    #[test]
    fn create_destroy_entities() {
        let mut w = World::default();
        // Create entities
        for i in 0..10 {
            let eid = w.pool.create();
            w.eids.add(eid, eid);
            w.pos.add(eid, Pos { x: i, y: i * 2 });
            w.vel.add(eid, Vel { x: 3, y: 4 });
        }
        // Update positions
        for (p, v) in (w.pos.iter_mut(), w.vel.iter()).join() {
            p.x += v.x;
            p.y += v.y;
        }
        for eid in w.eids.iter() {
            assert_eq!(
                *w.pos.get(*eid),
                Pos {
                    x: eid.index() as i32 + 3,
                    y: eid.index() as i32 * 2 + 4
                }
            );
        }
        // Mark half as destroyed
        for eid in w.eids.iter() {
            if eid.index() % 2 == 0 {
                w.destroyed.add(*eid, Default::default());
            }
        }
        // Commit destroyed
        w.commit();
        assert_eq!(w.eids.iter().into_iter().count(), 5);
        assert_eq!(w.pos.iter().into_iter().count(), 5);
        assert_eq!(w.vel.iter().into_iter().count(), 5);
        assert_eq!(w.destroyed.iter().into_iter().count(), 0);
    }

    #[test]
    fn destroy_entities() {}

    #[test]
    fn query_world() {}
}
