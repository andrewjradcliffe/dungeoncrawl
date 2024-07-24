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
    Empty,
}
use Element::*;

impl Element {
    pub const fn symbol(&self) -> char {
        match self {
            Player => '@',  // '🧝'
            Monster => 'm', // '👾',
            Tree => 't',    // '🌳',
            Rock => 'r',    // '🪨',
            Empty => '.',   // '🪜',
                             // Player => '🧝',
                             // Monster => '👾',
                             // Tree => '🌳',
                             // Rock => '🪨',
                             // Empty => '🏾',
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
pub struct Map {
    pub(crate) grid: Grid<Element>,
    pub(crate) player: (usize, usize),
}
impl Map {
    pub fn new() -> Self {
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
                Ok(_) => (),
                Err(e) => println!("Error in map menu readline: {:#?}", e),
            }

            let s = buf.trim();
            if let Ok(action) = s.parse::<Direction>() {
                return action;
            }
        }
    }
    pub fn movement(&mut self) {
        let (i_0, j_0) = self
            .grid
            .cartesian_index(self.grid.inner.iter().position(|x| *x == Player).unwrap());
        let (i_1, j_1) = match Self::menu() {
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
        }
    }
}

pub fn demo_movement() {
    let mut map = Map::new();
    loop {
        println!("{}", map.grid);
        map.movement();
        let _ = crate::readline::clear_screen();
        let _ = crate::readline::cursor_topleft();
        // let _ = crate::readline::clear_last_n_lines(7);
    }
}
