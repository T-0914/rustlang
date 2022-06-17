#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(n) => stack.push(*n),
            _ => match stack.len() {
                0 | 1 => return None,
                _ => {
                    let first_number = stack.pop().unwrap();
                    let second_number = stack.pop().unwrap();
                    match input {
                        CalculatorInput::Add => stack.push(first_number + second_number),
                        CalculatorInput::Subtract => stack.push(second_number - first_number),
                        CalculatorInput::Multiply => stack.push(first_number * second_number),
                        CalculatorInput::Divide => stack.push(second_number / first_number),
                        _ => return None,
                    }
                }
            },
        }
    }
    if stack.len() != 1 {
        None
    } else {
        stack.pop()
    }
}
