use indexmap::{map::Entry, IndexMap};
use once_cell::sync::Lazy;
use rand::Rng;
use regex::Regex;
use std::fmt::{self, Write};
use std::hash::Hash;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub enum Item {
    /// Restores 25 HP
    HealthPotion,
    /// Restores 25 MP
    ManaPotion,
    /// Restores 10 HP and 10 MP
    Food,
}
pub use Item::*;

use crate::loot::Loot;

impl Item {
    pub fn description(&self) -> &'static str {
        match self {
            HealthPotion => "restores 25 HP",
            ManaPotion => "restores 25 MP",
            Food => "restores 10 HP and 10 MP",
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory {
    bag: IndexMap<Item, usize>,
    sum: usize,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            bag: IndexMap::from([(HealthPotion, 1), (ManaPotion, 1), (Food, 2)]),
            sum: 4,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.sum == 0
    }

    pub fn menu(&mut self) -> Option<Item> {
        let mut buf = String::with_capacity(1 << 7);
        println!("---- Entering inventory menu... ----");
        loop {
            if self.is_empty() {
                println!("Inventory is empty!");
                break None;
            }
            buf.clear();
            println!("{}", self);

            print!("👜 ");
            io::Write::flush(&mut io::stdout()).unwrap();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            match handle.read_line(&mut buf) {
                Ok(_) => (),
                Err(e) => println!("Error in inventory menu readline: {:#?}", e),
            }
            let s = buf.trim();
            if let Some((lhs, rhs)) = s.split_once(' ') {
                if let Ok(action) = lhs.parse::<InventoryAction>() {
                    if let Ok(item) = rhs.parse::<Item>() {
                        match action {
                            InventoryAction::Drop => {
                                self.drop_item(item);
                                return None;
                            }
                            InventoryAction::Use => {
                                return self.pop_item(item);
                            }
                            InventoryAction::Quit => (),
                        }
                    }
                }
            } else {
                if let Ok(InventoryAction::Quit) = s.parse::<InventoryAction>() {
                    break None;
                }
            }
        }
    }
    pub fn pop_item(&mut self, item: Item) -> Option<Item> {
        match self.bag.entry(item) {
            Entry::Occupied(mut v) => {
                if *v.get() > 0 {
                    self.sum -= 1;
                    *v.get_mut() -= 1;
                    Some(item)
                } else {
                    None
                }
            }
            Entry::Vacant(_) => None,
        }
    }
    pub fn drop_item(&mut self, item: Item) {
        self.pop_item(item);
    }
    pub fn push_loot(&mut self, loot: Loot) {
        self.sum += loot.amount;
        match self.bag.entry(loot.item) {
            Entry::Occupied(mut v) => {
                *v.get_mut() += loot.amount;
            }
            Entry::Vacant(e) => {
                e.insert(loot.amount);
            }
        }
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Inventory:")?;
        for (item, count) in self.bag.iter().filter(|(_, count)| **count > 0) {
            writeln!(
                f,
                "    {:<30} x{} | {}",
                format!("{}", item),
                count,
                item.description()
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum InventoryAction {
    Use,
    Drop,
    Quit,
}

impl FromStr for InventoryAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_USE: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:use|u)$").unwrap());

        static RE_DROP: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:drop|d)$").unwrap());

        static RE_QUIT: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:quit|q)$").unwrap());

        if RE_USE.is_match(s) {
            Ok(InventoryAction::Use)
        } else if RE_DROP.is_match(s) {
            Ok(InventoryAction::Drop)
        } else if RE_QUIT.is_match(s) {
            Ok(InventoryAction::Quit)
        } else {
            Err(s.to_string())
        }
    }
}
