use crate::combat::Combatant;

pub(crate) const MONSTER_HP: i64 = 20;

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    pub(crate) current_hp: i64,
    pub(crate) max_hp: i64,
}

impl Combatant for Monster {
    fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
    fn receive_damage(&mut self, amount: i64) {
        self.current_hp -= amount;
    }
}

impl Monster {
    pub fn new() -> Self {
        Self {
            current_hp: MONSTER_HP,
            max_hp: MONSTER_HP,
        }
    }
    pub fn status(&self) -> String {
        format!("HP[{}/{}]", self.current_hp, self.max_hp,)
    }
}
