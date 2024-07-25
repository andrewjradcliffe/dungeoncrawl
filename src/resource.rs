use crate::melee::*;
use crate::spell::*;
use std::fmt;
use yansi::{Paint, Painted};

pub(crate) const PLAYER_HP: i64 = 100;
pub(crate) const PLAYER_MP: i64 = 100;
pub(crate) const PLAYER_TP: i64 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Health {
    pub(crate) current: i64,
    pub(crate) max: i64,
}

impl Health {
    pub const HP: Painted<&'static str> = Painted::new("HP").bold().red();
    pub fn new(max: i64) -> Self {
        Self { current: max, max }
    }
    pub fn restore(&mut self, amount: i64) {
        self.current = (self.current + amount).clamp(0, self.max);
    }
    pub fn receive_damage(&mut self, amount: i64) {
        self.current = (self.current - amount).clamp(0, self.max);
    }
    pub fn restore_all(&mut self) {
        self.current = self.max;
    }
    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
    pub fn pct_max(&self, pct: i64) -> i64 {
        (self.max * pct.clamp(0, 100)) / 100
    }
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pct = (self.max * 50) / 100;
        let current = if self.current < pct {
            self.current.italic().on_bright_red().blink()
        } else {
            self.current.italic()
        };
        write!(f, "{}[{}/{}]", Self::HP, current, self.max)
    }
}
impl Default for Health {
    fn default() -> Self {
        Self::new(PLAYER_HP)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mana {
    pub(crate) current: i64,
    pub(crate) max: i64,
}

impl Mana {
    pub const MP: Painted<&'static str> = Painted::new("MP").bold().green();
    pub fn new(max: i64) -> Self {
        Self { current: max, max }
    }
    pub fn restore(&mut self, amount: i64) {
        self.current = (self.current + amount).clamp(0, self.max);
    }
    pub fn restore_all(&mut self) {
        self.current = self.max;
    }
    pub fn cast_spell(&mut self, spell: SpellCast) -> Option<SpellCast> {
        let cost = spell.cost();
        if self.current >= cost {
            self.current = (self.current - cost).clamp(0, self.max);
            Some(spell)
        } else {
            None
        }
    }
}

impl fmt::Display for Mana {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let current = if self.current < 35 {
            self.current.italic().on_bright_green()
        } else {
            self.current.italic()
        };
        write!(f, "{}[{}/{}]", Self::MP, current, self.max)
    }
}
impl Default for Mana {
    fn default() -> Self {
        Self::new(PLAYER_MP)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Technical {
    pub(crate) current: i64,
    pub(crate) max: i64,
}

impl Technical {
    pub const TP: Painted<&'static str> = Painted::new("MP").bold().blue();
    pub fn new(max: i64) -> Self {
        Self { current: 0, max }
    }
    pub fn cast_melee(&mut self, melee: MeleeAttack) -> Option<MeleeAttack> {
        let cost = melee.cost();
        let gain = melee.gain();
        if self.current >= cost {
            self.current = (self.current - cost + gain).clamp(0, self.max);
            Some(melee)
        } else {
            None
        }
    }
    pub fn restore(&mut self, amount: i64) {
        self.current = (self.current + amount).clamp(0, self.max);
    }
    pub fn drain_all(&mut self) {
        self.current = 0;
    }
}

impl fmt::Display for Technical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = self.current.italic();
        if self.current >= Melee::Super.cost() {
            current = current.on_bright_blue();
        } else if self.current >= Melee::Power.cost() {
            current = current.on_blue();
        }
        write!(f, "{}[{}/{}]", Self::TP, current, self.max)
    }
}
impl Default for Technical {
    fn default() -> Self {
        Self::new(PLAYER_TP)
    }
}
