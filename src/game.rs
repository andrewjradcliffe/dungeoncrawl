use crate::{adventure::*, dungeon::Dungeon, maze::Maze, player::*, town::*, trade::Merchant};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    Town,
    Dungeon,
    Adventure,
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

    let mut state = State::Town;
    let mut player = Player::new();

    let mut merchant = Merchant::new();

    let mut maze = Maze::new_demo();

    loop {
        if player.is_alive() {
            match state {
                State::Town => match town_menu() {
                    TownAction::Adventure => {
                        state = State::Adventure;
                    }
                    TownAction::Dungeon => {
                        state = State::Dungeon;
                    }
                    TownAction::Sleep => {
                        player.sleep();
                    }
                    TownAction::Trade => merchant.trade(&mut player),
                    TownAction::Inventory => player.noncombat_inventory(),
                    TownAction::Equipment => player.noncombat_equipment(),
                    TownAction::Stats => println!("{}", player.attribute_message()),
                },
                State::Dungeon => {
                    let n_monster: usize = rng.gen_range(1..5);
                    state = State::Town;
                    let mut dungeon = Dungeon::new(&mut player, n_monster);
                    dungeon.run();
                }
                State::Adventure => {
                    // let mut adv = Adventure::new(&mut player);
                    let mut adv = Adventure::new(&mut player, &mut maze);
                    adv.run();

                    state = State::Town;
                }
            }
        } else {
            state = State::Town;
            println!("Another adventurer found your body and carried it to the town.");
            println!("You are now being revived...");
            player.revive();
        }
    }
}
