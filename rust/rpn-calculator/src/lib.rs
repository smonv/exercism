#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() > 1 && inputs.len() < 3 {
        return None;
    }

    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        if let CalculatorInput::Value(v) = input {
            stack.push(*v)
        } else {
            let v2 = stack.pop();
            let v1 = stack.pop();

            if v1 == None || v2 == None {
                return None;
            }

            if let (Some(v1), Some(v2)) = (v1, v2) {
                let r = match *input {
                    CalculatorInput::Add => v1 + v2,
                    CalculatorInput::Multiply => v1 * v2,
                    CalculatorInput::Subtract => v1 - v2,
                    CalculatorInput::Divide => v1 / v2,
                    _ => unreachable!(),
                };

                stack.push(r)
            }
        }
    }

    stack.pop()
}
