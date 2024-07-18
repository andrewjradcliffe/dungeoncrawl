use crate::combat::Combatant;
use crate::inventory::*;
use crate::item::*;
use crate::loot::Loot;
use crate::melee::Melee;
use crate::spell::Spell;
use crate::trade::*;
use crate::utils::*;
use ansi_term::Style;
use std::fmt::Write;

pub(crate) const PLAYER_HP: i64 = 100;
pub(crate) const PLAYER_MP: i64 = 100;
pub(crate) const PLAYER_TP: i64 = 100;
pub(crate) const PLAYER_GOLD: usize = 25;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub(crate) current_hp: i64,
    pub(crate) max_hp: i64,
    pub(crate) current_mp: i64,
    pub(crate) max_mp: i64,
    pub(crate) current_tp: i64,
    pub(crate) max_tp: i64,
    pub(crate) inventory: Inventory,
    pub(crate) gold: usize,
}

impl Combatant for Player {
    fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
    fn receive_damage(&mut self, amount: i64) {
        self.current_hp = (self.current_hp - amount).clamp(0, self.max_hp);
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            current_hp: PLAYER_HP,
            max_hp: PLAYER_HP,
            current_mp: PLAYER_MP,
            max_mp: PLAYER_MP,
            current_tp: 0,
            max_tp: PLAYER_TP,
            inventory: Inventory::new_player(),
            gold: PLAYER_GOLD,
        }
    }
    pub fn restore_hp(&mut self, amount: i64) {
        self.current_hp = (self.current_hp + amount).clamp(0, self.max_hp);
    }
    pub fn restore_mp(&mut self, amount: i64) {
        self.current_mp = (self.current_mp + amount).clamp(0, self.max_mp);
    }
    pub fn restore_tp(&mut self, amount: i64) {
        self.current_tp = (self.current_tp + amount).clamp(0, self.max_tp);
    }

    pub fn cast_spell(&mut self, spell: Spell) -> Option<Spell> {
        let cost = spell.cost();
        if self.current_mp >= cost {
            self.current_mp = (self.current_mp - cost).clamp(0, self.max_mp);
            Some(spell)
        } else {
            None
        }
    }
    pub fn cast_melee(&mut self, melee: Melee) -> Option<Melee> {
        let cost = melee.cost();
        let gain = melee.gain();
        if self.current_tp >= cost {
            self.current_tp = (self.current_tp - cost + gain).clamp(0, self.max_tp);
            Some(melee)
        } else {
            None
        }
    }

    pub fn visit_inventory(&mut self) {
        if let Some(item) = self.inventory.menu(&self.inventory_message()) {
            self.consume(item);
        }
    }
    pub fn consume(&mut self, item: Item) {
        match item {
            HealthPotion => self.restore_hp(25),
            ManaPotion => self.restore_mp(25),
            Food => {
                self.restore_hp(10);
                self.restore_mp(10);
            }
        }
    }
    pub fn acquire(&mut self, loot: Loot) {
        self.inventory.push_loot(loot)
    }
    pub fn status(&self) -> String {
        let mut buf = String::with_capacity(1 << 7);
        self.write_status(&mut buf);
        buf
    }
    pub fn write_hp(&self, buf: &mut String) {
        let hp = format!("{}", self.current_hp);
        write!(
            buf,
            "{}[{}/{}]",
            *ANSI_HP,
            Style::new().italic().paint(hp),
            self.max_hp
        )
        .unwrap();
    }
    pub fn write_mp(&self, buf: &mut String) {
        let mp = format!("{}", self.current_mp);
        write!(
            buf,
            "{}[{}/{}]",
            *ANSI_MP,
            Style::new().italic().paint(mp),
            self.max_mp
        )
        .unwrap();
    }
    pub fn write_tp(&self, buf: &mut String) {
        let tp = format!("{}", self.current_tp);
        write!(
            buf,
            "{}[{}/{}]",
            *ANSI_TP,
            Style::new().italic().paint(tp),
            self.max_tp
        )
        .unwrap();
    }
    pub fn write_status(&self, buf: &mut String) {
        self.write_hp(buf);
        write!(buf, " ").unwrap();
        self.write_mp(buf);
        write!(buf, " ").unwrap();
        self.write_tp(buf);
    }
    pub fn sleep(&mut self) {
        self.current_hp = self.max_hp;
        self.current_mp = self.max_mp;
        self.current_tp = 0;
    }
    pub fn inventory_message(&self) -> String {
        let mut s = String::with_capacity(1 << 10);
        writeln!(
            s,
            "{}: {}",
            Style::new().bold().underline().paint("Gold"),
            self.gold
        )
        .unwrap();
        if self.inventory.is_empty() {
            writeln!(s, "Inventory is empty!").unwrap();
        } else {
            writeln!(s, "{}:", Style::new().bold().underline().paint("Bag")).unwrap();
            for (item, count) in self.inventory.bag.iter().filter(|(_, count)| **count > 0) {
                writeln!(
                    s,
                    "    {:<30} x{:<4} | {}",
                    format!("{}", item),
                    count,
                    item.description()
                )
                .unwrap();
            }
        }
        s
    }
    // pub fn assess_transaction(&self, transaction: &Transaction) -> Assessment {
    //     match transaction.kind {
    //         TradeAction::Buy => {
    //             if self.gold >= transaction.total_cost() {
    //                 Assessment::SufficientGold
    //             } else {
    //                 Assessment::InsufficientGold
    //             }
    //         }
    //         TradeAction::Sell => {
    //             if self.inventory.n_available(&transaction.item) >= transaction.count {
    //                 Assessment::SufficientInventory
    //             } else {
    //                 Assessment::InsufficientInventory
    //             }
    //         }
    //         TradeAction::Quit => true,
    //     }
    // }
    pub fn can_perform(&self, transaction: &Transaction) -> bool {
        match transaction.kind {
            TradeAction::Buy => self.gold >= transaction.total_cost(),
            TradeAction::Sell => self.inventory.n_available(&transaction.item) >= transaction.count,
            TradeAction::Quit => true,
        }
    }
    pub fn perform(&mut self, transaction: &Transaction) {
        match transaction.kind {
            TradeAction::Buy => {
                let cost = transaction.total_cost();
                self.gold -= cost;
                self.inventory
                    .push_multiple(transaction.item, transaction.count);
                match transaction.count {
                    0 => (),
                    1 => println!("You bought 1 {} for {} gold.", transaction.item, cost),
                    n => println!("You bought {n} {}s for {} gold.", transaction.item, cost),
                }
            }
            TradeAction::Sell => {
                let cost = transaction.total_cost();
                self.gold += cost;
                self.inventory
                    .drop_multiple(transaction.item, transaction.count);
                match transaction.count {
                    0 => (),
                    1 => println!("You sold 1 {} for {} gold.", transaction.item, cost),
                    n => println!("You sold {n} {}s for {} gold.", transaction.item, cost),
                }
            }
            TradeAction::Quit => (),
        }
    }
    pub fn describe_rejected_transaction(&self, transaction: &Transaction) {
        match transaction.kind {
            TradeAction::Buy => println!("Player rejected transaction: insufficient gold!"),
            TradeAction::Sell => {
                if self.inventory.n_available(&transaction.item) <= transaction.count {
                    println!("Player rejected transaction: insufficient inventory!")
                }
            }
            TradeAction::Quit => (),
        }
    }
}
