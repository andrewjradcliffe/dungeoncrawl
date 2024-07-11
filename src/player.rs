use crate::combat::Combatant;
use crate::item::*;

pub(crate) const PLAYER_HP: i64 = 100;
pub(crate) const PLAYER_MP: i64 = 100;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub(crate) current_hp: i64,
    pub(crate) max_hp: i64,
    pub(crate) current_mp: i64,
    pub(crate) max_mp: i64,
    pub(crate) inventory: Vec<Item>,
}

impl Combatant for Player {
    fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
    fn receive_damage(&mut self, amount: i64) {
        self.current_hp -= amount;
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            current_hp: PLAYER_HP,
            max_hp: PLAYER_HP,
            current_mp: PLAYER_MP,
            max_mp: PLAYER_MP,
            inventory: vec![],
        }
    }
    fn restore_hp(&mut self, amount: i64) {
        self.current_hp = (self.current_hp + amount).clamp(0, self.max_hp);
    }
    fn restore_mp(&mut self, amount: i64) {
        self.current_mp = (self.current_mp + amount).clamp(0, self.max_mp);
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
    pub fn status(&self) -> String {
        format!(
            "HP[{}/{}] MP[{}/{}]",
            self.current_hp, self.max_hp, self.current_mp, self.max_mp
        )
    }
}
