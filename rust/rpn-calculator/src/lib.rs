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

        let result = match input {
            Add => calculate(&mut values, |a, b| b + a),
            Subtract => calculate(&mut values, |a, b| b - a),
            Multiply => calculate(&mut values, |a, b| b * a),
            Divide => calculate(&mut values, |a, b| b / a),
            _ => return None,
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

fn calculate<T>(values: &mut Vec<i32>, op: T) -> Option<i32>
where
    T: Fn(i32, i32) -> i32,
{
    values.pop().and_then(|a| values.pop().map(|b| op(a, b)))
}
