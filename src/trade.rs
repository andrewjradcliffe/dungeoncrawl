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

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    action: TradeAction,
    item: Item,
    count: usize,
}
impl Transaction {
    pub fn new(action: TradeAction, item: Item, count: usize) -> Self {
        Self {
            action,
            item,
            count,
        }
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
    pub fn buy(&mut self, item: Item) -> Option<Item> {
        self.inventory.pop_item(item)
    }
    pub fn buy_multiple(&mut self, item: Item, n: usize) -> Option<DuplicatedItem> {
        self.inventory.pop_multiple(item, n)
    }
    pub fn inventory_message(&self) -> String {
        let mut s = String::with_capacity(1 << 10);
        if self.inventory.is_empty() {
            writeln!(s, "Inventory is empty!").unwrap();
        } else {
            writeln!(s, "{}:", Style::new().bold().underline().paint("Inventory")).unwrap();
            for (item, count) in self.inventory.bag.iter().filter(|(_, count)| **count > 0) {
                writeln!(
                    s,
                    "    {:<30} x{:<4} | price: {:>1} {} | {:<30}",
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
    pub fn menu(&self) -> Option<Transaction> {
        let mut buf = String::with_capacity(1 << 7);
        println!("---- Browsing merchant's wares... ----");
        if self.inventory.is_empty() {
            println!("Inventory is empty!");
            None
        } else {
            println!("{}", self.inventory_message());
            loop {
                buf.clear();

                print!("🧞 ",);
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
                                            return Some(Transaction::new(action, item, n));
                                        }
                                    }
                                } else if let Ok(item) = rst.parse::<Item>() {
                                    return Some(Transaction::new(action, item, 1));
                                }
                            }
                            TradeAction::Quit => (),
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
    pub fn visit(&mut self, player: &mut Player) {
        if let Some(transaction) = self.menu() {
            match transaction.action {
                TradeAction::Buy => {
                    let item = transaction.item;
                    let count = transaction.count;
                    let actual_count = count.clamp(0, self.inventory.n_available(&item));
                    let actual_cost = actual_count * item.cost();
                    if actual_cost <= player.gold {
                        player.inventory.push_multiple(item, actual_count);
                        player.gold -= actual_cost;
                        match actual_count {
                            0 => (),
                            1 => println!("You bought 1 {} for {} gold.", item, actual_cost),
                            n => println!("You bought {n} {}s for {} gold.", item, actual_cost),
                        }
                        self.inventory.drop_multiple(item, actual_count);
                    }
                }
                TradeAction::Sell => {
                    let item = transaction.item;
                    let count = transaction.count;
                    let actual_count = count.clamp(0, player.inventory.n_available(&item));
                    let actual_cost = actual_count * item.cost();
                    player.inventory.drop_multiple(item, actual_count);
                    player.gold += actual_cost;
                    match actual_count {
                        0 => (),
                        1 => println!("You sold 1 {} for {} gold.", item, actual_cost),
                        n => println!("You sold {n} {}s for {} gold.", item, actual_cost),
                    }
                    self.inventory.push_multiple(item, actual_count);
                }
                _ => panic!(),
            }
        }
    }
}
