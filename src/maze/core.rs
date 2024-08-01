use crate::maze::element::*;
use crate::{grid::*, monster::MonsterKind, utils::is_quit};
use indexmap::IndexMap;
use rand::Rng;
use regex::Regex;
use std::{
    fmt,
    io::{self, BufRead},
    str::FromStr,
    sync::LazyLock,
};
use yansi::Paint;

use Element::*;

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
    pub(crate) monsters: IndexMap<(usize, usize), MonsterKind>,
    pub(crate) active_portals: Vec<(usize, usize)>,
}
impl Maze {
    pub fn hide_player_mark(&mut self) {
        self.grid[self.player] = Empty;
    }
    pub fn show_player_mark(&mut self) {
        self.grid[self.player] = Player;
    }
    pub fn reconcile_monster_positions(&mut self) {
        let pos = self.player.clone();
        let mut state = self.monsters.contains_key(&pos);
        loop {
            if state {
                if let Some(dst) = self.first_movement_proposal(pos) {
                    self.move_monster(pos, dst);
                    state = false;
                } else {
                    self.monster_movement();
                    state = self.monsters.contains_key(&pos);
                }
            } else {
                break;
            }
        }
    }
    pub fn new_default(n_rows: usize, n_cols: usize) -> Self {
        assert_ne!(n_rows, 0);
        assert_ne!(n_cols, 0);
        let mut grid = Grid::new_default(n_rows, n_cols);
        let player = (n_rows / 2, n_cols / 2);
        grid[player] = Player;
        Self {
            grid,
            player,
            monsters: IndexMap::new(),
            active_portals: Vec::new(),
        }
    }
    pub(crate) fn create_portal(&mut self, src_position: (usize, usize), dst: Destination) {
        match &mut self.grid[src_position] {
            ActivePortal(old_dst) => {
                *old_dst = dst;
            }
            x => {
                *x = ActivePortal(dst);
                self.active_portals.push(src_position);
            }
        }
    }
    pub fn spawn_monster(&mut self, kind: MonsterKind, pos: (usize, usize)) -> bool {
        if self.grid.check_bounds(pos) && self.grid[pos] == Empty {
            self.grid[pos] = Monster(kind);
            self.monsters.insert(pos, kind);
            true
        } else {
            false
        }
    }
    pub fn remove_monster(&mut self, pos: (usize, usize)) -> Option<MonsterKind> {
        if self.grid.check_bounds(pos) {
            match self.grid[pos] {
                Monster(_) => {
                    self.grid[pos] = Empty;
                    self.monsters.swap_remove(&pos)
                }
                _ => None,
            }
        } else {
            None
        }
    }
    pub(crate) fn move_monster(&mut self, src: (usize, usize), dst: (usize, usize)) {
        // Simplest implementation, but if anything more than MonsterKind is used
        // in the future, then one should prefer the more nuanced implementation.
        if let Some(kind) = self.remove_monster(src) {
            self.spawn_monster(kind, dst);
        }
        // Direct implementation, without checks
        // self.grid.swap(src, dst);
        // let kind = self.monsters.swap_remove(&src).unwrap();
        // self.monsters.insert(dst, kind);
    }
    pub(crate) fn monster_movement(&mut self) {
        let mut rng = rand::thread_rng();
        // We need a hard copy due to the fact that the monster storage
        // needs to be updated after each movement.
        let srcs: Vec<_> = self.monsters.keys().cloned().collect();
        for pos in srcs {
            let proposals = self.movement_proposals(pos);
            let n = proposals.len();
            if n > 0 {
                let i = rng.gen_range(0..n);
                let dst = proposals[i];
                self.move_monster(pos, dst);
            }
        }
    }
    pub fn new_demo() -> Self {
        let mut grid = Grid::new_default(20, 20);
        let player = (2, 2);
        grid[player] = Player;
        grid[(2, 1)] = Tree;
        grid[(3, 2)] = Rock;
        grid[(1, 2)] = Tree;
        grid[(8, 1)] = Treasure;

        grid[(0, 0)] = InactivePortal;
        grid[(8, 8)] = InactivePortal;
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

        let mut maze = Self {
            grid,
            player,
            monsters: IndexMap::new(),
            active_portals: Vec::new(),
        };

        maze.spawn_monster(MonsterKind::Frog, (4, 5));
        maze.spawn_monster(MonsterKind::Bat, (4, 6));
        maze.spawn_monster(MonsterKind::Snake, (5, 6));
        maze.spawn_monster(MonsterKind::Wolf, (4, 7));
        maze.spawn_monster(MonsterKind::Goblin, (4, 8));
        maze.spawn_monster(MonsterKind::Bear, (6, 6));
        maze.spawn_monster(MonsterKind::Undead, (15, 10));
        maze.spawn_monster(MonsterKind::Orc, (2, 7));
        maze.spawn_monster(MonsterKind::Vampire, (9, 9));
        maze.spawn_monster(MonsterKind::Troll, (10, 15));
        maze.spawn_monster(MonsterKind::Mammoth, (7, 7));
        maze.spawn_monster(MonsterKind::Dragon, (18, 7));
        maze
    }
    pub fn new_room() -> Self {
        let mut grid = Grid::new_default(5, 5);
        let player = (1, 2);
        grid[player] = Player;
        grid[(2, 2)] = Treasure;
        grid[(0, 0)] = Wall;
        grid[(0, 1)] = Wall;
        grid[(0, 2)] = InactivePortal;
        grid[(0, 3)] = Wall;
        grid[(0, 4)] = Wall;
        grid[(1, 0)] = Wall;
        grid[(2, 0)] = Wall;
        grid[(3, 0)] = Wall;
        grid[(4, 0)] = Wall;
        grid[(4, 1)] = Wall;
        grid[(4, 2)] = Wall;
        grid[(4, 3)] = Wall;
        grid[(4, 4)] = Wall;
        grid[(3, 4)] = Wall;
        grid[(2, 4)] = Wall;
        grid[(1, 4)] = Wall;
        Self {
            grid,
            player,
            monsters: IndexMap::new(),
            active_portals: Vec::new(),
        }
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
    pub(crate) fn position_imp(
        &self,
        (i_0, j_0): (usize, usize),
        dir: Direction,
    ) -> Option<(usize, usize)> {
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
    pub fn position(&self, dir: Direction) -> Option<(usize, usize)> {
        self.position_imp(self.player.clone(), dir)
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
    fn movement_proposals_imp(
        &self,
        pos: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        [Up, Down, Forward, Backward]
            .into_iter()
            .filter_map(move |dir| self.position_imp(pos, dir))
            .filter(move |new_pos| self.grid[*new_pos] == Empty)
    }
    pub(crate) fn movement_proposals(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        self.movement_proposals_imp(pos).collect()
    }
    pub(crate) fn first_movement_proposal(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        self.movement_proposals_imp(pos).next()
    }
    fn portal_proposals_imp(
        &self,
        pos: (usize, usize),
        other: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.movement_proposals_imp(pos)
            .filter(move |new_pos| *new_pos != other)
    }
    pub(crate) fn first_portal_proposal(
        &self,
        pos: (usize, usize),
        other: (usize, usize),
    ) -> Option<(usize, usize)> {
        self.portal_proposals_imp(pos, other).next()
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
                InactivePortal => {
                    println!("The portal is inactive.");
                    MazeEvent::Interact(InactivePortal, new_pos)
                }
                ActivePortal(x) => {
                    println!("You step into the portal...");
                    MazeEvent::Interact(ActivePortal(x), new_pos)
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

// pub fn demo_movement() {
//     let mut maze = Maze::new_demo();
//     loop {
//         // println!("{}", maze.grid);
//         maze.action();
//         // let _ = crate::readline::clear_screen();
//         // let _ = crate::readline::cursor_topleft();
//         // let _ = crate::readline::clear_last_n_lines(n);
//     }
// }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Destination {
    pub(crate) index: usize,
    pub(crate) position: (usize, usize),
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
