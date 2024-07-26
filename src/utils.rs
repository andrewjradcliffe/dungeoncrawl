use regex::Regex;
use std::sync::LazyLock;

pub(crate) fn is_quit(s: &str) -> bool {
    static RE_QUIT: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?i)^(?:quit|q)$").unwrap());
    RE_QUIT.is_match(s)
}
