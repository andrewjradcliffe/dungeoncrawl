use crate::inventory::*;
use crate::item::*;
use crate::player::Player;
use crate::utils::*;
use ansi_term::Colour::Yellow;
use ansi_term::Style;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Write;
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

// #[derive(Debug, Clone, PartialEq)]
// pub enum Transaction {
//     Buy{item: Item, count: usize},
//     Sell{item: Item, count: usize},
//     Quit,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Assessment {
//     SufficientGold,
//     SufficientInventory,
//     InsufficientInventory,
//     InsufficientGold,
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub(crate) kind: TradeAction,
    pub(crate) item: Item,
    pub(crate) count: usize,
}
impl Transaction {
    pub fn new(kind: TradeAction, item: Item, count: usize) -> Self {
        Self { kind, item, count }
    }
    pub fn total_cost(&self) -> usize {
        self.item.cost() * self.count
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
        if self.inventory.is_empty() {
            writeln!(s, "Inventory is empty!").unwrap();
        } else {
            writeln!(s, "{}:", Style::new().bold().underline().paint("Inventory")).unwrap();
            writeln!(
                s,
                "                          | {} |  {}  |  {}",
                Style::new().underline().paint("available"),
                Style::new().underline().paint("price"),
                Style::new().underline().paint("effect"),
            )
            .unwrap();
            for (item, count) in self.inventory.bag.iter().filter(|(_, count)| **count > 0) {
                writeln!(
                    s,
                    "    {:<30} | {:^9} | {:>2} {} | {:<30}",
                    format!("{}", item),
                    count,
                    item.cost(),
                    Yellow.bold().paint("gold"),
                    item.description(),
                )
                .unwrap();
            }
        }
        s
    }
    pub fn can_perform(&self, transaction: &Transaction) -> bool {
        match transaction.kind {
            TradeAction::Buy => self.inventory.n_available(&transaction.item) >= transaction.count,
            TradeAction::Sell => true,
            TradeAction::Quit => true,
        }
    }
    pub fn perform(&mut self, transaction: &Transaction) {
        match transaction.kind {
            TradeAction::Buy => {
                self.inventory
                    .drop_multiple(transaction.item, transaction.count);
            }
            TradeAction::Sell => {
                self.inventory
                    .push_multiple(transaction.item, transaction.count);
            }
            TradeAction::Quit => (),
        }
    }
    pub fn describe_rejected_transaction(&self, transaction: &Transaction) {
        match transaction.kind {
            TradeAction::Buy => {
                if self.inventory.n_available(&transaction.item) < transaction.count {
                    println!("Merchant rejected transaction: insufficient inventory!")
                }
            }
            TradeAction::Sell => (),
            TradeAction::Quit => (),
        }
    }
    pub fn menu(&self, gold: usize) -> Transaction {
        let mut buf = String::with_capacity(1 << 7);
        println!("---- Browsing merchant's wares... ----");
        if self.inventory.is_empty() {
            println!("Inventory is empty!");
            Transaction::new(TradeAction::Quit, Item::Food, 0)
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
                let s = buf.trim();

                if let Some((fst, rst)) = s.split_once(' ') {
                    if let Ok(action) = fst.parse::<TradeAction>() {
                        match action {
                            TradeAction::Buy | TradeAction::Sell => {
                                let rst = rst.trim();
                                if let Some((lhs, rhs)) = rst.split_once(' ') {
                                    if let Ok(n) = lhs.parse::<usize>() {
                                        if let Ok(item) = rhs.parse::<Item>() {
                                            return Transaction::new(action, item, n);
                                        }
                                    }
                                } else if let Ok(item) = rst.parse::<Item>() {
                                    return Transaction::new(action, item, 1);
                                }
                            }
                            TradeAction::Quit => (),
                        }
                    }
                } else {
                    if let Ok(TradeAction::Quit) = s.parse::<TradeAction>() {
                        break Transaction::new(TradeAction::Quit, Item::Food, 0);
                    }
                }
            }
        }
    }
    pub fn visit(&mut self, player: &mut Player) -> bool {
        let transaction = self.menu(player.gold.clone());
        match transaction.kind {
            TradeAction::Quit => false,
            TradeAction::Buy | TradeAction::Sell => {
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
