use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::{self, Write};
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Spell {
    Cure1,
    Cure2,
    Fire,
    Stone,
    Meditate,
}
pub use Spell::*;

use crate::utils::is_quit;

impl Spell {
    pub const fn cost(&self) -> i64 {
        match self {
            Cure1 => 10,
            Cure2 => 25,
            Fire => 15,
            Stone => 10,
            Meditate => 0,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Cure1 => "restores 25 HP",
            Cure2 => "restores 50 HP",
            Fire => "causes 35 damage",
            Stone => "causes 25 damage",
            Meditate => "restores 25 MP",
        }
    }
    pub const fn damage(&self) -> i64 {
        match self {
            Cure1 | Cure2 | Meditate => 0,
            Fire => 35,
            Stone => 25,
        }
    }
    pub const fn healing(&self) -> i64 {
        match self {
            Cure1 => 25,
            Cure2 => 50,
            Fire | Stone | Meditate => 0,
        }
    }
    pub const fn mana_restore(&self) -> i64 {
        match self {
            Meditate => 25,
            _ => 0,
        }
    }
    pub(crate) fn print_menu_item(&self) {
        println!(
            "    {:<30} | {:<30} | cost: {} MP",
            format!("{}", self),
            self.description(),
            self.cost()
        );
    }
}

impl FromStr for Spell {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_CURE1: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?i)^(?:c\s*[1i]|cure\s*[1i])$").unwrap());
        static RE_CURE2: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?i)^(?:c\s*(?:2|ii)|cure\s*(?:2|ii))$").unwrap());
        static RE_FIRE: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:fire|f)$").unwrap());
        static RE_STONE: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:stone|s)$").unwrap());
        static RE_MED: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:meditate|m)$").unwrap());

        if RE_CURE1.is_match(s) {
            Ok(Cure1)
        } else if RE_CURE2.is_match(s) {
            Ok(Cure2)
        } else if RE_FIRE.is_match(s) {
            Ok(Fire)
        } else if RE_STONE.is_match(s) {
            Ok(Stone)
        } else if RE_MED.is_match(s) {
            Ok(Meditate)
        } else {
            Err(s.to_string())
        }
    }
}

impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cure1 => write!(f, "Cure I"),
            Cure2 => write!(f, "Cure II"),
            Fire => write!(f, "Fire"),
            Stone => write!(f, "Stone"),
            Meditate => write!(f, "Meditate"),
        }
    }
}

pub(crate) fn spell_menu() -> Option<Spell> {
    let mut buf = String::with_capacity(1 << 7);
    println!("---- Entering spell menu... ----");
    Cure1.print_menu_item();
    Cure2.print_menu_item();
    Fire.print_menu_item();
    Stone.print_menu_item();
    Meditate.print_menu_item();

    loop {
        buf.clear();

        print!("🪄 ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("Error in inventory menu readline: {:#?}", e),
        }

        let s = buf.trim();

        if is_quit(s) {
            break None;
        } else {
            if let Ok(spell) = s.parse::<Spell>() {
                return Some(spell);
            }
        }
    }
}
