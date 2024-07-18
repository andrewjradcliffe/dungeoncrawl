use ansi_term::{
    ANSIGenericString,
    Colour::{Blue, Green, Red},
};
use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) fn is_quit(s: &str) -> bool {
    static RE_QUIT: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:quit|q)$").unwrap());
    RE_QUIT.is_match(s)
}

pub(crate) static ANSI_HP: Lazy<ANSIGenericString<'_, str>> = Lazy::new(|| Red.bold().paint("HP"));
pub(crate) static ANSI_MP: Lazy<ANSIGenericString<'_, str>> =
    Lazy::new(|| Green.bold().paint("MP"));
pub(crate) static ANSI_TP: Lazy<ANSIGenericString<'_, str>> = Lazy::new(|| Blue.bold().paint("TP"));
