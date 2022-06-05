use CalculatorInput::*;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut values = Vec::new();

    for input in inputs {
        if let Value(n) = input {
            values.push(*n);
            continue;
        }

        let value_pair = values.pop().and_then(|a| values.pop().map(|b| (b, a)));
        let result = if let Some((a, b)) = value_pair {
            let operator = input;
            calculate(operator, &a, &b)
        } else {
            None
        };

        if let Some(n) = result {
            values.push(n);
        } else {
            return None;
        }
    }

    if values.len() == 1 {
        values.pop()
    } else {
        None
    }
}

fn calculate(input: &CalculatorInput, a: &i32, b: &i32) -> Option<i32> {
    match input {
        Add => Some(a + b),
        Subtract => Some(a - b),
        Multiply => Some(a * b),
        Divide => Some(a / b),
        _ => None,
    }
}
