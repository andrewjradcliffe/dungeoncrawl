use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Action {
    Attack,
    Run,
    ShowInventory,
}
pub use Action::*;

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("a") || s.eq_ignore_ascii_case("attack") {
            Ok(Attack)
        } else if s.eq_ignore_ascii_case("r") || s.eq_ignore_ascii_case("run") {
            Ok(Run)
        } else if s.eq_ignore_ascii_case("i") || s.eq_ignore_ascii_case("inventory") {
            Ok(ShowInventory)
        } else {
            Err(s.to_string())
        }
    }
}

pub trait Combatant {
    fn is_alive(&self) -> bool;

    fn receive_damage(&mut self, amount: i64);
}
