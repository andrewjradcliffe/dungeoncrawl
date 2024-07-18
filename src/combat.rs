use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

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

        static RE_ATTACK: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:attack|a)$").unwrap());
        static RE_RUN: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:run|r)$").unwrap());
        static RE_INV: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:inventory|i)$").unwrap());
        static RE_CAST: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:cast|c)$").unwrap());
        static RE_NOOP: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?i)^(:?do\s*)?(?:nothing|n)$").unwrap());

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
