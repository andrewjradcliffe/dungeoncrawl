use crate::item::*;
use crate::loot::Loot;
use crate::player::*;
use indexmap::{map::Entry, IndexMap};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory {
    pub(crate) bag: IndexMap<Item, usize>,
    pub(crate) sum: usize,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            bag: IndexMap::with_capacity(Item::total_variants()),
            sum: 0,
        }
    }
    pub fn new_player() -> Self {
        [(HealthPotion, 1), (ManaPotion, 1), (Food, 2)]
            .into_iter()
            .collect()
    }
    pub fn is_empty(&self) -> bool {
        self.sum == 0
    }

    pub fn menu(&mut self, msg: &str) -> Option<Item> {
        let mut buf = String::with_capacity(1 << 7);
        println!("---- Entering inventory menu... ----");
        println!("{}", msg);
        if self.is_empty() {
            None
        } else {
            loop {
                buf.clear();
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
    pub fn pop_multiple(&mut self, item: Item, n: usize) -> Option<DuplicatedItem> {
        match self.bag.entry(item) {
            Entry::Occupied(mut v) => match *v.get() {
                0 => None,
                u if u >= n => {
                    self.sum -= n;
                    *v.get_mut() -= n;
                    Some(DuplicatedItem::new(item, n))
                }
                u => {
                    self.sum -= u;
                    *v.get_mut() = 0;
                    Some(DuplicatedItem::new(item, u))
                }
            },
            Entry::Vacant(_) => None,
        }
    }
    pub fn drop_multiple(&mut self, item: Item, n: usize) {
        self.pop_multiple(item, n);
    }
    pub fn drop_item(&mut self, item: Item) {
        self.pop_item(item);
    }
    pub fn push_multiple(&mut self, item: Item, count: usize) {
        self.sum += count;
        match self.bag.entry(item) {
            Entry::Occupied(mut v) => {
                *v.get_mut() += count;
            }
            Entry::Vacant(e) => {
                e.insert(count);
            }
        }
    }
    pub fn push(&mut self, item: Item) {
        self.push_multiple(item, 1)
    }
    pub fn push_loot(&mut self, loot: Loot) {
        self.push_multiple(loot.item, loot.amount)
    }
    pub fn push_duplicated(&mut self, dup: DuplicatedItem) {
        self.push_multiple(dup.kind, dup.n)
    }
    pub fn n_available(&self, item: &Item) -> usize {
        self.bag.get(item).map(Clone::clone).unwrap_or(0)
    }
}
impl FromIterator<(Item, usize)> for Inventory {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Item, usize)>,
    {
        let mut inv = Self::new();
        for (item, count) in iter {
            inv.push_multiple(item, count);
        }
        inv
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Inventory:")?;
        for (item, count) in self.bag.iter().filter(|(_, count)| **count > 0) {
            writeln!(
                f,
                "    {:<30} x{:<4} | {}",
                format!("{}", item),
                count,
                item.description()
            )?;
        }
        Ok(())
    }
}
