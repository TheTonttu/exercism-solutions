#[macro_use]
extern crate lazy_static;
use regex::Regex;

lazy_static! {
    // Initialize regex only once.
    static ref RE: Regex = Regex::new(r"\b[_]*(?P<first>[A-Za-z0-9])[\w\d]*").unwrap();
}

pub fn abbreviate(phrase: &str) -> String {
    RE.captures_iter(phrase)
        .map(|caps| caps["first"].to_uppercase())
        .collect::<String>()
}
