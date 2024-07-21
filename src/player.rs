use crate::equipment::*;
use crate::inventory::*;
use crate::item::*;
use crate::loot::Loot;
use crate::melee::*;
use crate::monster::*;
use crate::spell::*;
use crate::trade::*;
use crate::utils::*;
use ansi_term::{Colour, Style};
use std::fmt::Write;

pub(crate) const PLAYER_HP: i64 = 100;
pub(crate) const PLAYER_MP: i64 = 100;
pub(crate) const PLAYER_TP: i64 = 100;
pub(crate) const PLAYER_GOLD: usize = 25;
pub(crate) const PLAYER_STRENGTH: i64 = 1;
pub(crate) const PLAYER_INTELLECT: i64 = 1;

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
    pub(crate) strength: i64,
    pub(crate) intellect: i64,
    pub(crate) equipment: Equipment,
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
            strength: PLAYER_STRENGTH,
            intellect: PLAYER_INTELLECT,
            equipment: Equipment::default(),
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
    pub fn strength(&self) -> i64 {
        self.strength + self.equipment.strength()
    }
    pub fn intellect(&self) -> i64 {
        self.intellect + self.equipment.intellect()
    }
    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
    pub fn receive_damage(&mut self, amount: i64) {
        self.current_hp = (self.current_hp - amount).clamp(0, self.max_hp);
    }
    pub fn receive_melee_attack(&mut self, monster: &mut Monster) {
        match monster.kind {
            MonsterKind::Fairy => {
                println!(
                    "The {} heals you for {} {}!",
                    MonsterKind::Fairy,
                    Colour::Purple.paint("20"),
                    *ANSI_HP
                );
                self.receive_damage(monster.strength);
            }
            kind => {
                let attack = monster.produce_melee_attack();
                let amount = attack.damage;
                let melee = attack.kind;
                println!(
                    "The {kind}'s {melee} attack hits you for {} damage!",
                    Colour::Purple.paint(format!("{}", amount)),
                );
                self.receive_damage(amount);
            }
        }
    }

    pub fn receive_defensive_spell(&mut self, spell: DefenseSpell) {
        let kind = spell.kind;
        match kind {
            Cure1 | Cure2 => {
                let amount = spell.healing;
                println!(
                    "Your {kind} heals you for {} {}!",
                    Colour::Purple.paint(format!("{}", amount)),
                    *ANSI_HP
                );
                self.restore_hp(amount);
            }
            Meditate => {
                let amount = spell.mana_restore();
                println!(
                    "Your {kind} restores {} of your {}!",
                    Colour::Purple.paint(format!("{}", amount)),
                    *ANSI_MP
                );
                self.restore_mp(amount);
            }
        }
    }

    pub fn cast_spell(&mut self, spell: SpellCast) -> Option<SpellCast> {
        let cost = spell.cost();
        if self.current_mp >= cost {
            self.current_mp = (self.current_mp - cost).clamp(0, self.max_mp);
            Some(spell)
        } else {
            None
        }
    }
    pub fn cast_melee(&mut self, melee: MeleeAttack) -> Option<MeleeAttack> {
        let cost = melee.cost();
        let gain = melee.gain();
        if self.current_tp >= cost {
            self.current_tp = (self.current_tp - cost + gain).clamp(0, self.max_tp);
            Some(melee)
        } else {
            None
        }
    }

    pub fn consume(&mut self, item: Consumable) {
        self.restore_hp(item.healing());
        self.restore_mp(item.mana_restore());
        println!("Your {item} {}!", item.combat_description());
    }
    pub fn visit_inventory(&mut self) -> bool {
        match self.inventory.menu(&self.inventory_message()) {
            InventoryTransaction::Use(item) => {
                if let Some(item) = self.inventory.pop_item(item) {
                    self.consume(item);
                }
                true
            }
            InventoryTransaction::Drop(item) => {
                self.inventory.drop_item(item);
                true
            }
            InventoryTransaction::Quit => false,
        }
    }
    pub fn noncombat_inventory(&mut self) {
        while self.visit_inventory() {}
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
        println!("You feel well-rested!");
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
        writeln!(s, "{}", self.inventory).unwrap();
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
        match transaction {
            Transaction::Buy { .. } => self.gold >= transaction.total_cost(),
            Transaction::Sell { item, count } => self.inventory.n_available(item) >= *count,
            Transaction::Quit => true,
        }
    }
    pub fn perform(&mut self, transaction: &Transaction) {
        match transaction {
            Transaction::Buy { item, count } => {
                let cost = transaction.total_cost();
                self.gold -= cost;
                self.inventory.push_multiple(*item, *count);
                match *count {
                    0 => (),
                    1 => println!("You bought 1 {} for {} gold.", item, cost),
                    n => println!("You bought {n} {}s for {} gold.", item, cost),
                }
            }
            Transaction::Sell { item, count } => {
                let cost = transaction.total_cost();
                self.gold += cost;
                self.inventory.drop_multiple(*item, *count);
                match *count {
                    0 => (),
                    1 => println!("You sold 1 {} for {} gold.", item, cost),
                    n => println!("You sold {n} {}s for {} gold.", item, cost),
                }
            }
            Transaction::Quit => (),
        }
    }
    pub fn describe_rejected_transaction(&self, transaction: &Transaction) {
        match transaction {
            Transaction::Buy { .. } => println!("Player rejected transaction: insufficient gold!"),
            Transaction::Sell { item, count } => {
                if self.inventory.n_available(item) < *count {
                    println!("Player rejected transaction: insufficient inventory!")
                }
            }
            Transaction::Quit => (),
        }
    }
}
