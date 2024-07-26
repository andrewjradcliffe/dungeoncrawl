use crate::consumable::*;
use crate::inventory::*;
use crate::item::equipment_bag::EquipmentBag;
use crate::item::*;
use crate::player::Player;
use crate::utils::*;
use regex::Regex;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub struct Merchant {
    inventory: Inventory,
    equipment_bag: EquipmentBag,
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

        static RE_BUY: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)^(?:buy|b)$").unwrap());
        static RE_SELL: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)^(?:sell|s)$").unwrap());

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
        Self {
            inventory,
            equipment_bag: EquipmentBag::new_merchant(),
        }
    }
    pub fn inventory_message(&self) -> String {
        let mut s = String::with_capacity(1 << 10);
        self.inventory.fmt_imp(&mut s, "price").unwrap();
        s.push('\n');
        self.equipment_bag.fmt_imp(&mut s, "price").unwrap();
        s
    }
    pub fn can_perform(&self, transaction: &Transaction) -> bool {
        match transaction {
            Transaction::Buy {
                item: Item::Consumable(x),
                count,
            } => self.inventory.n_available(x) >= *count,
            Transaction::Buy {
                item: Item::Gear(x),
                count,
            } => self.equipment_bag.n_available(x) >= *count,
            _ => true,
        }
    }
    pub fn perform(&mut self, transaction: &Transaction) {
        match transaction {
            Transaction::Buy { item, count } => match item {
                Item::Consumable(x) => self.inventory.drop_multiple(*x, *count),
                Item::Gear(x) => self.equipment_bag.drop_multiple(*x, *count),
            },
            Transaction::Sell { item, count } => match item {
                Item::Consumable(x) => self.inventory.push_multiple(*x, *count),
                Item::Gear(x) => self.equipment_bag.push_multiple(*x, *count),
            },
            Transaction::Quit => (),
        }
    }
    pub fn describe_rejected_transaction(&self, transaction: &Transaction) {
        match transaction {
            Transaction::Buy { item, count } => match item {
                Item::Consumable(x) => {
                    if self.inventory.n_available(x) < *count {
                        println!("Merchant rejected transaction: insufficient inventory!")
                    }
                }
                Item::Gear(x) => {
                    if self.equipment_bag.n_available(x) < *count {
                        println!("Merchant rejected transaction: insufficient inventory!")
                    }
                }
            },
            _ => (),
        }
    }
    pub fn menu(&self, gold: usize, player_msg: &str) -> Transaction {
        let mut buf = String::with_capacity(1 << 7);
        println!("---- Browsing merchant's wares... ----");
        let msg = self.inventory_message();
        let n = msg.lines().count() + 2;
        println!("{}", msg);
        let n = n + player_msg.lines().count() + 2;
        println!("---- Your items... ----");
        println!("{}", player_msg);
        loop {
            buf.clear();

            print!("(💰: {}) 🧞 ", gold);
            io::Write::flush(&mut io::stdout()).unwrap();

            let stdin = io::stdin();
            let mut handle = stdin.lock();
            match handle.read_line(&mut buf) {
                Ok(_) => {
                    let _ = crate::readline::clear_last_n_lines(1);
                }
                Err(e) => println!("Error in inventory menu readline: {:#?}", e),
            }
            if let Ok(transaction) = buf.parse::<Transaction>() {
                let _ = crate::readline::clear_last_n_lines(n);
                break transaction;
            }
        }
    }
    pub fn visit(&mut self, player: &mut Player) -> bool {
        let transaction = self.menu(player.gold.clone(), &player.trade_msg());
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
            test!(TradeAction::Buy, "b", Item::from(HealthPotion) ; "hp", "HP", "health potion", "hEalth poTION");
            test!(TradeAction::Buy, "b", Item::from(ManaPotion) ; "mp", "MP", "mana potion", "MaNA poTION");
            test!(TradeAction::Buy, "b", Item::from(Food) ; "f", "F", "food", "FOOd");
            test!(TradeAction::Sell, "s", Item::from(HealthPotion) ; "hp", "HP", "health potion", "hEalth poTION");
            test!(TradeAction::Sell, "s", Item::from(ManaPotion) ; "mp", "MP", "mana potion", "MaNA poTION");
            test!(TradeAction::Sell, "s", Item::from(Food) ; "f", "F", "food", "FOOd");
        }
    }
}
