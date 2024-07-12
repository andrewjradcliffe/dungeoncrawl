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
}
pub use Spell::*;

impl Spell {
    pub fn cost(&self) -> i64 {
        match self {
            Cure1 => 10,
            Cure2 => 25,
            Fire => 15,
            Stone => 10,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Cure1 => "restores 25 HP",
            Cure2 => "restores 50 HP",
            Fire => "causes 35 damage",
            Stone => "causes 25 damage",
        }
    }
    pub fn damage(&self) -> i64 {
        match self {
            Cure1 | Cure2 => 0,
            Fire => 35,
            Stone => 25,
        }
    }
    pub fn healing(&self) -> i64 {
        match self {
            Cure1 => 25,
            Cure2 => 50,
            Fire => 0,
            Stone => 0,
        }
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

        if RE_CURE1.is_match(s) {
            Ok(Cure1)
        } else if RE_CURE2.is_match(s) {
            Ok(Cure2)
        } else if RE_FIRE.is_match(s) {
            Ok(Fire)
        } else if RE_STONE.is_match(s) {
            Ok(Stone)
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
        }
    }
}

pub(crate) fn spell_menu() -> Option<Spell> {
    static RE_QUIT: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)^(?:quit|q)$").unwrap());

    let mut buf = String::with_capacity(1 << 7);
    println!("---- Entering spell menu... ----");
    loop {
        buf.clear();
        println!(
            "    {:<30} | {:<30} | cost: {} MP",
            format!("{}", Cure1),
            Cure1.description(),
            Cure1.cost()
        );
        println!(
            "    {:<30} | {:<30} | cost: {} MP",
            format!("{}", Cure2),
            Cure2.description(),
            Cure2.cost()
        );
        println!(
            "    {:<30} | {:<30} | cost: {} MP",
            format!("{}", Fire),
            Fire.description(),
            Fire.cost()
        );
        println!(
            "    {:<30} | {:<30} | cost: {} MP",
            format!("{}", Stone),
            Stone.description(),
            Stone.cost()
        );

        print!("🪄 ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("Error in inventory menu readline: {:#?}", e),
        }

        let s = buf.trim();

        if RE_QUIT.is_match(s) {
            break None;
        } else {
            if let Ok(spell) = s.parse::<Spell>() {
                return Some(spell);
            }
        }
    }
}
