use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;
use yansi::{Paint, Painted};

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Weapon {
//     Fist,
//     Sword,
//     Axe,
//     Wand,
//     Staff,
// }
// use Weapon::*;
// impl Weapon {
//     pub const fn strength(&self) -> i64 {
//         match self {
//             Sword => 1,
//             Axe => 3,
//             _ => 0,
//         }
//     }
//     pub const fn intellect(&self) -> i64 {
//         match self {
//             Wand => 1,
//             Staff => 3,
//             _ => 0,
//         }
//     }
// }
// impl Default for Weapon {
//     fn default() -> Self {
//         Fist
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Head {
//     Bare,
//     Helmet,
//     WizardHat,
// }
// impl Head {
//     pub const fn strength(&self) -> i64 {
//         match self {
//             Head::Helmet => 2,
//             _ => 0,
//         }
//     }
//     pub const fn intellect(&self) -> i64 {
//         match self {
//             Head::WizardHat => 2,
//             _ => 0,
//         }
//     }
// }
// impl Default for Head {
//     fn default() -> Self {
//         Head::Bare
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Chest {
//     Bare,
//     Breastplate,
//     WizardRobe,
// }
// impl Chest {
//     pub const fn strength(&self) -> i64 {
//         match self {
//             Chest::Breastplate => 3,
//             _ => 0,
//         }
//     }
//     pub const fn intellect(&self) -> i64 {
//         match self {
//             Chest::WizardRobe => 3,
//             _ => 0,
//         }
//     }
// }
// impl Default for Chest {
//     fn default() -> Self {
//         Chest::Bare
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Hand {
//     Bare,
//     Gauntlet,
//     WizardGlove,
// }

// impl Hand {
//     pub const fn strength(&self) -> i64 {
//         match self {
//             Hand::Gauntlet => 1,
//             _ => 0,
//         }
//     }
//     pub const fn intellect(&self) -> i64 {
//         match self {
//             Hand::WizardGlove => 1,
//             _ => 0,
//         }
//     }
// }
// impl Default for Hand {
//     fn default() -> Self {
//         Hand::Bare
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Equipment {
//     weapon: Weapon,
//     head: Head,
//     chest: Chest,
//     hand: Hand,
// }
#[derive(Debug, Clone, PartialEq)]
pub struct Equipment {
    weapon: Gear,
    head: Gear,
    chest: Gear,
    hand: Gear,
}
pub use Gear::*;
impl Equipment {
    pub const fn strength(&self) -> i64 {
        self.weapon.strength() + self.head.strength() + self.chest.strength() + self.hand.strength()
    }
    pub const fn intellect(&self) -> i64 {
        self.weapon.intellect()
            + self.head.intellect()
            + self.chest.intellect()
            + self.hand.intellect()
    }
    pub const fn armor(&self) -> i64 {
        self.weapon.armor() + self.head.armor() + self.chest.armor() + self.hand.armor()
    }
    pub fn equip(&mut self, item: Gear) -> Gear {
        match item {
            Fist | Sword | Axe | Wand | Staff => {
                let old = self.weapon;
                self.weapon = item;
                old
            }
            Helmet | Hat => {
                let old = self.head;
                self.head = item;
                old
            }
            Breastplate | Robe => {
                let old = self.chest;
                self.chest = item;
                old
            }
            Gauntlet | Glove => {
                let old = self.hand;
                self.hand = item;
                old
            }
            Bare => Bare,
        }
    }
    pub fn unequip(&mut self, item: Gear) -> Gear {
        match item {
            Fist | Sword | Axe | Wand | Staff => {
                let old = self.weapon;
                self.weapon = Fist;
                old
            }
            Helmet | Hat => {
                let old = self.head;
                self.head = Bare;
                old
            }
            Breastplate | Robe => {
                let old = self.chest;
                self.chest = Bare;
                old
            }
            Gauntlet | Glove => {
                let old = self.hand;
                self.hand = Bare;
                old
            }
            Bare => Bare,
        }
    }
}

impl Default for Equipment {
    // fn default() -> Self {
    //     Self {
    //         weapon: Weapon::default(),
    //         head: Head::default(),
    //         chest: Chest::default(),
    //         hand: Hand::default(),
    //     }
    // }
    fn default() -> Self {
        Self {
            weapon: Gear::Fist,
            head: Gear::Bare,
            chest: Gear::Bare,
            hand: Gear::Bare,
        }
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", "Equipment".underline())?;
        writeln!(
            f,
            "                              |  {}  | {}",
            "value".underline(),
            "effect".underline(),
        )?;
        const GOLD: Painted<&'static str> = Painted::new("gold").bold().yellow();
        macro_rules! writeln_item {
            ($item:expr, $slot:literal) => {
                writeln!(
                    f,
                    "{:<10} {:<40} | {:>2} {} | {:<30}",
                    $slot,
                    format!("{}", $item),
                    $item.cost(),
                    GOLD,
                    $item.description(),
                )?;
            };
        }
        writeln_item!(self.weapon, "Weapon");
        writeln_item!(self.head, "Head");
        writeln_item!(self.chest, "Chest");
        writeln_item!(self.hand, "Hand");
        Ok(())
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Gear {
//     Weapon(Weapon),
//     Head(Head),
//     Chest(Chest),
//     Hand(Hand),
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gear {
    // Weapon
    Fist,
    Sword,
    Axe,
    Wand,
    Staff,
    // Head
    Helmet,
    Hat,
    // Chest
    Breastplate,
    Robe,
    // Hand
    Gauntlet,
    Glove,
    // default state
    Bare,
}
impl Gear {
    pub(crate) const fn total_variants() -> usize {
        12
    }
    pub const fn strength(&self) -> i64 {
        match self {
            Self::Sword => 2,
            Self::Axe => 4,
            Self::Helmet => 3,
            Self::Breastplate => 3,
            Self::Gauntlet => 2,
            _ => 0,
        }
    }
    pub const fn intellect(&self) -> i64 {
        match self {
            Self::Wand => 2,
            Self::Staff => 4,
            Self::Hat => 3,
            Self::Robe => 3,
            Self::Glove => 2,
            _ => 0,
        }
    }
    pub const fn armor(&self) -> i64 {
        match self {
            Self::Helmet => 3,
            Self::Breastplate => 3,
            Self::Gauntlet => 2,
            Self::Hat => 1,
            Self::Robe => 1,
            Self::Glove => 1,
            _ => 0,
        }
    }
    pub const fn cost(&self) -> usize {
        let lhs = self.strength() as usize;
        let rhs = self.intellect() as usize;
        10 * if lhs > rhs { lhs } else { rhs }
    }
    pub(crate) fn description(&self) -> String {
        format!(
            "STR +{:<2}  INT +{:<2}  ARMOR +{:<2}",
            self.strength(),
            self.intellect(),
            self.armor()
        )
    }
    pub(crate) fn from_index_trunc(idx: u8) -> Self {
        match idx {
            0 => Self::Sword,
            1 => Self::Axe,
            2 => Self::Wand,
            3 => Self::Staff,
            4 => Self::Helmet,
            5 => Self::Hat,
            6 => Self::Breastplate,
            7 => Self::Robe,
            8 => Self::Gauntlet,
            _ => Self::Glove,
        }
    }
}
impl FromStr for Gear {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        macro_rules! static_regex {
            { $var:ident, $regex:literal } => {
                static $var: Lazy<Regex> = Lazy::new(|| Regex::new($regex).unwrap());
            }
        }
        static_regex! { RE_FIST, "(?i)^fist$" }
        static_regex! { RE_SWORD, "(?i)^sword$" }
        static_regex! { RE_AXE, "(?i)^axe$" }
        static_regex! { RE_WAND, "(?i)^wand$" }
        static_regex! { RE_STAFF, "(?i)^staff$" }
        static_regex! { RE_HELMET, "(?i)^helmet$" }
        static_regex! { RE_HAT, "(?i)^hat$" }
        static_regex! { RE_BREASTPLATE, "(?i)^breastplate$" }
        static_regex! { RE_ROBE, "(?i)^robe$" }
        static_regex! { RE_GAUNTLET, "(?i)^gauntlet$" }
        static_regex! { RE_GLOVE, "(?i)^glove$" }

        macro_rules! branches {
            { ($var:ident, $variant:ident) ; $(($var_rest:ident, $variant_rest:ident),)+ } => {
                if $var.is_match(s) {
                    Ok(Gear::$variant)
                } $(else if $var_rest.is_match(s) { Ok(Gear::$variant_rest)})+
                else {
                    Err(s.to_string())
                }
            }
        }
        branches! { (RE_FIST, Fist) ; (RE_SWORD, Sword), (RE_AXE, Axe), (RE_WAND, Wand),
                     (RE_STAFF, Staff), (RE_HELMET, Helmet), (RE_HAT, Hat),
                     (RE_BREASTPLATE, Breastplate), (RE_ROBE, Robe),
                     (RE_GAUNTLET, Gauntlet), (RE_GLOVE, Glove),
        }
    }
}
impl fmt::Display for Gear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! arm {
            ($word:literal) => {
                write!(f, "{}", $word.rgb(0x8a, 0x2b, 0xe2))
            };
        }
        match self {
            Self::Fist => arm!("fist"),
            Self::Axe => arm!("axe"),
            Self::Sword => arm!("sword"),
            Self::Wand => arm!("wand"),
            Self::Staff => arm!("staff"),
            Self::Helmet => arm!("helmet"),
            Self::Hat => arm!("hat"),
            Self::Breastplate => arm!("breastplate"),
            Self::Robe => arm!("robe"),
            Self::Gauntlet => arm!("gauntlet"),
            Self::Glove => arm!("glove"),
            Self::Bare => arm!("bare"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let lhs = Equipment {
            weapon: Gear::Fist,
            head: Gear::Bare,
            chest: Gear::Bare,
            hand: Gear::Bare,
        };
        assert_eq!(lhs, Equipment::default())
    }
}
