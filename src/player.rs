use crate::consumable::*;
use crate::equipment::*;
use crate::inventory::*;
use crate::item::equipment_bag::*;
use crate::item::*;
use crate::loot::Loot;
use crate::melee::*;
use crate::monster::*;
use crate::resource::*;
use crate::spell::*;
use crate::trade::*;
use std::fmt::{self, Write};
use yansi::{Paint, Painted};

pub(crate) const PLAYER_GOLD: usize = 25;
pub(crate) const PLAYER_STRENGTH: i64 = 1;
pub(crate) const PLAYER_INTELLECT: i64 = 1;
pub(crate) const PLAYER_LEVEL: usize = 1;
pub(crate) const PLAYER_XP: usize = 0;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub(crate) hp: Health,
    pub(crate) mp: Mana,
    pub(crate) tp: Technical,
    pub(crate) inventory: Inventory,
    pub(crate) gold: usize,
    pub(crate) strength: i64,
    pub(crate) intellect: i64,
    pub(crate) equipment: Equipment,
    pub(crate) equipment_bag: EquipmentBag,
    pub(crate) level: usize,
    pub(crate) xp: usize,
}
// const R: f64 = 18466.496523378733; // -12800.0 / (0.5_f64).ln();
// const R: f64 = 9233.248261689367; // -6400.0 / (0.5_f64).ln();
// const R: f64 = 4616.624130844683; // -3200.0 / (0.5_f64).ln();
const R: f64 = 2308.3120654223417; // -1600.0 / (0.5_f64).ln();
pub(crate) fn level(x: usize) -> usize {
    (10.0 * (1.0 - (-(x as f64) / R).exp())).ceil().max(1.0) as usize
}
pub(crate) fn max_xp_at_level(level: usize) -> usize {
    let y = (level as f64) / 10.0;
    (-((1.0 - y).ln()) * R).floor() as usize
}

pub(crate) fn xp_to_next_level(x: usize) -> usize {
    let level = level(x);
    if level == 10 {
        0
    } else {
        let thresh = max_xp_at_level(level);
        thresh - x + 1
    }
}
const LEVEL: Painted<&'static str> = Painted::new("Level").bold().underline();

impl Player {
    pub fn new() -> Self {
        Self {
            hp: Health::default(),
            mp: Mana::default(),
            tp: Technical::default(),
            inventory: Inventory::new_player(),
            gold: PLAYER_GOLD,
            strength: PLAYER_STRENGTH,
            intellect: PLAYER_INTELLECT,
            equipment: Equipment::default(),
            equipment_bag: EquipmentBag::new_player(),
            level: PLAYER_LEVEL,
            xp: PLAYER_XP,
        }
    }
    pub(crate) fn update_level(&mut self) {
        let new_level = level(self.xp);
        if self.level < new_level {
            println!("You are now {} {}!", LEVEL, new_level);
            self.level = new_level;
            let new_hp = PLAYER_HP * self.level as i64;
            self.hp = Health::new(new_hp);
            self.mp.restore_all();
            self.strength = PLAYER_STRENGTH * self.level as i64;
            self.intellect = PLAYER_INTELLECT * self.level as i64;
        }
    }
    pub(crate) fn level(&self) -> usize {
        self.level
    }
    pub fn restore_hp(&mut self, amount: i64) {
        self.hp.restore(amount);
    }
    pub fn restore_mp(&mut self, amount: i64) {
        self.mp.restore(amount);
    }
    pub fn restore_tp(&mut self, amount: i64) {
        self.tp.restore(amount);
    }
    pub fn strength(&self) -> i64 {
        self.strength + self.equipment.strength()
    }
    pub fn intellect(&self) -> i64 {
        self.intellect + self.equipment.intellect()
    }
    pub fn armor(&self) -> i64 {
        self.equipment.armor()
    }
    pub fn is_alive(&self) -> bool {
        self.hp.is_alive()
    }
    pub fn revive(&mut self) {
        self.hp.restore_all();
        self.mp.restore_all();
        self.tp.drain_all();
    }
    pub fn receive_damage(&mut self, amount: i64) {
        self.hp.receive_damage(amount);
    }
    pub fn armor_reduction(&self, damage: i64) -> i64 {
        const U: i64 = 10; // maximum armor
        let a = self.armor();
        (U - a) * damage / U
    }
    pub fn receive_melee_attack(&mut self, monster: &mut Monster) {
        match monster.kind {
            MonsterKind::Fairy => {
                println!(
                    "The {} heals you for {} {}!",
                    MonsterKind::Fairy,
                    monster.strength.abs().magenta(),
                    Health::HP
                );
                self.receive_damage(monster.strength);
            }
            kind => {
                let attack = monster.produce_melee_attack();
                let amount = self.armor_reduction(attack.damage);
                let melee = attack.kind;
                println!(
                    "The {kind}'s {melee} attack hits you for {} damage!",
                    amount.magenta()
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
                let prev = self.hp.current;
                self.restore_hp(amount);
                let amount = self.hp.current - prev;
                println!(
                    "Your {kind} heals you for {} {}!",
                    amount.magenta(),
                    Health::HP,
                );
            }
            Meditate => {
                let amount = spell.mana_restore();
                let prev = self.mp.current;
                self.restore_mp(amount);
                let amount = self.mp.current - prev;
                println!(
                    "Your {kind} restores {} of your {}!",
                    amount.magenta(),
                    Mana::MP,
                );
            }
        }
    }

    pub fn cast_spell(&mut self, spell: SpellCast) -> Option<SpellCast> {
        self.mp.cast_spell(spell)
    }
    pub fn cast_melee(&mut self, melee: MeleeAttack) -> Option<MeleeAttack> {
        self.tp.cast_melee(melee)
    }

    pub fn consume(&mut self, item: Consumable) {
        let prev_hp = self.hp.current;
        let prev_mp = self.mp.current;
        self.hp.restore(item.healing());
        self.mp.restore(item.mana_restore());
        let heal_amt = Painted::new(self.hp.current - prev_hp).magenta();
        let mana_amt = Painted::new(self.mp.current - prev_mp).magenta();
        match item {
            Food => println!(
                "Your {item} heals you for {heal_amt} {} and restores {mana_amt} of your {}!",
                Health::HP,
                Mana::MP
            ),
            HealthPotion => println!("Your {item} heals you for {heal_amt} {}!", Health::HP),
            ManaPotion => println!("Your {item} heals you for {mana_amt} {}!", Mana::MP),
        }
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
    pub fn equip(&mut self, item: Gear) -> Gear {
        self.equipment.equip(item)
    }
    pub fn unequip(&mut self, item: Gear) -> Gear {
        self.equipment.unequip(item)
    }
    pub fn visit_equipment(&mut self) -> bool {
        match self.equipment_bag.menu(&self.equipment_message()) {
            EquipmentTransaction::Equip(item) => {
                if let Some(item) = self.equipment_bag.pop_item(item) {
                    let item = self.equip(item);
                    self.equipment_bag.push(item);
                }
                true
            }
            EquipmentTransaction::Unequip(item) => {
                let item = self.unequip(item);
                self.equipment_bag.push(item);
                true
            }
            EquipmentTransaction::Quit => false,
        }
    }
    pub fn noncombat_inventory(&mut self) {
        while self.visit_inventory() {}
    }
    pub fn noncombat_equipment(&mut self) {
        while self.visit_equipment() {}
    }
    pub fn acquire(&mut self, loot: Loot) {
        self.inventory.push_multiple(loot.item, loot.amount);
        if let Some(gear) = loot.gear {
            self.equipment_bag.push(gear);
        }
    }
    pub fn status(&self) -> String {
        let mut buf = String::with_capacity(1 << 7);
        self.write_status(&mut buf).unwrap();
        buf
    }
    pub fn write_status<T: fmt::Write>(&self, buf: &mut T) -> fmt::Result {
        write!(buf, "{} {} {}", self.hp, self.mp, self.tp)
    }
    pub fn sleep(&mut self) {
        self.revive();
        println!("You feel well-rested!");
    }
    pub fn inventory_message(&self) -> String {
        let mut s = String::with_capacity(1 << 10);
        writeln!(s, "{}: {}", "Gold".bold().underline(), self.gold).unwrap();
        writeln!(s, "{}", self.inventory).unwrap();
        s
    }
    pub fn equipment_message(&self) -> String {
        let mut s = String::with_capacity(1 << 10);
        writeln!(s, "{}", self.equipment,).unwrap();
        writeln!(s, "{}", self.equipment_bag).unwrap();
        s
    }
    pub fn trade_msg(&self) -> String {
        let mut s = String::with_capacity(1 << 10);
        writeln!(s, "{}", self.inventory).unwrap();
        writeln!(s, "{}", self.equipment_bag).unwrap();
        s
    }
    pub fn attribute_message(&self) -> String {
        let mut s = String::with_capacity(1 << 10);
        writeln!(s, "{}: {}", LEVEL, self.level).unwrap();
        writeln!(
            s,
            "{}: {:<10}    ({} until next level)",
            "Experience".bold().underline(),
            self.xp,
            xp_to_next_level(self.xp)
        )
        .unwrap();
        writeln!(s, "💰: {}", self.gold).unwrap();
        self.write_status(&mut s).unwrap();
        s.push('\n');
        writeln!(s, "{}: {}", "STR".bold().underline(), self.strength()).unwrap();
        writeln!(s, "{}: {}", "INT".bold().underline(), self.intellect()).unwrap();
        writeln!(s, "{}: {}", "ARMOR".bold().underline(), self.armor()).unwrap();
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
            Transaction::Sell { item, count } => match item {
                Item::Consumable(x) => self.inventory.n_available(x) >= *count,
                Item::Gear(x) => self.equipment_bag.n_available(x) >= *count,
            },
            Transaction::Quit => true,
        }
    }
    pub fn perform(&mut self, transaction: &Transaction) {
        match transaction {
            Transaction::Buy { item, count } => {
                let cost = transaction.total_cost();
                self.gold -= cost;
                match item {
                    Item::Consumable(x) => self.inventory.push_multiple(*x, *count),
                    Item::Gear(x) => self.equipment_bag.push_multiple(*x, *count),
                }
                match *count {
                    0 => (),
                    1 => println!("You bought 1 {} for {} gold.", item, cost),
                    n => println!("You bought {n} {}s for {} gold.", item, cost),
                }
            }
            Transaction::Sell { item, count } => {
                let cost = transaction.total_cost();
                self.gold += cost;
                match item {
                    Item::Consumable(x) => self.inventory.drop_multiple(*x, *count),
                    Item::Gear(x) => self.equipment_bag.drop_multiple(*x, *count),
                }
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
            Transaction::Sell { item, count } => match item {
                Item::Consumable(x) => {
                    if self.inventory.n_available(x) < *count {
                        println!("Player rejected transaction: insufficient inventory!")
                    }
                }
                Item::Gear(x) => {
                    if self.equipment_bag.n_available(x) < *count {
                        println!("Player rejected transaction: insufficient inventory!")
                    }
                }
            },
            Transaction::Quit => (),
        }
    }
}
