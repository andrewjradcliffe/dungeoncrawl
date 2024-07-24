use crate::consumable::*;
use crate::loot::Loot;
use crate::multiset::MultiSet;
use crate::utils::*;
use ansi_term::{Colour, Style};
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

        if RE_USE.is_match(s) {
            Ok(InventoryAction::Use)
        } else if RE_DROP.is_match(s) {
            Ok(InventoryAction::Drop)
        } else if is_quit(s) {
            Ok(InventoryAction::Quit)
        } else {
            Err(s.to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InventoryTransaction {
    Use(Consumable),
    Drop(Consumable),
    Quit,
}
impl FromStr for InventoryTransaction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some((lhs, rhs)) = s.split_once(' ') {
            if let Ok(action) = lhs.parse::<InventoryAction>() {
                if let Ok(item) = rhs.parse::<Consumable>() {
                    match action {
                        InventoryAction::Drop => {
                            return Ok(InventoryTransaction::Drop(item));
                        }
                        InventoryAction::Use => {
                            return Ok(InventoryTransaction::Use(item));
                        }
                        InventoryAction::Quit => (),
                    }
                }
            }
        } else if let Ok(InventoryAction::Quit) = s.parse::<InventoryAction>() {
            return Ok(InventoryTransaction::Quit);
        }
        Err(s.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory(MultiSet<Consumable>);

impl Inventory {
    pub fn new() -> Self {
        Self(MultiSet::with_capacity(Consumable::total_variants()))
    }
    pub fn new_player() -> Self {
        [(HealthPotion, 1), (ManaPotion, 1), (Food, 2)]
            .into_iter()
            .collect()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn menu(&self, msg: &str) -> InventoryTransaction {
        let mut buf = String::with_capacity(1 << 7);
        println!("---- Entering inventory menu... ----");
        println!("{}", msg);
        let n = msg.lines().count() + 2;
        if self.is_empty() {
            InventoryTransaction::Quit
        } else {
            loop {
                buf.clear();
                print!("👜 ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                match handle.read_line(&mut buf) {
                    Ok(_) => {
                        let _ = crate::readline::clear_last_n_lines(1);
                    }
                    Err(e) => println!("Error in inventory menu readline: {:#?}", e),
                }
                if let Ok(transaction) = buf.parse::<InventoryTransaction>() {
                    let _ = crate::readline::clear_last_n_lines(n);
                    break transaction;
                }
            }
        }
    }
    pub fn pop_item(&mut self, kind: Consumable) -> Option<Consumable> {
        self.0.pop_item(kind)
    }
    pub fn pop_multiple(&mut self, kind: Consumable, n: usize) -> Option<(Consumable, usize)> {
        self.0.pop_multiple(kind, n)
    }
    pub fn drop_multiple(&mut self, kind: Consumable, n: usize) {
        self.0.drop_multiple(kind, n);
    }
    pub fn drop_item(&mut self, kind: Consumable) {
        self.0.pop_item(kind);
    }
    pub fn push_multiple(&mut self, kind: Consumable, count: usize) {
        self.0.push_multiple(kind, count);
    }
    pub fn push(&mut self, kind: Consumable) {
        self.0.push(kind);
    }
    pub fn n_available(&self, kind: &Consumable) -> usize {
        self.0.n_available(kind)
    }
    pub fn push_loot(&mut self, loot: Loot) {
        self.push_multiple(loot.item, loot.amount);
    }
    pub(crate) fn fmt_imp<T: fmt::Write>(&self, f: &mut T, field2: &'static str) -> fmt::Result {
        if self.is_empty() {
            writeln!(f, "Inventory is empty!")?;
        } else {
            writeln!(f, "{}:", Style::new().bold().underline().paint("Inventory"))?;
            writeln!(
                f,
                "                          | {} |  {}  |  {}",
                Style::new().underline().paint("available"),
                Style::new().underline().paint(field2),
                Style::new().underline().paint("effect"),
            )?;
            for (item, count) in self.0.bag.iter().filter(|(_, count)| **count > 0) {
                writeln!(
                    f,
                    "    {:<30} | {:^9} | {:>2} {} | {:<30}",
                    format!("{}", item),
                    count,
                    item.cost(),
                    Colour::Yellow.bold().paint("gold"),
                    item.description(),
                )?;
            }
        }
        Ok(())
    }
}
impl FromIterator<(Consumable, usize)> for Inventory {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Consumable, usize)>,
    {
        Self(iter.into_iter().collect())
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_imp(f, "value")
    }
}
