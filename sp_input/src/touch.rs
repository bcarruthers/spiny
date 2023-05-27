use glam::{Vec2, UVec2};
use indexmap::{IndexMap, map::Entry};

#[derive(Clone)]
pub struct TouchPress {
    pub pos: Vec2,
    pub delta: Vec2,
    pub norm_pos: Vec2,
    pub norm_delta: Vec2,
}

#[derive(Debug)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

#[derive(Debug)]
pub struct TouchEvent {
    pub phase: TouchPhase,
    pub id: u64,
    pub pos: Vec2,
}

#[derive(Default, Clone)]
pub struct TouchState {
    presses: IndexMap<u64, TouchPress>
}

impl TouchState {
    pub fn iter(&self) -> impl Iterator<Item = (&u64, &TouchPress)> {
        self.presses.iter()
    }
    
    pub fn update(&mut self, event: &TouchEvent, size: UVec2) {
        //log::info!("Touch: {:?}", event);
        match self.presses.entry(event.id) {
            Entry::Occupied(entry) =>
                match event.phase {
                    TouchPhase::Started | 
                    TouchPhase::Moved => {
                        let norm_pos = event.pos / size.as_vec2();
                        let press = entry.into_mut();
                        press.delta = event.pos - press.pos;
                        press.norm_delta = norm_pos - press.norm_pos;
                        press.pos = event.pos;
                        press.norm_pos = norm_pos;
                    },
                    TouchPhase::Cancelled |
                    TouchPhase::Ended => {
                        entry.remove();
                    }
                },
            Entry::Vacant(entry) => 
                match event.phase {
                    TouchPhase::Started | 
                    TouchPhase::Moved => {
                        entry.insert(TouchPress {
                            pos: event.pos,
                            delta: Vec2::ZERO,
                            norm_pos: event.pos / size.as_vec2(),
                            norm_delta: Vec2::ZERO,
                        });
                    },
                    TouchPhase::Cancelled |
                    TouchPhase::Ended => ()
                }
        }
    }
}
