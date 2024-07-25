use once_cell::sync::Lazy;
use regex::Regex;
// use regex::{RegexSet, RegexSetBuilder};
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum TownAction {
    Adventure,
    Dungeon,
    Sleep,
    Trade,
    Inventory,
    Equipment,
    Stats,
}
use TownAction::*;

impl TownAction {
    pub fn description(&self) -> &'static str {
        match self {
            Adventure => "A free-form adventure",
            Dungeon => "A dungeon filled with a random number of monsters",
            Sleep => "Restore all HP and MP; lose any stored TP",
            Trade => "Visit the village merchant",
            Inventory => "Open inventory",
            Equipment => "Open equipment",
            Stats => "Display character statistics",
        }
    }
    pub(crate) fn print_menu_item(&self) {
        println!(
            "    {:<30} | {:<30}",
            format!("{}", self),
            self.description(),
        );
    }
}

impl fmt::Display for TownAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Adventure => write!(f, "Adventure"),
            Dungeon => write!(f, "Dungeon"),
            Sleep => write!(f, "Sleep"),
            Trade => write!(f, "Trade"),
            Inventory => write!(f, "Inventory"),
            Equipment => write!(f, "Equipment"),
            Stats => write!(f, "Stats"),
        }
    }
}

impl FromStr for TownAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // static RE: Lazy<RegexSet> = Lazy::new(|| {
        //     RegexSetBuilder::new([
        //         "^(?:adventure|a)$",
        //         "^(?:gauntlet|g)$",
        //         "^(?:sleep|s)$",
        //         "^(?:trade|t)$",
        //     ])
        //     .case_insensitive(true)
        //     .build()
        //     .unwrap()
        // });

        // match RE.matches(s).into_iter().next() {
        //     Some(0) => Ok(Adventure),
        //     Some(1) => Ok(Gauntlet),
        //     Some(2) => Ok(Sleep),
        //     Some(3) => Ok(Trade),
        //      => Err(s.to_string()),
        // }

        static RE_ADV: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:adventure|a)$").unwrap());
        static RE_GAUNTLET: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:gauntlet|g)$").unwrap());
        static RE_SLEEP: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:sleep|s)$").unwrap());
        static RE_TRADE: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:trade|t)$").unwrap());
        static RE_INV: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:inventory|i)$").unwrap());
        static RE_EQUIP: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:equipment|e)$").unwrap());
        static RE_STATS: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:stats?)$").unwrap());

        if RE_SLEEP.is_match(s) {
            Ok(Sleep)
        } else if RE_TRADE.is_match(s) {
            Ok(Trade)
        } else if RE_ADV.is_match(s) {
            Ok(Adventure)
        } else if RE_GAUNTLET.is_match(s) {
            Ok(Dungeon)
        } else if RE_INV.is_match(s) {
            Ok(Inventory)
        } else if RE_EQUIP.is_match(s) {
            Ok(Equipment)
        } else if RE_STATS.is_match(s) {
            Ok(Stats)
        } else {
            Err(s.to_string())
        }
    }
}

pub fn town_menu() -> TownAction {
    let mut buf = String::with_capacity(1 << 10);
    println!("==== Entering the town... ====");
    Adventure.print_menu_item();
    Dungeon.print_menu_item();
    Sleep.print_menu_item();
    Trade.print_menu_item();
    Inventory.print_menu_item();
    Equipment.print_menu_item();
    Stats.print_menu_item();
    loop {
        buf.clear();
        print!("🌆 ");
        io::Write::flush(&mut io::stdout()).unwrap();

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
