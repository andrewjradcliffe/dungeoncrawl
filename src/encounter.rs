use crate::combat::*;
use crate::melee::*;
use crate::monster::*;
use crate::player::*;
use crate::spell::*;
use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum EncounterOutcome {
    PlayerVictory,
    MonsterVictory,
    PlayerRan,
    Indeterminate,
}
pub use EncounterOutcome::*;

#[derive(Debug, PartialEq)]
pub struct Encounter<'a> {
    player: &'a mut Player,
    monster: Monster,
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
            monster: Monster::new(),
        }
    }
    pub fn is_monster_dead(&self) -> bool {
        !self.monster.is_alive()
    }
    pub fn is_player_dead(&self) -> bool {
        !self.player.is_alive()
    }
    pub fn damage_monster(&mut self) {
        self.monster.receive_damage(7)
    }
    pub fn damage_player(&mut self) {
        self.player.receive_damage(5)
    }
    pub fn perform(&mut self, action: CombatAction) -> EncounterOutcome {
        match action {
            Attack => {
                if let Some(melee) = melee_menu() {
                    match self.player.cast_melee(melee) {
                        Some(melee) => self.monster.receive_damage(melee.damage()),
                        None => {
                            println!("Insufficient TP!");
                            return Indeterminate;
                        }
                    }
                    if self.is_monster_dead() {
                        return PlayerVictory;
                    }
                    self.damage_player();
                    if self.is_player_dead() {
                        return MonsterVictory;
                    }
                    Indeterminate
                } else {
                    Indeterminate
                }
                // self.damage_monster();
                // if self.is_monster_dead() {
                //     return PlayerVictory;
                // }
                // self.damage_player();
                // if self.is_player_dead() {
                //     return MonsterVictory;
                // }
                // Indeterminate
            }
            ShowInventory => {
                self.player.inventory_action();
                Indeterminate
            }
            Run => PlayerRan,
            Cast => {
                if let Some(spell) = spell_menu() {
                    match self.player.cast_spell(spell) {
                        Some(Heal) => self.player.restore_hp(Heal.healing()),
                        Some(Fire) => self.monster.receive_damage(Fire.damage()),
                        Some(Stone) => self.monster.receive_damage(Stone.damage()),
                        None => {
                            println!("Insufficient MP!");
                            return Indeterminate;
                        }
                    }
                    if self.is_monster_dead() {
                        return PlayerVictory;
                    }
                    self.damage_player();
                    if self.is_player_dead() {
                        return MonsterVictory;
                    }
                    Indeterminate
                } else {
                    Indeterminate
                }
            }
        }
    }

    pub fn progress(&mut self) -> EncounterOutcome {
        println!("---- A wild monster appeared! ----");
        let mut buf = String::with_capacity(1 << 10);
        let mut res = Indeterminate;
        loop {
            match res {
                PlayerVictory => {
                    println!("---- The monster died! ----");
                    break;
                }
                PlayerRan => {
                    println!("---- You ran away! ----");
                    break;
                }
                MonsterVictory => {
                    println!("---- You died! ----");
                    break;
                }
                Indeterminate => {
                    println!(
                        // "There is a monster in front of you, with HP [{}/{}]",
                        "The monster in front of you has {}",
                        self.monster.status()
                    );
                    println!("ATTACK, CAST, RUN, or INVENTORY?");
                    match get_response(&mut buf, self.player.status()) {
                        Ok(()) => match buf.parse::<CombatAction>() {
                            Ok(action) => {
                                res = self.perform(action);
                            }
                            Err(s) => println!("not a valid response: {}", s),
                        },
                        Err(e) => println!("Unable to read line: {:#?}", e),
                    }
                }
            }
        }
        res
    }
}

pub fn get_response(buf: &mut String, status: String) -> io::Result<()> {
    buf.clear();
    print!("{} > ", status);
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(buf)?;
    Ok(())
}
