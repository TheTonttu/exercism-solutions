const QUESTION_REPLY: &str = "Sure.";
const SHOUT_REPLY: &str = "Whoa, chill out!";
const SHOUT_QUESTION_REPLY: &str = "Calm down, I know what I'm doing!";
const SILENCE_REPLY: &str = "Fine. Be that way!";
const ELSE_REPLY: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();

    if is_silence(trimmed_message) {
        SILENCE_REPLY
    } else if is_question(trimmed_message) {
        if is_shouting(trimmed_message) {
            SHOUT_QUESTION_REPLY
        } else {
            QUESTION_REPLY
        }
    } else if is_shouting(trimmed_message) {
        SHOUT_REPLY
    } else {
        ELSE_REPLY
    }
}

fn is_silence(message: &str) -> bool {
    message.is_empty()
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_shouting(message: &str) -> bool {
    let alphabetic_chars: Vec<char> = message.chars().filter(|c| c.is_alphabetic()).collect();

    if alphabetic_chars.is_empty() {
        false
    } else {
        alphabetic_chars.iter().all(|c| c.is_uppercase())
    }
}
