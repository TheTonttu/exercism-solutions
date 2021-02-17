const SILENCE_REPLY: &str = "Fine. Be that way!";
const SHOUT_QUESTION_REPLY: &str = "Calm down, I know what I'm doing!";
const QUESTION_REPLY: &str = "Sure.";
const SHOUT_REPLY: &str = "Whoa, chill out!";
const ELSE_REPLY: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if is_silence(m) => SILENCE_REPLY,
        m if is_question(m) => match m {
            q if is_shouting(q) => SHOUT_QUESTION_REPLY,
            _ => QUESTION_REPLY,
        },
        m if is_shouting(m) => SHOUT_REPLY,
        _ => ELSE_REPLY,
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

    match alphabetic_chars {
        ac if ac.is_empty() => false,
        ac => ac.iter().all(|c| c.is_uppercase()),
    }
}
