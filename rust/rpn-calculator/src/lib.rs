#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut pending_values = Vec::new();

    for input in inputs {
        if let CalculatorInput::Value(n) = input {
            pending_values.push(*n)
        } else {
            let result = match input {
                CalculatorInput::Add => pending_values
                    .pop()
                    .and_then(|a| pending_values.pop().map(|b| b + a)),
                CalculatorInput::Subtract => pending_values
                    .pop()
                    .and_then(|a| pending_values.pop().map(|b| b - a)),
                CalculatorInput::Multiply => pending_values
                    .pop()
                    .and_then(|a| pending_values.pop().map(|b| b * a)),
                CalculatorInput::Divide => pending_values
                    .pop()
                    .and_then(|a| pending_values.pop().map(|b| b / a)),
                _ => return None,
            };

            if let Some(n) = result {
                pending_values.push(n);
            } else {
                return None;
            }
        }
    }

    if pending_values.len() == 1 {
        pending_values.pop()
    } else {
        None
    }
}
