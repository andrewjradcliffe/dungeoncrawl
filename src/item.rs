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
