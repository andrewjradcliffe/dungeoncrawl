use crate::equipment::*;
use crate::multiset::MultiSet;
use crate::utils::*;
use ansi_term::{Colour, Style};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EquipmentAction {
    Equip,
    Unequip,
    Quit,
}

impl FromStr for EquipmentAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        static RE_EQUIP: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:equip|e)$").unwrap());
        static RE_UNEQUIP: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:unequip|u)$").unwrap());

        if RE_EQUIP.is_match(s) {
            Ok(EquipmentAction::Equip)
        } else if RE_UNEQUIP.is_match(s) {
            Ok(EquipmentAction::Unequip)
        } else if is_quit(s) {
            Ok(EquipmentAction::Quit)
        } else {
            Err(s.to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EquipmentTransaction {
    Equip(Gear),
    Unequip(Gear),
    Quit,
}
impl FromStr for EquipmentTransaction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some((lhs, rhs)) = s.split_once(' ') {
            if let Ok(action) = lhs.parse::<EquipmentAction>() {
                if let Ok(gear) = rhs.parse::<Gear>() {
                    match action {
                        EquipmentAction::Equip => {
                            return Ok(EquipmentTransaction::Equip(gear));
                        }
                        EquipmentAction::Unequip => {
                            return Ok(EquipmentTransaction::Unequip(gear));
                        }
                        EquipmentAction::Quit => (),
                    }
                }
            }
        } else if let Ok(EquipmentAction::Quit) = s.parse::<EquipmentAction>() {
            return Ok(EquipmentTransaction::Quit);
        }
        Err(s.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentBag(MultiSet<Gear>);

impl EquipmentBag {
    pub fn new() -> Self {
        Self(MultiSet::with_capacity(Gear::total_variants()))
    }
    pub fn new_player() -> Self {
        [(Sword, 1), (Helmet, 1), (Breastplate, 1), (Gauntlet, 1)]
            .into_iter()
            .collect()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn menu(&self, msg: &str) -> EquipmentTransaction {
        let mut buf = String::with_capacity(1 << 7);
        println!("---- Entering equipment menu... ----");
        println!("{}", msg);
        loop {
            buf.clear();
            print!("👜 ");
            io::Write::flush(&mut io::stdout()).unwrap();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            match handle.read_line(&mut buf) {
                Ok(_) => (),
                Err(e) => println!("Error in equipment menu readline: {:#?}", e),
            }
            if let Ok(transaction) = buf.parse::<EquipmentTransaction>() {
                break transaction;
            }
        }
    }
    pub fn pop_item(&mut self, kind: Gear) -> Option<Gear> {
        self.0.pop_item(kind)
    }
    pub fn pop_multiple(&mut self, kind: Gear, n: usize) -> Option<(Gear, usize)> {
        self.0.pop_multiple(kind, n)
    }
    pub fn drop_multiple(&mut self, kind: Gear, n: usize) {
        self.0.drop_multiple(kind, n);
    }
    pub fn drop_item(&mut self, kind: Gear) {
        self.0.pop_item(kind);
    }
    pub fn push_multiple(&mut self, kind: Gear, count: usize) {
        match kind {
            Bare | Fist => (),
            kind => self.0.push_multiple(kind, count),
        }
    }
    pub fn push(&mut self, kind: Gear) {
        match kind {
            Bare | Fist => (),
            kind => self.0.push(kind),
        }
    }
    pub fn n_available(&self, kind: &Gear) -> usize {
        self.0.n_available(kind)
    }
    pub(crate) fn fmt_imp<T: fmt::Write>(&self, f: &mut T, field2: &'static str) -> fmt::Result {
        if self.is_empty() {
            writeln!(f, "EquipmentBag is empty!")?;
        } else {
            writeln!(
                f,
                "{}:",
                Style::new().bold().underline().paint("EquipmentBag")
            )?;
            writeln!(
                f,
                "                          | {} |  {}  |  {}",
                Style::new().underline().paint("available"),
                Style::new().underline().paint(field2),
                Style::new().underline().paint("effect"),
            )?;
            for (item, count) in
                self.0.bag.iter().filter(|(kind, count)| {
                    **count > 0 && **kind != Gear::Fist && **kind != Gear::Bare
                })
            {
                writeln!(
                    f,
                    "    {:<30} | {:^9} | {:>2} {} | {:<30}",
                    format!("{}", item),
                    count,
                    item.cost(),
                    Colour::Yellow.bold().paint("gold"),
                    item.description(),
                )?;
            }
        }
        Ok(())
    }
}
impl FromIterator<(Gear, usize)> for EquipmentBag {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Gear, usize)>,
    {
        Self(iter.into_iter().collect())
    }
}

impl fmt::Display for EquipmentBag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_imp(f, "value")
    }
}
