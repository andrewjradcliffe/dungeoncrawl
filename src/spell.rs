use crate::utils::is_quit;
use crate::utils::*;
use ansi_term::{Colour, Style};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Offense {
    Stone,
    Fire,
}
pub use Offense::*;
impl Offense {
    pub const fn cost(&self) -> i64 {
        match self {
            Self::Stone => 10,
            Self::Fire => 15,
        }
    }
    pub const fn damage(&self) -> i64 {
        match self {
            Self::Stone => 25,
            Self::Fire => 35,
        }
    }
    pub const fn healing(&self) -> i64 {
        0
    }
    pub const fn mana_restore(&self) -> i64 {
        0
    }
    pub(crate) const fn display_offset(&self) -> usize {
        match self {
            Self::Fire => 3,
            _ => 0,
        }
    }
}

impl fmt::Display for Offense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fire => write!(f, "{}", Colour::RGB(0xff, 0x45, 0x00).paint("Fire")),
            Self::Stone => write!(f, "{}", Colour::RGB(0xa9, 0xa9, 0xa9).paint("Stone")),
        }
    }
}

impl FromStr for Offense {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_STONE: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:stone|s)$").unwrap());
        static RE_FIRE: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:fire|f)$").unwrap());

        if RE_STONE.is_match(s) {
            Ok(Self::Stone)
        } else if RE_FIRE.is_match(s) {
            Ok(Self::Fire)
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffenseSpell {
    pub(crate) kind: Offense,
    pub(crate) damage: i64,
}
impl OffenseSpell {
    pub fn new(kind: Offense, intellect: i64) -> Self {
        Self {
            kind,
            damage: intellect * kind.damage(),
        }
    }
    pub const fn cost(&self) -> i64 {
        self.kind.cost()
    }
    pub const fn mana_restore(&self) -> i64 {
        self.kind.mana_restore()
    }
    pub(crate) fn print_menu_item(&self) {
        println!(
            "    {:>width$} |  {:>6}   | {:>2} {} | {:>2} {}",
            format!("{}", self.kind),
            self.damage,
            self.cost(),
            *ANSI_MP,
            self.mana_restore(),
            *ANSI_MP,
            width = 40 - self.kind.display_offset()
        )
    }
    pub(crate) fn print_menu_preface() {
        println!("{}", Style::new().underline().italic().paint("Offensive"));
        println!(
            "                      |  {}   |  {} |  {}",
            Style::new().underline().paint("damage"),
            Style::new().underline().paint("cost"),
            Style::new().underline().paint("gain"),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Defense {
    Cure1,
    Cure2,
    Meditate,
}
pub use Defense::*;
impl Defense {
    pub const fn cost(&self) -> i64 {
        match self {
            Self::Cure1 => 10,
            Self::Cure2 => 25,
            _ => 0,
        }
    }
    pub const fn damage(&self) -> i64 {
        0
    }
    pub const fn healing(&self) -> i64 {
        match self {
            Self::Cure1 => 25,
            Self::Cure2 => 50,
            _ => 0,
        }
    }
    pub const fn mana_restore(&self) -> i64 {
        match self {
            Self::Meditate => 25,
            _ => 0,
        }
    }
    pub(crate) const fn display_offset(&self) -> usize {
        0
    }
}
impl fmt::Display for Defense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cure1 => write!(f, "{}", Colour::RGB(0xff, 0xb6, 0xc1).paint("Cure I")),
            Self::Cure2 => write!(f, "{}", Colour::RGB(0xff, 0xb6, 0xc1).paint("Cure II")),
            Self::Meditate => write!(f, "{}", Colour::RGB(0x6a, 0x6a, 0xcd).paint("Meditate")),
        }
    }
}

impl FromStr for Defense {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_CURE1: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?i)^(?:c\s*[1i]|cure\s*[1i])$").unwrap());
        static RE_CURE2: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?i)^(?:c\s*(?:2|ii)|cure\s*(?:2|ii))$").unwrap());
        static RE_MED: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:meditate|m)$").unwrap());

        if RE_CURE1.is_match(s) {
            Ok(Self::Cure1)
        } else if RE_CURE2.is_match(s) {
            Ok(Self::Cure2)
        } else if RE_MED.is_match(s) {
            Ok(Self::Meditate)
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefenseSpell {
    pub(crate) kind: Defense,
    pub(crate) healing: i64,
}
impl DefenseSpell {
    pub fn new(kind: Defense, intellect: i64) -> Self {
        Self {
            kind,
            healing: intellect * kind.healing(),
        }
    }
    pub const fn cost(&self) -> i64 {
        self.kind.cost()
    }
    pub const fn mana_restore(&self) -> i64 {
        self.kind.mana_restore()
    }
    pub(crate) fn print_menu_item(&self) {
        println!(
            "    {:>width$} |{:>6} {}  | {:>2} {} | {:>2} {}",
            format!("{}", self.kind),
            self.healing,
            *ANSI_HP,
            self.cost(),
            *ANSI_MP,
            self.mana_restore(),
            *ANSI_MP,
            width = 40 - self.kind.display_offset()
        )
    }
    pub(crate) fn print_menu_preface() {
        println!("{}", Style::new().underline().italic().paint("Defensive"));
        println!(
            "                      |  {}  |  {} |  {}",
            Style::new().underline().paint("healing"),
            Style::new().underline().paint("cost"),
            Style::new().underline().paint("gain"),
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellCast {
    Offense(OffenseSpell),
    Defense(DefenseSpell),
}
impl SpellCast {
    pub const fn cost(&self) -> i64 {
        match self {
            Self::Offense(spell) => spell.cost(),
            Self::Defense(spell) => spell.cost(),
        }
    }
}

pub(crate) fn spell_menu(intellect: i64) -> Option<SpellCast> {
    let mut buf = String::with_capacity(1 << 7);
    println!("---- Entering spell menu... ----");
    OffenseSpell::print_menu_preface();
    let fire = OffenseSpell::new(Offense::Fire, intellect);
    let stone = OffenseSpell::new(Offense::Stone, intellect);
    fire.print_menu_item();
    stone.print_menu_item();
    DefenseSpell::print_menu_preface();
    let cure1 = DefenseSpell::new(Defense::Cure1, intellect);
    let cure2 = DefenseSpell::new(Defense::Cure2, intellect);
    let meditate = DefenseSpell::new(Defense::Meditate, intellect);
    cure1.print_menu_item();
    cure2.print_menu_item();
    meditate.print_menu_item();

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
            if let Ok(offense) = s.parse::<Offense>() {
                return Some(SpellCast::Offense(match offense {
                    Stone => stone,
                    Fire => fire,
                }));
            } else if let Ok(defense) = s.parse::<Defense>() {
                return Some(SpellCast::Defense(match defense {
                    Cure1 => cure1,
                    Cure2 => cure2,
                    Meditate => meditate,
                }));
            }
        }
    }
}
