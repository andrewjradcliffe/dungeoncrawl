use crate::melee::*;
use crate::resource::*;
use crate::spell::*;
use rand::Rng;
use std::fmt;
use std::hash::Hash;
use yansi::Paint;

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

    pub fn rand(level: usize) -> Self {
        let mut rng = rand::thread_rng();
        Monster::new(
            MonsterKind::rand(),
            rng.gen_range(1usize..=level.min(10usize)),
        )
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
        } else if self.tp.current >= Power.cost() && self.hp.current <= self.hp.pct_max(10) {
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
    Wolf,
    Goblin,
    Bear,
    Orc,
    Dragon,
    Fairy,
}
pub use MonsterKind::*;

impl MonsterKind {
    pub const fn max_hp(&self) -> i64 {
        match self {
            Frog => 20,
            Bat => 25,
            Wolf => 35,
            Goblin => 50,
            Bear => 75,
            Orc => 100,
            Dragon => 250,
            Fairy => 1,
        }
    }

    pub const fn strength(&self) -> i64 {
        match self {
            Frog => 1,
            Bat => 1,
            Wolf => 2,
            Goblin => 3,
            Bear => 4,
            Orc => 6,
            Dragon => 10,
            Fairy => -20,
        }
    }
    pub(crate) const fn loot_weight(&self) -> usize {
        (self.max_hp() / 20) as usize
    }
    pub const fn experience_points(&self) -> usize {
        (self.max_hp() / 2) as usize
    }

    pub(crate) const fn from_index(i: u8) -> Self {
        const FROG: u8 = Frog as u8;
        const BAT: u8 = Bat as u8;
        const WOLF: u8 = Wolf as u8;
        const GOBLIN: u8 = Goblin as u8;
        const BEAR: u8 = Bear as u8;
        const ORC: u8 = Orc as u8;
        const DRAGON: u8 = Dragon as u8;
        const FAIRY: u8 = Fairy as u8;

        match i {
            FROG => Frog,
            BAT => Bat,
            WOLF => Wolf,
            GOBLIN => Goblin,
            BEAR => Bear,
            ORC => Orc,
            DRAGON => Dragon,
            FAIRY => Fairy,
            _ => panic!(),
        }
    }
    pub fn gen<T: Rng>(rng: &mut T) -> Self {
        Self::from_index(rng.gen_range(0u8..8u8))
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
            Goblin => "goblin",
            Bear => "bear",
            Orc => "orc",
            Dragon => "dragon",
            Fairy => "fairy",
        }
    }
    pub const fn plural(&self) -> &'static str {
        match self {
            Frog => "frogs",
            Wolf => "wolves",
            Bat => "bats",
            Goblin => "goblins",
            Bear => "bears",
            Orc => "orcs",
            Dragon => "dragons",
            Fairy => "fairies",
        }
    }
}

impl fmt::Display for MonsterKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.singular().cyan())
    }
}
