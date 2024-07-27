use crate::{consumable::*, equipment::*, monster::MonsterKind};
use rand::Rng;

use MonsterKind::*;

pub struct Loot {
    pub(crate) item: Consumable,
    pub(crate) amount: usize,
    pub(crate) gear: Option<Gear>,
}

impl Loot {
    // pub fn rand() -> Self {
    //     let mut rng = rand::thread_rng();
    //     let item = Consumable::gen(&mut rng);
    //     let amount = rng.gen_range(0..3);
    //     Self { item, amount }
    // }
    pub fn announce(&self) {
        match self.amount {
            0 => (),
            1 => println!("You found a {}!", self.item),
            x => println!("You found {} {}s!", x, self.item),
        }
        if let Some(ref x) = self.gear {
            println!("You found a {x}!")
        }
    }
    pub(crate) fn gen_imp<T: Rng>(rng: &mut T, kind: MonsterKind) -> Self {
        let item = Consumable::gen(rng);
        let (amount, gear) = match kind {
            Fairy => (0, None),
            _ => {
                let amount = rng.gen_range(0..kind.loot_weight());
                let gear = if rng.gen_bool(kind.loot_prob()) {
                    Some(Gear::from_index_trunc(rng.gen_range(0u8..10u8)))
                } else {
                    None
                };
                (amount, gear)
            }
        };

        Self { item, amount, gear }
    }
    pub fn gen<T: Rng>(rng: &mut T) -> Self {
        let kind = MonsterKind::gen(rng);
        Self::gen_imp(rng, kind)
    }
    pub fn rand_weighted(kind: MonsterKind) -> Self {
        Self::gen_imp(&mut rand::thread_rng(), kind)
    }

    pub fn rand() -> Self {
        Self::gen(&mut rand::thread_rng())
    }
}
