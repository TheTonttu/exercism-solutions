const OPENING_BRACKETS: [char; 3] = ['(', '[', '{'];
const CLOSING_BRACKETS: [char; 3] = [')', ']', '}'];

const PARENTHESES: (char, char) = ('(', ')');
const BRACES: (char, char) = ('{', '}');
const SQUARE_BRACKETS: (char, char) = ('[', ']');

const BRACKET_PAIRS: [(char, char); 3] = [PARENTHESES, BRACES, SQUARE_BRACKETS];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut leftover_open_brackets = Vec::new();

    for char in string.chars() {
        match char {
            c if is_opening_bracket(&c) => leftover_open_brackets.push(c),
            c if is_closing_bracket(&c) => {
                if leftover_open_brackets.is_empty() {
                    return false;
                }

                let last_opening = leftover_open_brackets.pop().unwrap();
                let closing_pair = *BRACKET_PAIRS
                    .iter()
                    .find(|(opening, _closing)| *opening == last_opening)
                    .map(|(_opening, closing)| closing)
                    .unwrap();

                if c != closing_pair {
                    return false;
                }
            }
            _ => (),
        }
    }

    leftover_open_brackets.is_empty()
}

fn is_opening_bracket(c: &char) -> bool {
    OPENING_BRACKETS.contains(c)
}

fn is_closing_bracket(c: &char) -> bool {
    CLOSING_BRACKETS.contains(c)
}
