use once_cell::sync::Lazy;
use regex::Regex;
// use regex::{RegexSet, RegexSetBuilder};
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum AdventureAction {
    Encounter,
    Town,
    Inventory,
}
use AdventureAction::*;

impl AdventureAction {
    pub fn description(&self) -> &'static str {
        match self {
            Encounter => "Engage a random monster in combat",
            Town => "Visit the town",
            Inventory => "Open inventory",
        }
    }
    pub(crate) fn print_menu_item(&self) {
        println!(
            "    {:<30} | {:<30}",
            format!("{}", self),
            self.description(),
        );
    }
}

impl fmt::Display for AdventureAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Encounter => write!(f, "Encounter"),
            Town => write!(f, "Town"),
            Inventory => write!(f, "Inventory"),
        }
    }
}

impl FromStr for AdventureAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_ENC: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:encounter|e)$").unwrap());
        static RE_TOWN: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:town|t)$").unwrap());
        static RE_INV: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:inventory|i)$").unwrap());

        if RE_ENC.is_match(s) {
            Ok(Encounter)
        } else if RE_TOWN.is_match(s) {
            Ok(Town)
        } else if RE_INV.is_match(s) {
            Ok(Inventory)
        } else {
            Err(s.to_string())
        }
    }
}

pub fn adventure_menu() -> AdventureAction {
    let mut buf = String::with_capacity(1 << 10);
    println!("==== Entering the adventure... ====");
    loop {
        buf.clear();
        Encounter.print_menu_item();
        Town.print_menu_item();
        Inventory.print_menu_item();

        print!("🍁 ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("Error in adventure menu readline: {:#?}", e),
        }

        let s = buf.trim();
        if let Ok(action) = s.parse::<AdventureAction>() {
            return action;
        }
    }
}
