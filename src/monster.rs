use crate::combat::Combatant;
use rand::Rng;
use std::fmt::{self, Write};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    pub(crate) kind: MonsterKind,
    pub(crate) strength: i64,
    pub(crate) current_hp: i64,
    pub(crate) max_hp: i64,
}

impl Combatant for Monster {
    fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
    fn receive_damage(&mut self, amount: i64) {
        self.current_hp = (self.current_hp - amount).clamp(0, self.max_hp);
    }
}

impl Monster {
    pub fn new(kind: MonsterKind) -> Self {
        Self {
            kind,
            strength: kind.strength(),
            current_hp: kind.max_hp(),
            max_hp: kind.max_hp(),
        }
    }
    pub fn status(&self) -> String {
        format!("HP[{}/{}]", self.current_hp, self.max_hp,)
    }
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0u8..8u8);
        let kind = match i {
            0 => Frog,
            1 => Bat,
            2 => Wolf,
            3 => Goblin,
            4 => Bear,
            5 => Orc,
            6 => Dragon,
            7 => Fairy,
            _ => panic!(),
        };
        Monster::new(kind)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub enum MonsterKind {
    Frog,
    Bat,
    Wolf,
    Goblin,
    Bear,
    Orc,
    Dragon,
    Fairy,
}
pub use MonsterKind::*;

impl MonsterKind {
    pub const fn max_hp(&self) -> i64 {
        match self {
            Frog => 20,
            Bat => 25,
            Wolf => 35,
            Goblin => 50,
            Bear => 75,
            Orc => 100,
            Dragon => 250,
            Fairy => 1,
        }
    }

    pub const fn strength(&self) -> i64 {
        match self {
            Frog => 5,
            Bat => 7,
            Wolf => 8,
            Goblin => 10,
            Bear => 12,
            Orc => 15,
            Dragon => 20,
            Fairy => -20,
        }
    }
}

impl fmt::Display for MonsterKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Frog => write!(f, "frog"),
            Wolf => write!(f, "wolf"),
            Bat => write!(f, "bat"),
            Goblin => write!(f, "goblin"),
            Bear => write!(f, "bear"),
            Orc => write!(f, "orc"),
            Dragon => write!(f, "dragon"),
            Fairy => write!(f, "fairy"),
        }
    }
}
