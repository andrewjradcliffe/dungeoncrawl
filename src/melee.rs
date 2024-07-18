use crate::utils::*;
use ansi_term::Style;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Melee {
    Basic,
    Power,
    Super,
}
pub use Melee::*;

use crate::utils::is_quit;

impl Melee {
    pub const fn damage(&self) -> i64 {
        match self {
            Basic => 10,
            Power => 35,
            Super => 70,
        }
    }
    pub const fn cost(&self) -> i64 {
        match self {
            Basic => 0,
            Power => 35,
            Super => 60,
        }
    }
    pub const fn gain(&self) -> i64 {
        match self {
            Basic => 10,
            Power | Super => 5,
        }
    }
    pub(crate) fn description_imp(&self) -> String {
        format!(
            "{:>6} | {:>2} {} | {:>2} {}",
            self.damage(),
            self.gain(),
            *ANSI_TP,
            self.cost(),
            *ANSI_TP,
        )
    }

    pub fn description(&self) -> &String {
        static BASIC: Lazy<String> = Lazy::new(|| Basic.description_imp());
        static POWER: Lazy<String> = Lazy::new(|| Power.description_imp());
        static SUPER: Lazy<String> = Lazy::new(|| Super.description_imp());
        match self {
            Basic => &*BASIC,
            Power => &*POWER,
            Super => &*SUPER,
        }
    }
    pub(crate) fn print_menu_item(&self) {
        println!("    {:<30} | {}", format!("{}", self), self.description(),);
    }
}

impl FromStr for Melee {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_BASIC: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:basic|b)$").unwrap());
        static RE_POWER: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:power|p)$").unwrap());
        static RE_SUPER: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:super|s)$").unwrap());

        if RE_BASIC.is_match(s) {
            Ok(Basic)
        } else if RE_POWER.is_match(s) {
            Ok(Power)
        } else if RE_SUPER.is_match(s) {
            Ok(Super)
        } else {
            Err(s.to_string())
        }
    }
}

impl fmt::Display for Melee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Basic => write!(f, "Basic"),
            Power => write!(f, "Power"),
            Super => write!(f, "Super"),
        }
    }
}

pub(crate) fn melee_menu() -> Option<Melee> {
    let mut buf = String::with_capacity(1 << 7);
    println!("---- Entering melee menu... ----");
    println!(
        "                                   | {} |  {} |  {}",
        Style::new().underline().paint("damage"),
        Style::new().underline().paint("gain"),
        Style::new().underline().paint("cost"),
    );
    Basic.print_menu_item();
    Power.print_menu_item();
    Super.print_menu_item();
    loop {
        buf.clear();

        print!("🪓 ");
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
            if let Ok(melee) = s.parse::<Melee>() {
                return Some(melee);
            }
        }
    }
}
