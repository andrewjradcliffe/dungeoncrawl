use crate::grid::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Element {
    Player,
    Monster,
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
            // Player => '@',  // '🧝'
            // Monster => 'm', // '👾',
            // Tree => 't',    // '🌳',
            // Rock => 'r',    // '🪨',
            // Empty => '.',   // '🪜',
            Player => '🧝',
            Monster => '👾',
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
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE_UP: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:up|u)$").unwrap());
        static RE_DOWN: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:down|d)$").unwrap());
        static RE_FORWARD: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:forward|f)$").unwrap());
        static RE_BACKWARD: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:backward|b)$").unwrap());

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
            Err(s.to_string())
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
        let mut grid = Grid::new_default(n_rows, n_cols);
        let player = (n_rows / 2, n_cols / 2);
        grid[player] = Player;
        Self { grid, player }
    }
    pub fn new_demo() -> Self {
        let mut grid = Grid::new_default(10, 10);
        let player = (2, 2);
        grid[player] = Player;
        grid[(2, 3)] = Tree;
        grid[(3, 2)] = Rock;
        grid[(1, 2)] = Tree;
        Self { grid, player }
    }
    pub fn menu() -> Direction {
        let mut buf = String::with_capacity(1 << 10);
        println!("==== Select a direction... ====");
        loop {
            buf.clear();
            // print!("adventure > ");
            // io::Write::flush(&mut io::stdout());

            let stdin = io::stdin();
            let mut handle = stdin.lock();
            match handle.read_line(&mut buf) {
                Ok(_) => {
                    let _ = crate::readline::clear_last_n_lines(1);
                }
                Err(e) => println!("Error in map menu readline: {:#?}", e),
            }

            let s = buf.trim();
            if let Ok(action) = s.parse::<Direction>() {
                // let _ = crate::readline::clear_last_n_lines(1);
                return action;
            }
        }
    }
    pub(crate) fn movement_imp(&mut self, dir: Direction) {
        let (i_0, j_0) = self.player.clone();
        let (i_1, j_1) = match dir {
            Up => {
                let i_1 = if i_0 == 0 { 0 } else { i_0 - 1 };
                let j_1 = j_0;
                (i_1, j_1)
            }
            Down => {
                let i_1 = i_0 + 1;
                let j_1 = j_0;
                (if i_1 == self.grid.n_rows { i_0 } else { i_1 }, j_1)
            }
            Forward => {
                let i_1 = i_0;
                let j_1 = j_0 + 1;
                (i_1, if j_1 == self.grid.n_cols { j_0 } else { j_1 })
            }
            Backward => {
                let i_1 = i_0;
                let j_1 = if j_0 == 0 { 0 } else { j_0 - 1 };
                (i_1, j_1)
            }
        };
        if self.grid[(i_1, j_1)] == Empty {
            self.grid[(i_0, j_0)] = Empty;
            self.grid[(i_1, j_1)] = Player;
            self.player = (i_1, j_1);
        }
    }
    pub fn movement(&mut self) {
        self.movement_imp(Self::menu());
    }
}

pub fn demo_movement() {
    let mut maze = Maze::new_demo();
    let n = maze.grid.n_rows() + 1;
    loop {
        println!("{}", maze.grid);
        maze.movement();
        // let _ = crate::readline::clear_screen();
        // let _ = crate::readline::cursor_topleft();
        let _ = crate::readline::clear_last_n_lines(n);
    }
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
