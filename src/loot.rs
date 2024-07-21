use crate::item::*;
use crate::monster::MonsterKind;
use rand::Rng;

use MonsterKind::*;

pub struct Loot {
    pub(crate) item: Consumable,
    pub(crate) amount: usize,
}

impl Loot {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        let item = Consumable::gen(&mut rng);

        let amount = rng.gen_range(0..3);
        Self { item, amount }
    }
    pub fn announce(&self) {
        match self.amount {
            0 => (),
            1 => println!("You found a {}!", self.item),
            x => println!("You found {} {}s!", x, self.item),
        }
    }
    pub fn rand_weighted(kind: MonsterKind) -> Self {
        let mut rng = rand::thread_rng();
        let item = Consumable::gen(&mut rng);

        let amount = match kind {
            Fairy => 0,
            _ => rng.gen_range(0..kind.loot_weight()),
        };

        Self { item, amount }
    }
}
