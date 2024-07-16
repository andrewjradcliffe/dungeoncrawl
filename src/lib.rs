pub mod combat;
pub mod encounter;
pub mod item;
pub mod loot;
pub mod melee;
pub mod monster;
pub mod player;
pub mod scoreboard;
pub mod spell;
pub mod town;

use crate::combat::*;
use crate::encounter::*;
use crate::player::*;
use crate::scoreboard::*;

pub fn crawl() {
    println!(
        "\n\n\n================================================================================"
    );
    println!("Welcome to dungeon crawler!");
    println!(
        "================================================================================\n\n\n"
    );
    let mut player = Player::new();

    let mut scoreboard = Scoreboard::new();

    while player.is_alive() {
        let mut enc = Encounter::new(&mut player);
        let kind = enc.monster.kind.clone();
        match enc.run() {
            PlayerVictory => scoreboard.record(kind),
            MonsterVictory => break,
            _ => (),
        }
    }
    println!("================================================================================");
    print!("{}", scoreboard);
    println!(
        "================================================================================\n\n\n"
    );
}
