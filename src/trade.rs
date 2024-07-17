use crate::inventory::*;
use crate::item::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Merchant {
    inventory: Inventory,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum TradeAction {
    Buy,
    Sell,
    Quit,
}

impl FromStr for TradeAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_BUY: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:buy|b)$").unwrap());
        static RE_SELL: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:sell|s)$").unwrap());
        static RE_QUIT: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:quit|q)$").unwrap());

        if RE_BUY.is_match(s) {
            Ok(TradeAction::Buy)
        } else if RE_SELL.is_match(s) {
            Ok(TradeAction::Sell)
        } else if RE_QUIT.is_match(s) {
            Ok(TradeAction::Quit)
        } else {
            Err(s.to_string())
        }
    }
}

impl Merchant {
    pub fn new() -> Self {
        let inventory = [(HealthPotion, 10), (ManaPotion, 10), (Food, 20)]
            .into_iter()
            .collect();
        Self { inventory }
    }
    pub fn buy(&mut self, item: Item) -> Option<Item> {
        self.inventory.pop_item(item)
    }
    pub fn buy_multiple(&mut self, item: Item, n: usize) -> Option<DuplicatedItem> {
        self.inventory.pop_multiple(item, n)
    }
    pub fn menu(&mut self) -> Option<Item> {
        let mut buf = String::with_capacity(1 << 7);
        println!("---- Browsing merchant's wares... ----");
        loop {
            buf.clear();

            println!("{}", self.inventory);
            print!("🧞 ");
            io::Write::flush(&mut io::stdout()).unwrap();

            let stdin = io::stdin();
            let mut handle = stdin.lock();
            match handle.read_line(&mut buf) {
                Ok(_) => (),
                Err(e) => println!("Error in inventory menu readline: {:#?}", e),
            }
            let s = buf.trim();

            if let Some((lhs, rhs)) = s.split_once(' ') {
                if let Ok(action) = lhs.parse::<TradeAction>() {
                    if let Ok(item) = rhs.parse::<Item>() {
                        match action {
                            TradeAction::Buy => return self.buy(item),
                            TradeAction::Sell => return None,
                            TradeAction::Quit => (),
                        }
                    }
                }
            } else {
                if let Ok(TradeAction::Quit) = s.parse::<TradeAction>() {
                    break None;
                }
            }
        }
    }
}
