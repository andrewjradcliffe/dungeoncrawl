use crate::adventure::*;
use crate::encounter::*;
use crate::player::*;
use crate::scoreboard::*;
use crate::town::*;
use crate::trade::Merchant;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Town,
    Dungeon,
    Adventure,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    state: State,
    player: Player,
}
impl Game {
    pub fn new() -> Self {
        Self {
            state: State::Town,
            player: Player::new(),
        }
    }
}

pub fn game() {
    // crate::readline::read_line();
    // crate::readline::read_direction();
    // crate::readline::progress_bar();

    println!(
        "\n\n\n================================================================================"
    );
    println!("Welcome to dungeon crawler!");
    println!(
        "================================================================================\n\n\n"
    );
    // crate::maze::demo_movement();
    let mut rng = rand::thread_rng();

    let mut game = Game::new();

    let mut merchant = Merchant::new();

    loop {
        if game.player.is_alive() {
            match game.state {
                State::Town => match town_menu() {
                    TownAction::Adventure => {
                        game.state = State::Adventure;
                    }
                    TownAction::Dungeon => {
                        game.state = State::Dungeon;
                    }
                    TownAction::Sleep => {
                        game.player.sleep();
                    }
                    TownAction::Trade => merchant.trade(&mut game.player),
                    TownAction::Inventory => game.player.noncombat_inventory(),
                    TownAction::Equipment => game.player.noncombat_equipment(),
                    TownAction::Stats => println!("{}", game.player.attribute_message()),
                },
                State::Dungeon => {
                    let n_monster: usize = rng.gen_range(1..5);
                    game.state = State::Town;
                    gauntlet(&mut game, n_monster);
                }
                State::Adventure => {
                    let mut adv = Adventure::new(&mut game.player);
                    adv.run();
                    game.state = State::Town;
                }
            }
        } else {
            game.state = State::Town;
            println!("Another adventurer found your body and carried it to the town.");
            println!("You are now being revived...");
            game.player.revive();
        }
    }
}

pub fn gauntlet(game: &mut Game, n: usize) {
    println!(
        "\n\n\n================================================================================"
    );
    println!("Let the gauntlet commence!");
    println!(
        "================================================================================\n\n\n"
    );
    let mut scoreboard = Scoreboard::new();

    let mut i = 0;
    while game.player.is_alive() && i < n {
        let mut enc = Encounter::rand(&mut game.player);
        let kind = enc.monster.kind.clone();
        match enc.run() {
            PlayerVictory => {
                i += 1;
                scoreboard.record(kind);
            }
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
