type BracketPair = (char, char);

const OPEN_BRACKETS: [char; 3] = ['(', '[', '{'];
const CLOSED_BRACKETS: [char; 3] = [')', ']', '}'];

const PARENTHESES: BracketPair = ('(', ')');
const BRACES: BracketPair = ('{', '}');
const SQUARE_BRACKETS: BracketPair = ('[', ']');

const BRACKET_PAIRS: [BracketPair; 3] = [PARENTHESES, BRACES, SQUARE_BRACKETS];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut leftover_open_brackets = Vec::new();

    for char in string.chars() {
        match char {
            c if is_open_bracket(&c) => leftover_open_brackets.push(c),
            c if is_closed_bracket(&c) => {
                if leftover_open_brackets.is_empty() {
                    return false;
                }

                let last_open_bracket = leftover_open_brackets.pop().unwrap();
                let matching_close_bracket = BRACKET_PAIRS
                    .iter()
                    .find_by_open_bracket(&last_open_bracket)
                    .unwrap()
                    .closed_bracket();

                if c != matching_close_bracket {
                    return false;
                }
            }
            _ => (),
        }
    }

    leftover_open_brackets.is_empty()
}

fn is_open_bracket(c: &char) -> bool {
    OPEN_BRACKETS.contains(c)
}

fn is_closed_bracket(c: &char) -> bool {
    CLOSED_BRACKETS.contains(c)
}

trait FindBracketPair {
    /// Find bracket pair based on open bracket.
    ///
    /// Returns the matching `BracketPair`, or `None` if none found.
    fn find_by_open_bracket(&mut self, open_bracket: &char) -> Option<&BracketPair>;
}

impl FindBracketPair for std::slice::Iter<'_, BracketPair> {
    fn find_by_open_bracket(&mut self, open_bracket: &char) -> Option<&BracketPair> {
        self.find(|(open, _closed)| open == open_bracket)
    }
}

trait BracketPairAccess {
    /// Get the closed bracket from `BracketPair`.
    ///
    /// Returns `BracketPair` closed bracket.
    fn closed_bracket(&self) -> char;
}

impl BracketPairAccess for BracketPair {
    fn closed_bracket(&self) -> char {
        let (_open, closed) = self;
        *closed
    }
}
