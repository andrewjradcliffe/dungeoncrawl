use crate::item::*;
use rand::Rng;

pub struct Loot {
    pub(crate) item: Item,
    pub(crate) amount: usize,
}

impl Loot {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        let item = Item::gen(&mut rng);

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
}
