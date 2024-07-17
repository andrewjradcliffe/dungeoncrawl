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
        Monster::new(MonsterKind::rand())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub(crate) const fn loot_weight(&self) -> usize {
        (self.max_hp() / 20) as usize
    }

    pub(crate) const fn from_index(i: u8) -> Self {
        const FROG: u8 = Frog as u8;
        const BAT: u8 = Bat as u8;
        const WOLF: u8 = Wolf as u8;
        const GOBLIN: u8 = Goblin as u8;
        const BEAR: u8 = Bear as u8;
        const ORC: u8 = Orc as u8;
        const DRAGON: u8 = Dragon as u8;
        const FAIRY: u8 = Fairy as u8;

        match i {
            FROG => Frog,
            BAT => Bat,
            WOLF => Wolf,
            GOBLIN => Goblin,
            BEAR => Bear,
            ORC => Orc,
            DRAGON => Dragon,
            FAIRY => Fairy,
            _ => panic!(),
        }
    }
    pub fn gen<T: Rng>(rng: &mut T) -> Self {
        Self::from_index(rng.gen_range(0u8..8u8))
    }
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self::gen(&mut rng)
    }

    pub const fn singular(&self) -> &'static str {
        match self {
            Frog => "frog",
            Wolf => "wolf",
            Bat => "bat",
            Goblin => "goblin",
            Bear => "bear",
            Orc => "orc",
            Dragon => "dragon",
            Fairy => "fairy",
        }
    }
    pub const fn plural(&self) -> &'static str {
        match self {
            Frog => "frogs",
            Wolf => "wolves",
            Bat => "bats",
            Goblin => "goblins",
            Bear => "bears",
            Orc => "orcs",
            Dragon => "dragons",
            Fairy => "fairies",
        }
    }
}

impl fmt::Display for MonsterKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.singular())
    }
}
