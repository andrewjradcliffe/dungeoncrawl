use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) fn is_quit(s: &str) -> bool {
    static RE_QUIT: Lazy<Regex> = Lazy::new(|| Regex::new("(?i)^(?:quit|q)$").unwrap());
    RE_QUIT.is_match(s)
}
