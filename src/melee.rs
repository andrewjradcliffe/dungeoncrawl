use crate::utils::*;
use ansi_term::{Colour, Style};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub(crate) const fn display_offset(&self) -> usize {
        match self {
            Super => 1,
            _ => 0,
        }
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
            Basic => write!(f, "{}", Colour::RGB(0xb0, 0xc4, 0xde).paint("Basic")),
            Power => write!(f, "{}", Colour::RGB(0x70, 0x80, 0x90).paint("Power")),
            Super => write!(f, "{}", Colour::RGB(0x5f, 0x93, 0xa0).paint("Super")),
        }
    }
}

pub(crate) fn melee_menu(strength: i64) -> Option<MeleeAttack> {
    const N: usize = 5;
    let mut buf = String::with_capacity(1 << 7);
    println!("---- Entering melee menu... ----");
    println!(
        "                      |  {}   |  {} |  {}",
        Style::new().underline().paint("damage"),
        Style::new().underline().paint("cost"),
        Style::new().underline().paint("gain"),
    );
    let basic = MeleeAttack::new(Basic, strength);
    let power = MeleeAttack::new(Power, strength);
    let sup = MeleeAttack::new(Super, strength);
    basic.print_menu_item();
    power.print_menu_item();
    sup.print_menu_item();
    loop {
        buf.clear();

        print!("🪓 ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buf) {
            Ok(_) => {
                let _ = crate::readline::clear_last_n_lines(1);
            }
            Err(e) => println!("Error in inventory menu readline: {:#?}", e),
        }

        let s = buf.trim();

        if is_quit(s) {
            let _ = crate::readline::clear_last_n_lines(N);
            break None;
        } else {
            if let Ok(melee) = s.parse::<Melee>() {
                let _ = crate::readline::clear_last_n_lines(N);
                return Some(match melee {
                    Basic => basic,
                    Power => power,
                    Super => sup,
                });
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeleeAttack {
    pub(crate) kind: Melee,
    pub(crate) damage: i64,
}
impl MeleeAttack {
    pub fn new(kind: Melee, strength: i64) -> Self {
        Self {
            kind,
            damage: strength * kind.damage(),
        }
    }
    pub const fn cost(&self) -> i64 {
        self.kind.cost()
    }
    pub const fn gain(&self) -> i64 {
        self.kind.gain()
    }
    pub(crate) fn print_menu_item(&self) {
        println!(
            "    {:>width$} |  {:>6}   | {:>2} {} | {:>2} {}",
            format!("{}", self.kind),
            self.damage,
            self.cost(),
            *ANSI_TP,
            self.gain(),
            *ANSI_TP,
            width = 40 - self.kind.display_offset(),
        );
    }
}
