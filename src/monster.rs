use crate::combat::Combatant;

pub(crate) const MONSTER_HP: i64 = 20;

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    pub(crate) hp: i64,
}

impl Combatant for Monster {
    fn is_alive(&self) -> bool {
        self.hp > 0
    }
    fn receive_damage(&mut self, amount: i64) {
        self.hp -= amount;
    }
}

impl Monster {
    pub fn new() -> Self {
        Self { hp: MONSTER_HP }
    }
}
