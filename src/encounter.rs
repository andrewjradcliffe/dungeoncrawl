use crate::combat::*;
use crate::monster::*;
use crate::player::*;
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
    pub fn consume(&mut self, action: Action) -> EncounterOutcome {
        match action {
            Attack => {
                self.damage_monster();
                if self.is_monster_dead() {
                    return PlayerVictory;
                }
                self.damage_player();
                if self.is_player_dead() {
                    return MonsterVictory;
                }
                Indeterminate
            }
            ShowInventory => {
                self.player.inventory_action();
                Indeterminate
            }
            Run => PlayerRan,
        }
    }

    pub fn progress(&mut self) -> EncounterOutcome {
        println!("---- A wild monster appeared! ----");
        let mut buf = String::with_capacity(1 << 10);
        let mut res = Indeterminate;
        loop {
            match res {
                PlayerVictory => {
                    println!("You are victorious!");
                    break;
                }
                PlayerRan => {
                    println!("You ran away!");
                    break;
                }
                MonsterVictory => {
                    println!("You died!");
                    break;
                }
                Indeterminate => {
                    println!(
                        "There is a monster in front of you, with HP [{}/{}]",
                        self.monster.hp, MONSTER_HP
                    );
                    println!("ATTACK or RUN or INVENTORY?");
                    match get_response(&mut buf, self.player.status()) {
                        Ok(()) => match buf.parse::<Action>() {
                            Ok(action) => {
                                res = self.consume(action);
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
