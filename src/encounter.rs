use crate::{
    combat::*, loot::*, melee::*, monster::*, player::*, resource::Mana, resource::Technical,
    spell::*,
};
use std::io::{self, BufRead, Write};
use yansi::Paint;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum EncounterOutcome {
    PlayerVictory,
    MonsterVictory,
    PlayerRan,
}
pub use EncounterOutcome::*;

#[derive(Debug, PartialEq)]
pub struct Encounter<'a> {
    pub(crate) player: &'a mut Player,
    pub(crate) monster: Monster,
    buf: String,
    status: String,
}

impl<'a> Encounter<'a> {
    pub fn try_new(player: &'a mut Player) -> Option<Self> {
        if player.is_alive() {
            Some(Self::rand(player))
        } else {
            None
        }
    }
    pub fn rand(player: &'a mut Player) -> Self {
        let level = player.level();
        Self {
            player,
            monster: Monster::rand(level),
            buf: String::with_capacity(1 << 7),
            status: String::with_capacity(1 << 7),
        }
    }
    pub fn new(kind: MonsterKind, player: &'a mut Player) -> Self {
        let level = player.level();
        Self {
            player,
            monster: Monster::rand_level(kind, level),
            buf: String::with_capacity(1 << 7),
            status: String::with_capacity(1 << 7),
        }
    }
    pub fn is_monster_dead(&self) -> bool {
        !self.monster.is_alive()
    }
    pub fn is_player_dead(&self) -> bool {
        !self.player.is_alive()
    }

    pub fn run(&mut self) -> EncounterOutcome {
        let kind = self.monster.kind.clone();
        println!(
            "---- A wild {kind} (level {}) appeared! ----",
            self.monster.level
        );
        let res = self.dialogue();
        match res {
            PlayerVictory => {
                println!("---- The {kind} died! ----");
                let loot = Loot::rand_weighted(self.monster.kind);
                loot.announce();
                self.player.acquire(loot);
                let xp = self.monster.experience_points();
                println!("You earned {} experience points!", xp.bold());
                self.player.xp += xp;
                self.player.update_level();
            }
            PlayerRan => {
                println!("---- You ran away! ----");
            }
            MonsterVictory => {
                println!("---- You died! ----");
            }
        }
        res
    }

    pub(crate) fn dialogue(&mut self) -> EncounterOutcome {
        macro_rules! damage_and_check {
            () => {
                self.player.receive_melee_attack(&mut self.monster);
                if self.is_player_dead() {
                    return MonsterVictory;
                }
            };
        }
        loop {
            match self.menu() {
                Attack => {
                    if let Some(melee) = melee_menu(self.player.strength()) {
                        match self.player.cast_melee(melee) {
                            Some(melee) => self.monster.receive_melee_attack(melee),
                            None => {
                                println!("Insufficient {}!", Technical::TP);
                                continue;
                            }
                        }
                        if self.is_monster_dead() {
                            return PlayerVictory;
                        }
                        damage_and_check!();
                    }
                }
                Cast => {
                    if let Some(spell) = spell_menu(self.player.intellect()) {
                        match self.player.cast_spell(spell) {
                            Some(SpellCast::Offense(x)) => self.monster.receive_spell_attack(x),
                            Some(SpellCast::Defense(x)) => self.player.receive_defensive_spell(x),
                            None => {
                                println!("Insufficient {}!", Mana::MP);
                                continue;
                            }
                        }
                        if self.is_monster_dead() {
                            return PlayerVictory;
                        }
                        damage_and_check!();
                    }
                }
                ShowInventory => {
                    if self.player.visit_inventory() {
                        damage_and_check!();
                    }
                }
                Run => {
                    damage_and_check!();
                    return PlayerRan;
                }
                DoNothing => {
                    damage_and_check!();
                }
            }
        }
    }

    pub fn update_status(&mut self) {
        String::clear(&mut self.status);
        self.player.write_status(&mut self.status).unwrap();
    }

    pub fn menu(&mut self) -> CombatAction {
        self.update_status();
        println!(
            "The {} in front of you has {}",
            self.monster.kind,
            self.monster.status()
        );
        println!(
            "{}, {}, {}, {}, or do {}?",
            Attack, Cast, ShowInventory, Run, DoNothing
        );
        loop {
            String::clear(&mut self.buf);
            print!("{} > ", self.status);
            io::stdout().flush().unwrap();

            let stdin = io::stdin();
            let mut handle = stdin.lock();
            match handle.read_line(&mut self.buf) {
                Ok(_) => {
                    let _ = crate::readline::clear_last_n_lines(1);
                }
                Err(e) => println!("Error in combat menu readline: {:#?}", e),
            }
            if let Ok(action) = self.buf.parse::<CombatAction>() {
                // let _ = crate::readline::clear_last_n_lines(2);
                let _ = crate::readline::clear_last_n_lines(1);
                break action;
            }
        }
    }
}
