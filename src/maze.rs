use crate::grid::*;
use crate::monster::MonsterKind;
use crate::utils::is_quit;
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Element {
    Player,
    Monster(MonsterKind),
    Tree,
    Rock,
    Treasure,
    Ladder,
    Empty,
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
        grid[(2, 3)] = Tree;
        grid[(3, 2)] = Rock;
        grid[(1, 2)] = Tree;
        grid[(5, 5)] = Monster(MonsterKind::Orc);
        grid[(7, 7)] = Monster(MonsterKind::Dragon);
        Self { grid, player }
    }
    pub fn menu(&self) -> MazeAction {
        let mut buf = String::with_capacity(1 << 10);
        let n = self.grid.n_rows() + 1;
        println!("==== Select a direction... ====");
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
                Monster(kind) => {
                    println!("It's a {kind}!");
                    MazeEvent::Interact(Monster(kind))
                }
                Tree => {
                    println!("It's a shady tree!");
                    MazeEvent::Interact(Tree)
                }
                Rock => {
                    println!("It's a warm rock!");
                    MazeEvent::Interact(Rock)
                }
                Treasure => {
                    println!("It's a treasure box");
                    MazeEvent::Interact(Treasure)
                }
                Ladder => {
                    println!("You climb the ladder...");
                    MazeEvent::Interact(Ladder)
                }
                Empty => {
                    println!("There's nothing there.");
                    MazeEvent::Interact(Empty)
                }
                _ => MazeEvent::Interact(Empty),
            }
        } else {
            MazeEvent::Interact(Empty)
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
    Interact(Element),
    Movement,
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
