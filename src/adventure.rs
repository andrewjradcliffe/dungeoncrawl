use crate::encounter::*;
use crate::maze::*;
use crate::player::Player;
use crate::spell::*;
use regex::Regex;
use std::sync::LazyLock;
// use regex::{RegexSet, RegexSetBuilder};
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum AdventureAction {
    Movement,
    Town,
    Inventory,
    Cast,
    Equipment,
    Stats,
}
use AdventureAction::*;

impl AdventureAction {
    pub fn description(&self) -> &'static str {
        match self {
            Movement => "Move freely about the world",
            Town => "Return to town",
            Inventory => "Open inventory",
            Cast => "Cast a spell",
            Equipment => "Open equipment",
            Stats => "Display character statistics",
        }
    }
    pub(crate) fn print_menu_item(&self) {
        println!(
            "    {:<30} | {:<30}",
            format!("{}", self),
            self.description(),
        );
    }
}

impl fmt::Display for AdventureAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Movement => write!(f, "Movement"),
            Town => write!(f, "Town"),
            Inventory => write!(f, "Inventory"),
            Cast => write!(f, "Cast"),
            Equipment => write!(f, "Equipment"),
            Stats => write!(f, "Stats"),
        }
    }
}

impl FromStr for AdventureAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_MOVE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("(?i)^(?:movement|m)$").unwrap());
        static RE_TOWN: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)^(?:town|t)$").unwrap());
        static RE_INV: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("(?i)^(?:inventory|i)$").unwrap());
        static RE_CAST: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)^(?:cast|c)$").unwrap());
        static RE_EQUIP: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("(?i)^(?:equipment|e)$").unwrap());
        static RE_STATS: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("(?i)^(?:stats?)$").unwrap());

        if RE_MOVE.is_match(s) {
            Ok(Movement)
        } else if RE_TOWN.is_match(s) {
            Ok(Town)
        } else if RE_INV.is_match(s) {
            Ok(Inventory)
        } else if RE_CAST.is_match(s) {
            Ok(Cast)
        } else if RE_EQUIP.is_match(s) {
            Ok(Equipment)
        } else if RE_STATS.is_match(s) {
            Ok(Stats)
        } else {
            Err(s.to_string())
        }
    }
}

pub fn adventure_menu() -> AdventureAction {
    let mut buf = String::with_capacity(1 << 10);
    println!("==== Entering the adventure... ====");
    Movement.print_menu_item();
    Town.print_menu_item();
    Inventory.print_menu_item();
    Cast.print_menu_item();
    Equipment.print_menu_item();
    Stats.print_menu_item();
    loop {
        String::clear(&mut buf);
        print!("🍁 ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buf) {
            Ok(_) => {
                let _ = crate::readline::clear_last_n_lines(1);
            }
            Err(e) => println!("Error in adventure menu readline: {:#?}", e),
        }

        let s = buf.trim();
        if let Ok(action) = s.parse::<AdventureAction>() {
            return action;
        }
    }
}

pub struct Adventure<'a> {
    maze: Maze,
    player: &'a mut Player,
}

impl<'a> Adventure<'a> {
    pub fn new(player: &'a mut Player) -> Self {
        Self {
            player,
            maze: Maze::new_demo(),
        }
    }
    pub fn run(&mut self) {
        'outer: loop {
            match adventure_menu() {
                AdventureAction::Movement => loop {
                    match self.maze.action() {
                        MazeEvent::Interact(Element::Monster(kind)) => {
                            let mut enc = Encounter::new(kind, &mut self.player);
                            match enc.run() {
                                PlayerVictory => {
                                    let xp = enc.monster.experience_points();
                                    println!("You earned {xp} experience points!");
                                    self.player.xp += xp;
                                    self.player.update_level();
                                }
                                MonsterVictory => break 'outer,
                                _ => (),
                            }
                        }
                        MazeEvent::Quit => break,
                        MazeEvent::Movement => (),
                        _ => (),
                    }
                },
                AdventureAction::Town => {
                    break;
                }
                AdventureAction::Inventory => self.player.noncombat_inventory(),
                AdventureAction::Equipment => self.player.noncombat_equipment(),
                AdventureAction::Stats => {
                    println!("{}", self.player.attribute_message())
                }
                AdventureAction::Cast => {
                    if let Some(spell) = spell_menu(self.player.intellect()) {
                        match self.player.cast_spell(spell) {
                            Some(SpellCast::Offense(_)) => {
                                println!("There is no target!")
                            }
                            Some(SpellCast::Defense(x)) => self.player.receive_defensive_spell(x),
                            None => println!("Insufficient MP!"),
                        }
                    }
                }
            }
        }
    }
}
