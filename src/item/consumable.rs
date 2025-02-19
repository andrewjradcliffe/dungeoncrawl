use crate::resource::*;
use rand::Rng;
use regex::Regex;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;
use std::sync::LazyLock;
use yansi::Paint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Consumable {
    /// Restores 50 HP
    HealthPotion,
    /// Restores 50 MP
    ManaPotion,
    /// Restores 10 HP and 10 MP
    Food,
}
pub use Consumable::*;

impl Consumable {
    pub(crate) const fn total_variants() -> usize {
        3
    }
    pub const fn healing(&self) -> i64 {
        match self {
            HealthPotion => 50,
            ManaPotion => 0,
            Food => 10,
        }
    }
    pub const fn mana_restore(&self) -> i64 {
        match self {
            HealthPotion => 0,
            ManaPotion => 50,
            Food => 10,
        }
    }
    pub(crate) fn description_imp(&self) -> String {
        match self {
            HealthPotion => format!("restores {} {}", self.healing(), Health::HP),
            ManaPotion => format!("restores {} {}", self.mana_restore(), Mana::MP),
            Food => format!(
                "restores {} {} and {} {}",
                self.healing(),
                Health::HP,
                self.mana_restore(),
                Mana::MP,
            ),
        }
    }

    pub fn description(&self) -> &str {
        static HEALTH_POTION: LazyLock<String> = LazyLock::new(|| HealthPotion.description_imp());
        static MANA_POTION: LazyLock<String> = LazyLock::new(|| ManaPotion.description_imp());
        static FOOD: LazyLock<String> = LazyLock::new(|| Food.description_imp());

        match self {
            HealthPotion => &*HEALTH_POTION,
            ManaPotion => &*MANA_POTION,
            Food => &*FOOD,
        }
    }

    pub(crate) fn combat_description_imp(&self) -> String {
        match self {
            HealthPotion => format!("restores {} {}", self.healing().magenta(), Health::HP),
            ManaPotion => format!("restores {} {}", self.mana_restore().magenta(), Mana::MP),
            Food => format!(
                "restores {} {} and {} {}",
                self.healing().magenta(),
                Health::HP,
                self.mana_restore().magenta(),
                Mana::MP,
            ),
        }
    }

    pub fn combat_description(&self) -> &str {
        static HEALTH_POTION: LazyLock<String> =
            LazyLock::new(|| HealthPotion.combat_description_imp());
        static MANA_POTION: LazyLock<String> =
            LazyLock::new(|| ManaPotion.combat_description_imp());
        static FOOD: LazyLock<String> = LazyLock::new(|| Food.combat_description_imp());

        match self {
            HealthPotion => &*HEALTH_POTION,
            ManaPotion => &*MANA_POTION,
            Food => &*FOOD,
        }
    }
    pub const fn cost(&self) -> usize {
        match self {
            HealthPotion => 2,
            ManaPotion => 3,
            Food => 1,
        }
    }

    pub(crate) fn from_index(i: u8) -> Self {
        const HEALTHPOTION: u8 = HealthPotion as u8;
        const MANAPOTION: u8 = ManaPotion as u8;
        const FOOD: u8 = Food as u8;

        match i {
            HEALTHPOTION => HealthPotion,
            MANAPOTION => ManaPotion,
            FOOD => Food,
            _ => panic!(),
        }
    }
    pub fn gen<T: Rng>(rng: &mut T) -> Self {
        Self::from_index(rng.gen_range(0u8..3u8))
    }
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self::gen(&mut rng)
    }
}

impl FromStr for Consumable {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_HP: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(?:hp|health\s+potion)$").unwrap());
        static RE_MP: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(?:mp|mana\s+potion)$").unwrap());
        static RE_FOOD: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)^(?:food|f)$").unwrap());

        if RE_HP.is_match(s) {
            Ok(HealthPotion)
        } else if RE_MP.is_match(s) {
            Ok(ManaPotion)
        } else if RE_FOOD.is_match(s) {
            Ok(Food)
        } else {
            Err(s.to_string())
        }
    }
}

impl fmt::Display for Consumable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthPotion => write!(f, "{}", "Health potion".cyan()),
            ManaPotion => write!(f, "{}", "Mana potion".cyan()),
            Food => write!(f, "{}", "Food".cyan()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        macro_rules! test_eq {
            ($lhs:expr ; $($s:literal),+) => {
                $(
                    assert_eq!($lhs, $s.parse::<Consumable>().unwrap());
                )+
            }
        }
        test_eq!(HealthPotion ; "hp", "HP", "health potion", "Health potion", "health    potion");
        test_eq!(ManaPotion ; "mp", "MP", "mana potion", "Mana potion", "mana    potion");
        test_eq!(Food ; "f", "F", "foOd", "food");

        macro_rules! test_err {
            ($($s:literal),+) => {
                $(
                    assert!($s.parse::<Consumable>().is_err());
                )+
            }
        }
        test_err!("a", "c", "bu", "sel", "qui", "1234");
    }
}
