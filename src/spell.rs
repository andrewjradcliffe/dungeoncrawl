use std::fmt::{self, Write};
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Spell {
    Heal,
    Fire,
    Stone,
}
pub use Spell::*;

impl Spell {
    pub fn cost(&self) -> i64 {
        match self {
            Heal => 25,
            Fire => 15,
            Stone => 10,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Heal => "restores 50 HP",
            Fire => "causes 35 damage",
            Stone => "causes 25 damage",
        }
    }
    pub fn damage(&self) -> i64 {
        match self {
            Heal => 0,
            Fire => 35,
            Stone => 25,
        }
    }
    pub fn healing(&self) -> i64 {
        match self {
            Heal => 50,
            Fire => 0,
            Stone => 0,
        }
    }
}

impl FromStr for Spell {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("heal") {
            Ok(Heal)
        } else if s.eq_ignore_ascii_case("fire") {
            Ok(Fire)
        } else if s.eq_ignore_ascii_case("stone") {
            Ok(Stone)
        } else {
            Err(s.to_string())
        }
    }
}

impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Heal => write!(f, "Heal"),
            Fire => write!(f, "Fire"),
            Stone => write!(f, "Stone"),
        }
    }
}

pub(crate) fn spell_menu() -> Option<Spell> {
    let mut buf = String::with_capacity(1 << 7);
    println!("---- Entering spell menu... ----");
    loop {
        buf.clear();
        println!(
            "    {:<30} | {:<30} | cost: {} MP",
            format!("{}", Heal),
            Heal.description(),
            Heal.cost()
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

        if s.eq_ignore_ascii_case("q") || s.eq_ignore_ascii_case("quit") {
            break None;
        } else {
            if let Ok(spell) = s.parse::<Spell>() {
                return Some(spell);
            }
        }
    }
}
