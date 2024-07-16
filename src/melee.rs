use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::{self, Write};
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Melee {
    Basic,
    Power,
    Super,
}
pub use Melee::*;

impl Melee {
    pub const fn cost(&self) -> i64 {
        match self {
            Basic => 0,
            Power => 35,
            Super => 100,
        }
    }
    pub const fn gain(&self) -> i64 {
        match self {
            Basic => 10,
            Power | Super => 5,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Basic => "Causes 10 damage; gain 10 TP",
            Power => "Causes 35 damage; gain 5 TP",
            Super => "Causes 70 damage; gain 5 TP",
        }
    }
    pub const fn damage(&self) -> i64 {
        match self {
            Basic => 10,
            Power => 35,
            Super => 70,
        }
    }
    pub(crate) fn print_menu_item(&self) {
        println!(
            "    {:<30} | {:<30} | cost: {} TP",
            format!("{}", self),
            self.description(),
            self.cost(),
        );
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
    loop {
        buf.clear();
        Basic.print_menu_item();
        Power.print_menu_item();
        Super.print_menu_item();

        print!("🪓 ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("Error in inventory menu readline: {:#?}", e),
        }

        let s = buf.trim();

        if s.eq_ignore_ascii_case("q") || s.eq_ignore_ascii_case("quit") {
            break None;
        } else {
            if let Ok(melee) = s.parse::<Melee>() {
                return Some(melee);
            }
        }
    }
}
