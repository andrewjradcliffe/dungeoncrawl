use regex::Regex;
use std::{fmt, str::FromStr, sync::LazyLock};
use yansi::Paint;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum CombatAction {
    Attack,
    Run,
    ShowInventory,
    Cast,
    DoNothing,
}
pub use CombatAction::*;

impl FromStr for CombatAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_ATTACK: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("(?i)^(?:attack|a)$").unwrap());
        static RE_RUN: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)^(?:run|r)$").unwrap());
        static RE_INV: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("(?i)^(?:inventory|i)$").unwrap());
        static RE_CAST: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)^(?:cast|c)$").unwrap());
        static RE_NOOP: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(?i)^(?:nothing|n)$").unwrap());

        if RE_ATTACK.is_match(s) {
            Ok(Attack)
        } else if RE_CAST.is_match(s) {
            Ok(Cast)
        } else if RE_INV.is_match(s) {
            Ok(ShowInventory)
        } else if RE_RUN.is_match(s) {
            Ok(Run)
        } else if RE_NOOP.is_match(s) {
            Ok(DoNothing)
        } else {
            Err(s.to_string())
        }
    }
}

impl fmt::Display for CombatAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! arm {
            ($fmt_str:literal, $ch:literal) => {
                write!(f, $fmt_str, $ch.underline().bold())
            };
        }
        match self {
            Attack => arm!("{}ttack", "A"),
            Run => arm!("{}un", "R"),
            ShowInventory => arm!("{}nventory", "I"),
            Cast => arm!("{}ast", "C"),
            DoNothing => arm!("{}othing", "N"),
        }
    }
}
