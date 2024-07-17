use once_cell::sync::Lazy;
use regex::Regex;
// use regex::{RegexSet, RegexSetBuilder};
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum AdventureAction {
    Encounter,
    Adventure,
}
