#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Weapon {
    Fist,
    Sword,
    Axe,
    Wand,
    Staff,
}
use Weapon::*;
impl Weapon {
    pub const fn strength(&self) -> i64 {
        match self {
            Sword => 1,
            Axe => 3,
            _ => 0,
        }
    }
    pub const fn intellect(&self) -> i64 {
        match self {
            Wand => 1,
            Staff => 3,
            _ => 0,
        }
    }
}
impl Default for Weapon {
    fn default() -> Self {
        Fist
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Head {
    Bare,
    Helmet,
    WizardHat,
}
impl Head {
    pub const fn strength(&self) -> i64 {
        match self {
            Head::Helmet => 2,
            _ => 0,
        }
    }
    pub const fn intellect(&self) -> i64 {
        match self {
            Head::WizardHat => 2,
            _ => 0,
        }
    }
}
impl Default for Head {
    fn default() -> Self {
        Head::Bare
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Chest {
    Bare,
    Breastplate,
    WizardRobe,
}
impl Chest {
    pub const fn strength(&self) -> i64 {
        match self {
            Chest::Breastplate => 3,
            _ => 0,
        }
    }
    pub const fn intellect(&self) -> i64 {
        match self {
            Chest::WizardRobe => 3,
            _ => 0,
        }
    }
}
impl Default for Chest {
    fn default() -> Self {
        Chest::Bare
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hand {
    Bare,
    Gauntlet,
    WizardGlove,
}

impl Hand {
    pub const fn strength(&self) -> i64 {
        match self {
            Hand::Gauntlet => 1,
            _ => 0,
        }
    }
    pub const fn intellect(&self) -> i64 {
        match self {
            Hand::WizardGlove => 1,
            _ => 0,
        }
    }
}
impl Default for Hand {
    fn default() -> Self {
        Hand::Bare
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Equipment {
    weapon: Weapon,
    head: Head,
    chest: Chest,
    hand: Hand,
}
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
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            weapon: Weapon::default(),
            head: Head::default(),
            chest: Chest::default(),
            hand: Hand::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let lhs = Equipment {
            weapon: Fist,
            head: Head::Bare,
            chest: Chest::Bare,
            hand: Hand::Bare,
        };
        assert_eq!(lhs, Equipment::default())
    }
}
