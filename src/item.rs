use crate::utils::*;
use once_cell::sync::Lazy;
use rand::Rng;
use regex::Regex;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Item {
    /// Restores 25 HP
    HealthPotion,
    /// Restores 25 MP
    ManaPotion,
    /// Restores 10 HP and 10 MP
    Food,
}
pub use Item::*;

impl Item {
    pub(crate) const fn total_variants() -> usize {
        3
    }
    // pub const fn description(&self) -> &'static str {
    //     match self {
    //         HealthPotion => "restores 25 HP",
    //         ManaPotion => "restores 25 MP",
    //         Food => "restores 10 HP and 10 MP",
    //     }
    // }
    pub fn description(&self) -> &String {
        static HEALTH_POTION: Lazy<String> = Lazy::new(|| format!("restores 25 {}", *ANSI_HP));
        static MANA_POTION: Lazy<String> = Lazy::new(|| format!("restores 25 {}", *ANSI_MP));
        static FOOD: Lazy<String> =
            Lazy::new(|| format!("restores 10 {} and 10 {}", *ANSI_HP, *ANSI_MP));

        match self {
            HealthPotion => &*HEALTH_POTION,
            ManaPotion => &*MANA_POTION,
            Food => &*FOOD,
        }
    }
    pub const fn cost(&self) -> usize {
        match self {
            HealthPotion => 2,
            ManaPotion => 3,
            Food => 1,
        }
    }

    pub(crate) fn from_index(i: u8) -> Self {
        const HEALTHPOTION: u8 = HealthPotion as u8;
        const MANAPOTION: u8 = ManaPotion as u8;
        const FOOD: u8 = Food as u8;

        match i {
            HEALTHPOTION => HealthPotion,
            MANAPOTION => ManaPotion,
            FOOD => Food,
            _ => panic!(),
        }
    }
    pub fn gen<T: Rng>(rng: &mut T) -> Self {
        Self::from_index(rng.gen_range(0u8..3u8))
    }
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self::gen(&mut rng)
    }
}

impl FromStr for Item {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_HP: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:hp|health potion)$").unwrap());
        static RE_MP: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:mp|mana potion)$").unwrap());
        static RE_FOOD: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^food$").unwrap());

        if RE_HP.is_match(s) {
            Ok(HealthPotion)
        } else if RE_MP.is_match(s) {
            Ok(ManaPotion)
        } else if RE_FOOD.is_match(s) {
            Ok(Food)
        } else {
            Err(s.to_string())
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthPotion => write!(f, "Health potion"),
            ManaPotion => write!(f, "Mana potion"),
            Food => write!(f, "Food"),
        }
    }
}

pub struct DuplicatedItem {
    pub(crate) kind: Item,
    pub(crate) n: usize,
}
impl DuplicatedItem {
    #[inline]
    pub fn new(kind: Item, n: usize) -> Self {
        Self { kind, n }
    }
}
