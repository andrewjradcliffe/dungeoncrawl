use crate::utils::is_quit;
use crate::utils::*;
use ansi_term::{Colour, Style};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Spell {
    Stone,
    Fire,
    Cure1,
    Cure2,
    Meditate,
}
pub use Spell::*;

impl Spell {
    pub const fn cost(&self) -> i64 {
        match self {
            Stone => 10,
            Fire => 15,
            Cure1 => 10,
            Cure2 => 25,
            Meditate => 0,
        }
    }
    pub const fn damage(&self) -> i64 {
        match self {
            Stone => 25,
            Fire => 35,
            Cure1 | Cure2 | Meditate => 0,
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
        match self {
            Stone | Fire => println!(
                "    {:>width$} |  {:>6}   | {:>2} {} | {:>2} {}",
                format!("{}", self),
                self.damage(),
                self.cost(),
                *ANSI_MP,
                self.mana_restore(),
                *ANSI_MP,
                width = 40 - self.display_offset()
            ),
            _ => println!(
                "    {:>width$} |{:>6} {}  | {:>2} {} | {:>2} {}",
                format!("{}", self),
                self.healing(),
                *ANSI_HP,
                self.cost(),
                *ANSI_MP,
                self.mana_restore(),
                *ANSI_MP,
                width = 40 - self.display_offset()
            ),
        };
    }
    pub(crate) const fn display_offset(&self) -> usize {
        match self {
            Fire => 3,
            _ => 0,
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
            Cure1 => write!(f, "{}", Colour::RGB(0xff, 0xb6, 0xc1).paint("Cure I")),
            Cure2 => write!(f, "{}", Colour::RGB(0xff, 0xb6, 0xc1).paint("Cure II")),
            Fire => write!(f, "{}", Colour::RGB(0xff, 0x45, 0x00).paint("Fire")),
            Stone => write!(f, "{}", Colour::RGB(0xa9, 0xa9, 0xa9).paint("Stone")),
            Meditate => write!(f, "{}", Colour::RGB(0x6a, 0x6a, 0xcd).paint("Meditate")),
        }
    }
}

pub(crate) fn spell_menu() -> Option<Spell> {
    let mut buf = String::with_capacity(1 << 7);
    println!("---- Entering spell menu... ----");
    println!("{}", Style::new().underline().italic().paint("Offensive"));
    println!(
        "                      |  {}   |  {} |  {}",
        Style::new().underline().paint("damage"),
        Style::new().underline().paint("cost"),
        Style::new().underline().paint("gain"),
    );
    Stone.print_menu_item();
    Fire.print_menu_item();

    println!("{}", Style::new().underline().italic().paint("Defensive"));
    println!(
        "                      |  {}  |  {} |  {}",
        Style::new().underline().paint("healing"),
        Style::new().underline().paint("cost"),
        Style::new().underline().paint("gain"),
    );
    Cure1.print_menu_item();
    Cure2.print_menu_item();
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
