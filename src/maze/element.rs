use crate::{maze::core::*, monster::MonsterKind};
use std::{convert::TryFrom, fmt};

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
    InactivePortal,
    ActivePortal(Destination),
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
            InactivePortal | ActivePortal(_) => '🪞',
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
            '🪞' => InactivePortal,
            '🔶' => Fence,
            '⬛' => Wall,
            _ => match MonsterKind::try_from(value) {
                Ok(kind) => Monster(kind),
                Err(_) => return Err(()),
            },
        })
    }
}
