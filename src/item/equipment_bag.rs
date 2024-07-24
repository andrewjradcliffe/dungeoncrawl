use crate::equipment::*;
use ansi_term::{Colour, Style};
use indexmap::{map::Entry, IndexMap};
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipmentBag {
    bag: IndexMap<Gear, usize>,
    sum: usize,
}

impl EquipmentBag {
    pub fn new() -> Self {
        Self {
            bag: IndexMap::with_capacity(Gear::total_variants()),
            sum: 0,
        }
    }
    pub fn new_player() -> Self {
        [(Sword, 1), (Helmet, 1), (Breastplate, 1), (Gauntlet, 1)]
            .into_iter()
            .collect()
    }
    pub fn is_empty(&self) -> bool {
        self.sum == 0
    }

    pub fn menu(&self, msg: &str) -> EquipmentTransaction {
        let mut buf = String::with_capacity(1 << 7);
        println!("---- Entering equipment menu... ----");
        println!("{}", msg);
        if self.is_empty() {
            EquipmentTransaction::Quit
        } else {
            loop {
                buf.clear();
                print!("👜 ");
                io::Write::flush(&mut io::stdout()).unwrap();
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                match handle.read_line(&mut buf) {
                    Ok(_) => (),
                    Err(e) => println!("Error in inventory menu readline: {:#?}", e),
                }
                if let Ok(transaction) = buf.parse::<EquipmentTransaction>() {
                    break transaction;
                }
            }
        }
    }
    pub fn pop_item(&mut self, kind: Gear) -> Option<Gear> {
        match self.bag.entry(kind) {
            Entry::Occupied(mut v) => {
                if *v.get() > 0 {
                    self.sum -= 1;
                    *v.get_mut() -= 1;
                    Some(kind)
                } else {
                    None
                }
            }
            Entry::Vacant(_) => None,
        }
    }
    pub fn pop_multiple(&mut self, kind: Gear, n: usize) -> Option<(Gear, usize)> {
        match self.bag.entry(kind) {
            Entry::Occupied(mut v) => match *v.get() {
                0 => None,
                u if u >= n => {
                    self.sum -= n;
                    *v.get_mut() -= n;
                    Some((kind, n))
                }
                u => {
                    self.sum -= u;
                    *v.get_mut() = 0;
                    Some((kind, u))
                }
            },
            Entry::Vacant(_) => None,
        }
    }
    pub fn drop_multiple(&mut self, kind: Gear, n: usize) {
        self.pop_multiple(kind, n);
    }
    pub fn drop_item(&mut self, kind: Gear) {
        self.pop_item(kind);
    }
    pub fn push_multiple(&mut self, kind: Gear, count: usize) {
        self.sum += count;
        match self.bag.entry(kind) {
            Entry::Occupied(mut v) => {
                *v.get_mut() += count;
            }
            Entry::Vacant(e) => {
                e.insert(count);
            }
        }
    }
    pub fn push(&mut self, kind: Gear) {
        self.push_multiple(kind, 1)
    }
    pub fn push_duplicated(&mut self, kind: Gear, count: usize) {
        self.push_multiple(kind, count)
    }
    pub fn n_available(&self, item: &Gear) -> usize {
        self.bag.get(item).map(Clone::clone).unwrap_or(0)
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
            for (item, count) in self
                .bag
                .iter()
                .filter(|(kind, count)| **count > 0 && **kind != Gear::Fist && **kind != Gear::Bare)
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
        let mut inv = Self::new();
        for (item, count) in iter {
            inv.push_multiple(item, count);
        }
        inv
    }
}

impl fmt::Display for EquipmentBag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_imp(f, "value")
    }
}
