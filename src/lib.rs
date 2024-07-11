use std::io::{self, BufRead, Write};
use std::str::FromStr;

pub enum EncounterOutcome {
    PlayerVictory,
    MonsterVictory,
    PlayerRan,
    Indeterminate,
}
use EncounterOutcome::*;

pub enum Action {
    Attack,
    Run,
}
use Action::*;

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("a") || s.eq_ignore_ascii_case("attack") {
            Ok(Action::Attack)
        } else if s.eq_ignore_ascii_case("r") || s.eq_ignore_ascii_case("run") {
            Ok(Action::Run)
        } else {
            Err(s.to_string())
        }
    }
}

const PLAYER_HP: i64 = 100;
const MONSTER_HP: i64 = 20;

pub struct Player {
    hp: i64,
}

impl Player {
    pub fn new() -> Self {
        Self { hp: PLAYER_HP }
    }
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
    pub fn receive_damage(&mut self, amount: i64) {
        self.hp -= amount;
    }
}

pub struct Monster {
    hp: i64,
}

impl Monster {
    pub fn new() -> Self {
        Self { hp: 100 }
    }
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
    pub fn receive_damage(&mut self, amount: i64) {
        self.hp -= amount;
    }
}

pub struct GameState {
    player: Player,
    monster: Monster,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
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
            Run => PlayerRan,
        }
    }
}

pub fn get_response(buf: &mut String, hp: i64) -> io::Result<()> {
    buf.clear();
    print!("[{}/{}] > ", hp, PLAYER_HP);
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(buf)?;
    Ok(())
}
pub fn encounter(gs: &mut GameState) {
    let mut buf = String::with_capacity(1 << 10);
    let mut res = Indeterminate;
    loop {
        match res {
            PlayerVictory => {
                println!("You are victorious!");
                break;
            }
            PlayerRan => {
                println!("You are away!");
                break;
            }
            MonsterVictory => {
                println!("You lose!");
                break;
            }
            Indeterminate => {
                println!(
                    "There is a monster in front of you, with HP [{}/{}]",
                    gs.monster.hp, MONSTER_HP
                );
                println!("ATTACK or RUN?");
                match get_response(&mut buf, gs.player.hp) {
                    Ok(()) => match buf.parse::<Action>() {
                        Ok(action) => {
                            res = gs.consume(action);
                        }
                        Err(s) => println!("not a valid response: {}", s),
                    },
                    Err(e) => println!("Unable to read line: {:#?}", e),
                }
            }
        }
    }
}
