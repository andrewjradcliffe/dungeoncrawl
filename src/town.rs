use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, BufRead};
use std::str::FromStr;

pub enum TownAction {
    Trade,
    Adventure,
}

impl FromStr for TownAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_TRADE: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:trade|t)$").unwrap());
        static RE_ADV: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:adventure|a)$").unwrap());

        if RE_TRADE.is_match(s) {
            Ok(TownAction::Trade)
        } else if RE_ADV.is_match(s) {
            Ok(TownAction::Adventure)
        } else {
            Err(s.to_string())
        }
    }
}

pub fn town_menu() -> TownAction {
    let mut buf = String::with_capacity(1 << 10);
    println!("==== Entering the town... ====");
    loop {
        buf.clear();
        println!("TRADE or ADVENTURE?");

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("Error in town menu readline: {:#?}", e),
        }

        let s = buf.trim();
        if let Ok(action) = s.parse::<TownAction>() {
            return action;
        }
    }
}
