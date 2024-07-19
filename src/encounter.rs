use crate::combat::*;
use crate::loot::*;
use crate::melee::*;
use crate::monster::*;
use crate::player::*;
use crate::spell::*;
use ansi_term::Style;
use std::io::{self, BufRead, Write};

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
            Some(Self::new(player))
        } else {
            None
        }
    }
    pub fn new(player: &'a mut Player) -> Self {
        Self {
            player,
            monster: Monster::rand(),
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
        println!("---- A wild {kind} appeared! ----");
        let res = self.dialogue();
        match res {
            PlayerVictory => {
                println!("---- The {kind} died! ----");
                let loot = Loot::rand_weighted(self.monster.kind);
                loot.announce();
                self.player.acquire(loot);
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
                self.player.receive_melee_attack(&self.monster);
                if self.is_player_dead() {
                    return MonsterVictory;
                }
            };
        }
        loop {
            match self.menu() {
                Attack => {
                    if let Some(melee) = melee_menu() {
                        match self.player.cast_melee(melee) {
                            Some(melee) => self.monster.receive_melee_attack(melee),
                            None => {
                                println!("Insufficient TP!");
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
                    if let Some(spell) = spell_menu() {
                        match self.player.cast_spell(spell) {
                            Some(Fire | Stone) => self.monster.receive_spell_attack(spell),
                            Some(Cure1 | Cure2 | Meditate) => {
                                self.player.receive_defensive_spell(spell)
                            }
                            None => {
                                println!("Insufficient MP!");
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
                    self.player.visit_inventory();
                    damage_and_check!();
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
        self.status.clear();
        self.player.write_status(&mut self.status);
    }

    pub fn menu(&mut self) -> CombatAction {
        self.update_status();
        println!(
            "The {} in front of you has {}",
            self.monster.kind,
            self.monster.status()
        );
        println!(
            "{}TTACK, {}AST, {}UN, {}NVENTORY, or do {}OTHING?",
            Style::new().underline().paint("A"),
            Style::new().underline().paint("C"),
            Style::new().underline().paint("R"),
            Style::new().underline().paint("I"),
            Style::new().underline().paint("N"),
        );
        loop {
            self.buf.clear();
            print!("{} > ", self.status);
            io::stdout().flush().unwrap();

            let stdin = io::stdin();
            let mut handle = stdin.lock();
            match handle.read_line(&mut self.buf) {
                Ok(_) => (),
                Err(e) => println!("Error in combat menu readline: {:#?}", e),
            }
            if let Ok(action) = self.buf.parse::<CombatAction>() {
                break action;
            }
        }
    }
}
