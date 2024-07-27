use crate::{grid::*, monster::MonsterKind, utils::is_quit};
use regex::Regex;
use std::{
    convert::TryFrom,
    fmt,
    io::{self, BufRead},
    str::FromStr,
    sync::LazyLock,
};
use yansi::Paint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Element {
    Player,
    Monster(MonsterKind),
    Tree,
    Rock,
    Treasure,
    Ladder,
    Empty,
    Dungeon,
    Portal,
    Fence,
    Wall,
}
use Element::*;

impl Element {
    pub const fn symbol(&self) -> char {
        match self {
            Player => '🧝',
            Monster(kind) => kind.symbol(),
            Tree => '🌳',
            Rock => '🪨',
            Treasure => '🎁',
            Ladder => '🪜',
            Empty => '⬜',
            Dungeon => '🏰',
            Portal => '🪞',
            Fence => '🔶',
            Wall => '⬛',
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
impl Default for Element {
    fn default() -> Self {
        Empty
    }
}

impl TryFrom<char> for Element {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '🧝' => Player,
            '🌳' => Tree,
            '🪨' => Rock,
            '🎁' => Treasure,
            '🪜' => Ladder,
            '⬜' => Empty,
            '🏰' => Dungeon,
            '🪞' => Portal,
            '🔶' => Fence,
            '⬛' => Wall,
            _ => match MonsterKind::try_from(value) {
                Ok(kind) => Monster(kind),
                Err(_) => return Err(()),
            },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Forward,
    Backward,
}
use Direction::*;

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE_UP: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)^(?:up|u)$").unwrap());
        static RE_DOWN: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)^(?:down|d)$").unwrap());
        static RE_FORWARD: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("(?i)^(?:forward|f)$").unwrap());
        static RE_BACKWARD: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("(?i)^(?:backward|b)$").unwrap());

        let s = s.trim();
        if RE_UP.is_match(s) {
            Ok(Up)
        } else if RE_DOWN.is_match(s) {
            Ok(Down)
        } else if RE_FORWARD.is_match(s) {
            Ok(Forward)
        } else if RE_BACKWARD.is_match(s) {
            Ok(Backward)
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Up => write!(f, "{}p", "U".underline().bold()),
            Down => write!(f, "{}own", "D".underline().bold()),
            Forward => write!(f, "{}orward", "F".underline().bold()),
            Backward => write!(f, "{}ackward", "B".underline().bold()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Maze {
    pub(crate) grid: Grid<Element>,
    pub(crate) player: (usize, usize),
}
impl Maze {
    pub fn new_default(n_rows: usize, n_cols: usize) -> Self {
        assert_ne!(n_rows, 0);
        assert_ne!(n_cols, 0);
        let mut grid = Grid::new_default(n_rows, n_cols);
        let player = (n_rows / 2, n_cols / 2);
        grid[player] = Player;
        Self { grid, player }
    }
    pub fn new_demo() -> Self {
        let mut grid = Grid::new_default(20, 20);
        let player = (2, 2);
        grid[player] = Player;
        grid[(2, 1)] = Tree;
        grid[(3, 2)] = Rock;
        grid[(1, 2)] = Tree;
        grid[(2, 7)] = Monster(MonsterKind::Orc);
        grid[(7, 7)] = Monster(MonsterKind::Dragon);
        grid[(4, 5)] = Monster(MonsterKind::Frog);
        grid[(4, 6)] = Monster(MonsterKind::Bat);
        grid[(4, 7)] = Monster(MonsterKind::Wolf);
        grid[(4, 8)] = Monster(MonsterKind::Goblin);
        grid[(8, 1)] = Treasure;

        grid[(0, 0)] = Portal;
        grid[(10, 10)] = Dungeon;

        grid[(15, 15)] = Fence;
        grid[(15, 16)] = Fence;
        grid[(15, 17)] = Fence;
        grid[(15, 18)] = Fence;
        grid[(16, 18)] = Fence;
        grid[(17, 18)] = Fence;
        grid[(18, 18)] = Fence;
        grid[(18, 15)] = Fence;
        grid[(18, 16)] = Fence;
        grid[(18, 17)] = Fence;

        Self { grid, player }
    }
    pub fn menu(&self) -> MazeAction {
        let mut buf = String::with_capacity(1 << 10);
        let n = self.grid.n_rows() + 1;
        println!(
            "==== Select a direction... {}, {}, {}, or {} ====",
            Up, Down, Forward, Backward
        );
        println!("{}", self.grid);
        loop {
            String::clear(&mut buf);

            print!("👣 ");
            io::Write::flush(&mut io::stdout()).unwrap();

            let stdin = io::stdin();
            let mut handle = stdin.lock();
            match handle.read_line(&mut buf) {
                Ok(_) => {
                    let _ = crate::readline::clear_last_n_lines(1);
                }
                Err(e) => println!("Error in map menu readline: {:#?}", e),
            }

            let s = buf.trim();
            if let Ok(action) = s.parse::<MazeAction>() {
                let _ = crate::readline::clear_last_n_lines(n);
                return action;
            }
        }
    }
    pub fn position(&self, dir: Direction) -> Option<(usize, usize)> {
        let (i_0, j_0) = self.player.clone();
        match dir {
            Up => {
                if i_0 == 0 {
                    None
                } else {
                    Some((i_0 - 1, j_0))
                }
            }
            Down => {
                let i_1 = i_0 + 1;
                if i_1 == self.grid.n_rows() {
                    None
                } else {
                    Some((i_1, j_0))
                }
            }
            Forward => {
                let j_1 = j_0 + 1;
                if j_1 == self.grid.n_cols() {
                    None
                } else {
                    Some((i_0, j_1))
                }
            }
            Backward => {
                if j_0 == 0 {
                    None
                } else {
                    Some((i_0, j_0 - 1))
                }
            }
        }
    }
    pub(crate) fn movement_imp(&mut self, dir: Direction) -> MazeEvent {
        if let Some(new_pos) = self.position(dir) {
            if self.grid[new_pos] == Empty {
                self.grid[self.player] = Empty;
                self.grid[new_pos] = Player;
                self.player = new_pos;
            }
        }
        MazeEvent::Movement
    }
    pub(crate) fn interact_imp(&mut self, dir: Direction) -> MazeEvent {
        if let Some(new_pos) = self.position(dir) {
            match self.grid[new_pos] {
                Monster(kind) => MazeEvent::Interact(Monster(kind), new_pos),
                Tree => {
                    println!("It's a shady tree!");
                    MazeEvent::Interact(Tree, new_pos)
                }
                Rock => {
                    println!("It's a warm rock!");
                    MazeEvent::Interact(Rock, new_pos)
                }
                Treasure => {
                    println!("It's a treasure box!");
                    MazeEvent::Interact(Treasure, new_pos)
                }
                Ladder => {
                    println!("You climb the ladder...");
                    MazeEvent::Interact(Ladder, new_pos)
                }
                Dungeon => {
                    println!("You enter the dungeon...");
                    MazeEvent::Interact(Dungeon, new_pos)
                }
                Portal => {
                    println!("You step into the portal...");
                    MazeEvent::Interact(Portal, new_pos)
                }
                Fence => {
                    println!("It's a fence.");
                    MazeEvent::Interact(Fence, new_pos)
                }
                Wall => {
                    println!("It's a wall.");
                    MazeEvent::Interact(Wall, new_pos)
                }
                Empty => {
                    println!("There's nothing there.");
                    MazeEvent::Interact(Empty, new_pos)
                }
                _ => MazeEvent::Interact(Empty, new_pos),
            }
        } else {
            MazeEvent::NoOp
        }
    }
    pub fn action(&mut self) -> MazeEvent {
        match self.menu() {
            MazeAction::Interact(dir) => self.interact_imp(dir),
            MazeAction::Movement(dir) => self.movement_imp(dir),
            MazeAction::Quit => MazeEvent::Quit,
        }
    }
}

pub fn demo_movement() {
    let mut maze = Maze::new_demo();
    loop {
        // println!("{}", maze.grid);
        maze.action();
        // let _ = crate::readline::clear_screen();
        // let _ = crate::readline::cursor_topleft();
        // let _ = crate::readline::clear_last_n_lines(n);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MazeAction {
    Interact(Direction),
    Movement(Direction),
    Quit,
}

impl FromStr for MazeAction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE_INTERACT: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("(?i)^(?:interact|i)$").unwrap());
        let s = s.trim();
        if let Ok(dir) = s.parse::<Direction>() {
            return Ok(MazeAction::Movement(dir));
        } else if let Some((lhs, rhs)) = s.split_once(' ') {
            if RE_INTERACT.is_match(lhs.trim()) {
                if let Ok(dir) = rhs.parse::<Direction>() {
                    return Ok(MazeAction::Interact(dir));
                }
            }
        } else if is_quit(s) {
            return Ok(MazeAction::Quit);
        }
        Err(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MazeEvent {
    Interact(Element, (usize, usize)),
    Movement,
    NoOp,
    Quit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement() {
        let mut maze = Maze::new_default(10, 10);
        assert_eq!(maze.grid[(5, 5)], Player);
        for _ in 0..5 {
            maze.movement_imp(Up);
        }
        assert_eq!(maze.grid[(0, 5)], Player);
        for _ in 0..5 {
            maze.movement_imp(Down);
        }
        for _ in 0..5 {
            maze.movement_imp(Forward);
        }
        assert_eq!(maze.grid[(5, 9)], Player);
        for _ in 0..9 {
            maze.movement_imp(Backward);
        }
        assert_eq!(maze.grid[(5, 0)], Player);
    }
}
