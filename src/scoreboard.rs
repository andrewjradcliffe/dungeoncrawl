use crate::monster::*;
use indexmap::{map::Entry, IndexMap};
use std::fmt::{self, Write};

pub struct Scoreboard(IndexMap<MonsterKind, usize>);
impl Scoreboard {
    pub fn new() -> Self {
        Self(IndexMap::from([
            (Frog, 0),
            (Bat, 0),
            (Goblin, 0),
            (Orc, 0),
            (Dragon, 0),
        ]))
    }
    pub fn record(&mut self, kind: MonsterKind) {
        match self.0.entry(kind) {
            Entry::Occupied(mut v) => {
                *v.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }
}

impl fmt::Display for Scoreboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (kind, count) in self.0.iter().filter(|(_, count)| **count > 0) {
            if *count == 1 {
                writeln!(f, "You defeated 1 {}!", kind.singular())?;
            } else {
                writeln!(f, "You defeated {count} {}!", kind.plural())?;
            }
        }
        Ok(())
    }
}
