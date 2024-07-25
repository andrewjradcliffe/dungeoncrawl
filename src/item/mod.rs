pub mod consumable;
pub mod equipment;
pub mod equipment_bag;

use self::consumable::*;
use self::equipment::*;

use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Item {
    Consumable(Consumable),
    Gear(Gear),
}
impl Item {
    pub fn cost(&self) -> usize {
        match self {
            Self::Consumable(x) => x.cost(),
            Self::Gear(x) => x.cost(),
        }
    }
}
macro_rules! impl_from {
    { $T:ident } => {
        impl From<$T> for Item {
            fn from(value: $T) -> Self {
                Self::$T(value)
            }
        }
    }
}
impl_from! { Gear }
impl_from! { Consumable }

impl FromStr for Item {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Ok(consumable) = s.parse::<Consumable>() {
            Ok(Self::from(consumable))
        } else if let Ok(gear) = s.parse::<Gear>() {
            Ok(Self::from(gear))
        } else {
            Err(s.to_string())
        }
    }
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Consumable(x) => write!(f, "{}", x),
            Self::Gear(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    #[test]
    fn item_map() {
        let mut map: IndexMap<Item, usize> = IndexMap::new();
        map.insert(Item::Gear(Gear::Fist), 1);
        map.insert(Item::Gear(Gear::Sword), 1);
        map.insert(Item::Consumable(Consumable::HealthPotion), 2);
        map.insert(Item::Consumable(Consumable::ManaPotion), 1);
        assert_eq!(map.len(), 4);
    }
}
