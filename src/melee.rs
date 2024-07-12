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
    pub fn cost(&self) -> i64 {
        match self {
            Basic => -5,
            Power => 35,
            Super => 100,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Basic => "Causes 5 damage",
            Power => "Causes 35 damage",
            Super => "Causes 70 damage",
        }
    }
    pub fn damage(&self) -> i64 {
        match self {
            Basic => 5,
            Power => 35,
            Super => 70,
        }
    }
}

impl FromStr for Melee {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("b") || s.eq_ignore_ascii_case("basic") {
            Ok(Basic)
        } else if s.eq_ignore_ascii_case("p") || s.eq_ignore_ascii_case("power") {
            Ok(Power)
        } else if s.eq_ignore_ascii_case("s") || s.eq_ignore_ascii_case("super") {
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
        println!(
            "    {:<30} | {:<30} | cost: {} TP",
            format!("{}", Basic),
            Basic.description(),
            0 // Basic.cost()
        );
        println!(
            "    {:<30} | {:<30} | cost: {} TP",
            format!("{}", Power),
            Power.description(),
            Power.cost()
        );
        println!(
            "    {:<30} | {:<30} | cost: {} TP",
            format!("{}", Super),
            Super.description(),
            Super.cost()
        );

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
