use std::fmt::{self, Write};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Item {
    /// Restores 25 HP
    HealthPotion,
    /// Restores 25 MP
    ManaPotion,
    /// Restores 10 HP and 10 MP
    Food,
}
pub use Item::*;

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthPotion => write!(f, "Health potion"),
            ManaPotion => write!(f, "Mana potion"),
            Food => write!(f, "Food"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory(Vec<Item>);

impl Inventory {
    pub fn new() -> Self {
        Self(vec![HealthPotion, ManaPotion, Food])
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Inventory:")?;
        for item in self.0.iter() {
            writeln!(f, "    {}", item)?;
        }
        Ok(())
    }
}
