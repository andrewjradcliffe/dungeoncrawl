use crate::inventory::*;
use crate::item::*;
use crate::player::Player;
use crate::utils::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Merchant {
    inventory: Inventory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

        if RE_BUY.is_match(s) {
            Ok(TradeAction::Buy)
        } else if RE_SELL.is_match(s) {
            Ok(TradeAction::Sell)
        } else if is_quit(s) {
            Ok(TradeAction::Quit)
        } else {
            Err(s.to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transaction {
    Buy { item: Item, count: usize },
    Sell { item: Item, count: usize },
    Quit,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Assessment {
//     SufficientGold,
//     SufficientInventory,
//     InsufficientInventory,
//     InsufficientGold,
// }

impl Transaction {
    pub fn new(kind: TradeAction, item: Item, count: usize) -> Self {
        match kind {
            TradeAction::Quit => Transaction::Quit,
            TradeAction::Buy => Transaction::Buy { item, count },
            TradeAction::Sell => Transaction::Sell { item, count },
        }
    }
    pub fn total_cost(&self) -> usize {
        match self {
            Self::Quit => 0,
            Self::Buy { item, count } => item.cost() * count,
            Self::Sell { item, count } => item.cost() * count,
        }
    }
}

impl FromStr for Transaction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if let Some((fst, rst)) = s.split_once(' ') {
            if let Ok(action) = fst.parse::<TradeAction>() {
                match action {
                    TradeAction::Buy | TradeAction::Sell => {
                        let rst = rst.trim();
                        if let Ok(item) = rst.parse::<Item>() {
                            return Ok(Transaction::new(action, item, 1));
                        } else if let Some((lhs, rhs)) = rst.split_once(' ') {
                            if let Ok(n) = lhs.parse::<usize>() {
                                if let Ok(item) = rhs.parse::<Item>() {
                                    return Ok(Transaction::new(action, item, n));
                                }
                            }
                        }
                    }
                    TradeAction::Quit => (),
                }
            }
        } else {
            if let Ok(TradeAction::Quit) = s.parse::<TradeAction>() {
                return Ok(Transaction::Quit);
            }
        }
        Err(s.to_string())
    }
}

impl Merchant {
    pub fn new() -> Self {
        let inventory = [(HealthPotion, 10), (ManaPotion, 10), (Food, 20)]
            .into_iter()
            .collect();
        Self { inventory }
    }
    pub fn inventory_message(&self) -> String {
        let mut s = String::with_capacity(1 << 10);
        self.inventory.fmt_imp(&mut s, "price").unwrap();
        s
    }
    pub fn can_perform(&self, transaction: &Transaction) -> bool {
        match transaction {
            Transaction::Buy { item, count } => self.inventory.n_available(item) >= *count,
            _ => true,
        }
    }
    pub fn perform(&mut self, transaction: &Transaction) {
        match transaction {
            Transaction::Buy { item, count } => {
                self.inventory.drop_multiple(*item, *count);
            }
            Transaction::Sell { item, count } => {
                self.inventory.drop_multiple(*item, *count);
            }
            Transaction::Quit => (),
        }
    }
    pub fn describe_rejected_transaction(&self, transaction: &Transaction) {
        match transaction {
            Transaction::Buy { item, count } => {
                if self.inventory.n_available(item) < *count {
                    println!("Merchant rejected transaction: insufficient inventory!")
                }
            }
            _ => (),
        }
    }
    pub fn menu(&self, gold: usize) -> Transaction {
        let mut buf = String::with_capacity(1 << 7);
        println!("---- Browsing merchant's wares... ----");
        if self.inventory.is_empty() {
            println!("Inventory is empty!");
            Transaction::Quit
        } else {
            println!("{}", self.inventory_message());
            loop {
                buf.clear();

                print!("(💰: {}) 🧞 ", gold);
                io::Write::flush(&mut io::stdout()).unwrap();

                let stdin = io::stdin();
                let mut handle = stdin.lock();
                match handle.read_line(&mut buf) {
                    Ok(_) => (),
                    Err(e) => println!("Error in inventory menu readline: {:#?}", e),
                }
                if let Ok(transaction) = buf.parse::<Transaction>() {
                    break transaction;
                }
            }
        }
    }
    pub fn visit(&mut self, player: &mut Player) -> bool {
        let transaction = self.menu(player.gold.clone());
        match transaction {
            Transaction::Quit => false,
            _ => {
                if self.can_perform(&transaction) {
                    if player.can_perform(&transaction) {
                        self.perform(&transaction);
                        player.perform(&transaction);
                    } else {
                        player.describe_rejected_transaction(&transaction);
                    }
                } else {
                    self.describe_rejected_transaction(&transaction);
                }
                true
            }
        }
    }
    pub fn trade(&mut self, player: &mut Player) {
        while self.visit(player) {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod trade_action {
        use super::*;

        #[test]
        fn from_str() {
            macro_rules! test_eq {
                ($lhs:expr ; $($s:literal),+) => {
                    $(
                        assert_eq!($lhs, $s.parse::<TradeAction>().unwrap());
                    )+
                }
            }
            test_eq!(TradeAction::Buy ; "buy", "b", "BUY", "B", "bUY");
            test_eq!(TradeAction::Sell ; "sell", "s", "SELL", "S", "sElL");
            test_eq!(TradeAction::Quit ; "quit", "q", "QUIT", "Q", "Quit");

            macro_rules! test_err {
                ($($s:literal),+) => {
                    $(
                        assert!($s.parse::<TradeAction>().is_err());
                    )+
                }
            }
            test_err!("a", "c", "bu", "sel", "qui", "1234");
        }
    }
    mod transaction {
        use super::*;

        #[test]
        fn from_str() {
            macro_rules! test {
                ($action:expr, $a:literal, $item:expr ; $($s:literal),+) => {
                    $(
                        assert_eq!(format!("{} {}", $a, $s).parse::<Transaction>().unwrap(), Transaction::new($action, $item, 1));
                        assert_eq!(format!("{} 1 {}", $a, $s).parse::<Transaction>().unwrap(), Transaction::new($action, $item, 1));
                        assert_eq!(format!("{} 5 {}", $a, $s).parse::<Transaction>().unwrap(), Transaction::new($action, $item, 5));
                    )+

                }
            }
            test!(TradeAction::Buy, "b", HealthPotion ; "hp", "HP", "health potion", "hEalth poTION");
            test!(TradeAction::Buy, "b", ManaPotion ; "mp", "MP", "mana potion", "MaNA poTION");
            test!(TradeAction::Buy, "b", Food ; "f", "F", "food", "FOOd");
            test!(TradeAction::Sell, "s", HealthPotion ; "hp", "HP", "health potion", "hEalth poTION");
            test!(TradeAction::Sell, "s", ManaPotion ; "mp", "MP", "mana potion", "MaNA poTION");
            test!(TradeAction::Sell, "s", Food ; "f", "F", "food", "FOOd");
        }
    }
}
