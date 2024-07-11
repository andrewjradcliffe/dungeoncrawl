pub mod combat;
pub mod encounter;
pub mod item;
pub mod monster;
pub mod player;

use crate::combat::*;
use crate::encounter::*;
use crate::player::*;

pub fn crawl() {
    let mut player = Player::new();
    let mut i = 0;

    while player.is_alive() {
        let mut enc = Encounter::new(&mut player);
        match enc.progress() {
            PlayerVictory => i += 1,
            MonsterVictory => break,
            _ => (),
        }
    }
    println!("You defeated {} monsters!", i);
}
