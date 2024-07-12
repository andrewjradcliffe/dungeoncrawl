use indexmap::{map::Entry, IndexMap};
use std::fmt::{self, Write};
use std::hash::Hash;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Item {
    /// Restores 25 HP
    HealthPotion,
    /// Restores 25 MP
    ManaPotion,
    /// Restores 10 HP and 10 MP
    Food,
}
pub use Item::*;

impl FromStr for Item {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("hp") || s.eq_ignore_ascii_case("health potion") {
            Ok(HealthPotion)
        } else if s.eq_ignore_ascii_case("mp") || s.eq_ignore_ascii_case("mana potion") {
            Ok(ManaPotion)
        } else if s.eq_ignore_ascii_case("food") {
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
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Inventory:")?;
        for (item, count) in self.bag.iter().filter(|(_, count)| **count > 0) {
            writeln!(f, "    {:<40} x{}", format!("{}", item), count)?;
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
        if s.eq_ignore_ascii_case("u") || s.eq_ignore_ascii_case("use") {
            Ok(InventoryAction::Use)
        } else if s.eq_ignore_ascii_case("d") || s.eq_ignore_ascii_case("drop") {
            Ok(InventoryAction::Drop)
        } else if s.eq_ignore_ascii_case("q") || s.eq_ignore_ascii_case("quit") {
            Ok(InventoryAction::Quit)
        } else {
            Err(s.to_string())
        }
    }
}
