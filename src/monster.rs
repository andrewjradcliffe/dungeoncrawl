use crate::{melee::*, resource::*, spell::*};
use rand::Rng;
use std::{convert::TryFrom, fmt, hash::Hash};
use yansi::{Paint, Painted};

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    pub(crate) kind: MonsterKind,
    pub(crate) strength: i64,
    pub(crate) hp: Health,
    pub(crate) tp: Technical,
    pub(crate) level: usize,
}

impl Monster {
    pub fn new(kind: MonsterKind, level: usize) -> Self {
        Self {
            kind,
            strength: kind.strength() * level as i64,
            hp: Health::new(kind.max_hp() * level as i64),
            tp: Technical::default(),
            level,
        }
    }
    pub fn strength(&self) -> i64 {
        self.strength
    }
    pub fn experience_points(&self) -> usize {
        self.hp.max as usize / 2
    }
    pub fn write_status<T: fmt::Write>(&self, buf: &mut T) -> fmt::Result {
        write!(buf, "{} {}", self.hp, self.tp)
    }
    pub fn status(&self) -> String {
        let mut buf = String::with_capacity(1 << 7);
        self.write_status(&mut buf).unwrap();
        buf
    }

    pub fn rand_level(kind: MonsterKind, level: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(kind, rng.gen_range(1usize..=level.min(10usize)))
    }
    pub fn rand(level: usize) -> Self {
        Self::rand_level(MonsterKind::rand(), level)
    }

    pub fn is_alive(&self) -> bool {
        self.hp.is_alive()
    }
    pub fn receive_damage(&mut self, amount: i64) {
        self.hp.receive_damage(amount)
    }
    pub fn receive_melee_attack(&mut self, melee: MeleeAttack) {
        let amount = melee.damage;
        println!(
            "Your {} attack hits the {} for {} damage!",
            melee.kind,
            self.kind,
            amount.magenta(),
        );
        self.receive_damage(amount);
    }
    pub fn receive_spell_attack(&mut self, spell: OffenseSpell) {
        let amount = spell.damage;
        let kind = spell.kind;
        println!(
            "Your {kind} hits the {} for {} damage!",
            self.kind,
            amount.magenta(),
        );
        self.receive_damage(amount);
    }
    pub(crate) fn cast_melee(&mut self, melee: Melee) -> MeleeAttack {
        let cost = melee.cost();
        let gain = melee.gain();
        self.tp.current = self.tp.current - cost + gain;
        MeleeAttack::new(melee, self.strength())
    }
    pub fn produce_melee_attack(&mut self) -> MeleeAttack {
        if self.tp.current >= Super.cost() {
            self.cast_melee(Super)
        } else if self.tp.current >= Power.cost() && self.hp.current <= self.hp.pct_max(20) {
            self.cast_melee(Power)
        } else {
            self.cast_melee(Basic)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MonsterKind {
    Frog,
    Bat,
    Snake,
    Wolf,
    Goblin,
    Bear,
    Undead,
    Orc,
    Vampire,
    Troll,
    Mammoth,
    Dragon,
    Fairy,
}
pub use MonsterKind::*;

impl MonsterKind {
    pub const COUNT: u8 = 13;
    pub const fn max_hp(&self) -> i64 {
        match self {
            Frog => 20,
            Bat => 25,
            Snake => 30,
            Wolf => 35,
            Goblin => 50,
            Bear => 75,
            Undead => 90,
            Orc => 100,
            Vampire => 125,
            Troll => 150,
            Mammoth => 300,
            Dragon => 500,
            Fairy => 1,
        }
    }

    pub const fn strength(&self) -> i64 {
        match self {
            Frog => 1,
            Bat => 1,
            Snake => 1,
            Wolf => 2,
            Goblin => 3,
            Bear => 3,
            Undead => 4,
            Orc => 5,
            Vampire => 6,
            Troll => 7,
            Mammoth => 7,
            Dragon => 15,
            Fairy => -20,
        }
    }
    pub(crate) const fn loot_weight(&self) -> usize {
        (self.max_hp() / 20) as usize
    }
    pub(crate) fn loot_prob(&self) -> f64 {
        (self.max_hp() as f64) / (Dragon.max_hp() as f64)
    }
    pub const fn experience_points(&self) -> usize {
        (self.max_hp() / 2) as usize
    }

    pub const fn radius(&self) -> usize {
        match self {
            Frog | Bat | Snake => 1,
            Wolf => 2,
            Goblin => 3,
            Bear => 4,
            Undead => 2,
            Orc => 5,
            Vampire => 5,
            Troll | Mammoth => 7,
            Dragon => 10,
            Fairy => 1,
        }
    }

    pub(crate) const fn from_index(i: u8) -> Self {
        const FROG: u8 = Frog as u8;
        const BAT: u8 = Bat as u8;
        const SNAKE: u8 = Snake as u8;
        const WOLF: u8 = Wolf as u8;
        const GOBLIN: u8 = Goblin as u8;
        const BEAR: u8 = Bear as u8;
        const UNDEAD: u8 = Undead as u8;
        const ORC: u8 = Orc as u8;
        const VAMPIRE: u8 = Vampire as u8;
        const TROLL: u8 = Troll as u8;
        const MAMMOTH: u8 = Mammoth as u8;
        const DRAGON: u8 = Dragon as u8;
        const FAIRY: u8 = Fairy as u8;

        match i {
            FROG => Frog,
            BAT => Bat,
            SNAKE => Snake,
            WOLF => Wolf,
            GOBLIN => Goblin,
            BEAR => Bear,
            UNDEAD => Undead,
            ORC => Orc,
            VAMPIRE => Vampire,
            TROLL => Troll,
            MAMMOTH => Mammoth,
            DRAGON => Dragon,
            FAIRY => Fairy,
            _ => unreachable!(),
        }
    }

    pub fn gen<T: Rng>(rng: &mut T) -> Self {
        Self::from_index(rng.gen_range(0u8..Self::COUNT))
    }
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self::gen(&mut rng)
    }

    pub const fn singular(&self) -> &'static str {
        match self {
            Frog => "frog",
            Wolf => "wolf",
            Bat => "bat",
            Snake => "snake",
            Goblin => "goblin",
            Bear => "bear",
            Undead => "undead",
            Orc => "orc",
            Vampire => "vampire",
            Troll => "troll",
            Mammoth => "mammoth",
            Dragon => "dragon",
            Fairy => "fairy",
        }
    }
    pub const fn plural(&self) -> &'static str {
        match self {
            Frog => "frogs",
            Wolf => "wolves",
            Bat => "bats",
            Snake => "snakes",
            Goblin => "goblins",
            Bear => "bears",
            Undead => "undead",
            Orc => "orcs",
            Vampire => "vampires",
            Troll => "trolls",
            Mammoth => "mammoths",
            Dragon => "dragons",
            Fairy => "fairies",
        }
    }
    pub const fn adjective(&self) -> &'static str {
        match self {
            Frog | Bat | Snake | Wolf | Bear | Mammoth => "wild",
            Goblin => "wandering",
            Undead => "vile",
            Orc => "ferocious",
            Vampire => "blood-sucking",
            Troll => "hulking",
            Dragon => "fire-breathing",
            Fairy => "harmless",
        }
    }
    pub const fn symbol(&self) -> char {
        match self {
            Frog => '🐸',
            Wolf => '🐺',
            Bat => '🦇',
            Snake => '🐍',
            Goblin => '👺',
            Bear => '🐻',
            Undead => '🧟',
            Orc => '👹',
            Vampire => '🧛',
            Troll => '🧌',
            Mammoth => '🦣',
            Dragon => '🐉',
            Fairy => '🧚',
        }
    }
    #[inline]
    const fn painted(s: &'static str) -> Painted<&'static str> {
        Painted::new(s).rgb(0xff, 0x1c, 0x00).bold()
    }
    pub const fn singular_painted(&self) -> Painted<&'static str> {
        Self::painted(self.singular())
    }
    pub const fn plural_painted(&self) -> Painted<&'static str> {
        Self::painted(self.plural())
    }
}

impl fmt::Display for MonsterKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.singular_painted())
    }
}

impl TryFrom<char> for MonsterKind {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '🐸' => Frog,
            '🐺' => Wolf,
            '🦇' => Bat,
            '👺' => Goblin,
            '🐻' => Bear,
            '🧟' => Undead,
            '👹' => Orc,
            '🧛' => Vampire,
            '🧌' => Troll,
            '🦣' => Mammoth,
            '🐉' => Dragon,
            '🧚' => Fairy,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_trip() {
        for kind in [
            Frog, Bat, Snake, Wolf, Goblin, Bear, Undead, Orc, Vampire, Troll, Mammoth, Dragon,
            Fairy,
        ] {
            assert_eq!(kind, MonsterKind::try_from(kind.symbol()).unwrap());
        }
    }
}
