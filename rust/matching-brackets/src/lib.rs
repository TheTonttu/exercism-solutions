type BracketPair = (char, char);

const OPEN_BRACKETS: [char; 3] = ['(', '[', '{'];
const CLOSED_BRACKETS: [char; 3] = [')', ']', '}'];

const PARENTHESES: BracketPair = ('(', ')');
const BRACES: BracketPair = ('{', '}');
const SQUARE_BRACKETS: BracketPair = ('[', ']');

const BRACKET_PAIRS: [BracketPair; 3] = [PARENTHESES, BRACES, SQUARE_BRACKETS];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut expected_closed_brackets = Vec::new();

    for char in string.chars() {
        match char {
            c if is_open_bracket(&c) => expected_closed_brackets.push(get_closed_pair(&c)),
            c if is_closed_bracket(&c) => {
                if expected_closed_brackets.pop() != Some(c) {
                    return false;
                }
            }
            _ => (),
        }
    }

    expected_closed_brackets.is_empty()
}

fn is_open_bracket(char: &char) -> bool {
    OPEN_BRACKETS.contains(char)
}

fn is_closed_bracket(char: &char) -> bool {
    CLOSED_BRACKETS.contains(char)
}

fn get_closed_pair(open_bracket: &char) -> char {
    match BRACKET_PAIRS.iter().find(|(open, ..)| open == open_bracket) {
        Some((.., closed)) => *closed,
        None => panic!(format!("Unexpected '{}' open bracket", open_bracket)),
    }
}
