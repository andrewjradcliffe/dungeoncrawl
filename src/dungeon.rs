use crate::{encounter::*, player::*, scoreboard::Scoreboard};

#[derive(Debug, PartialEq)]
pub struct Dungeon<'a> {
    pub(crate) player: &'a mut Player,
    pub(crate) scoreboard: Scoreboard,
    pub(crate) n_monster: usize,
}

static DELIM: &'static str =
    "================================================================================";

impl<'a> Dungeon<'a> {
    pub fn new(player: &'a mut Player, n: usize) -> Self {
        Self {
            scoreboard: Scoreboard::new(),
            player,
            n_monster: n,
        }
    }
    pub fn run(&mut self) {
        println!("\n\n\n{DELIM}");
        println!("Let the gauntlet commence!");
        println!("{DELIM}\n\n\n");
        let n = self.n_monster;
        for _ in 0..n {
            let mut enc = Encounter::rand(&mut self.player);
            match enc.run() {
                PlayerVictory => self.scoreboard.record(enc.monster.kind),
                MonsterVictory => break,
                _ => (),
            }
        }
        println!("\n\n\n{}", DELIM);
        print!("{}", self.scoreboard);
        println!("{DELIM}\n\n\n");
    }
}
