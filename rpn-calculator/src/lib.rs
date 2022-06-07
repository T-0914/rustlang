#[derive(Debug, Clone, Copy)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &Vec<CalculatorInput>) -> Option<i32> {
    // let mut stack = Stack::<CalculatorInput>::new();
    let mut stack = inputs.clone();
    for (index, &input) in inputs.iter().enumerate() {
        if is_operator(input).is_err() {
            stack.push(input);
        } else {
            // match is_operator(input).unwrap() {
            //     CalculatorInput::Add => add(stack[index - 2], stack[index - 1]),
            //     CalculatorInput::Subtract => subtract(stack[index - 2], stack[index - 1]),
            //     CalculatorInput::Multiply => multiply(stack[index - 2], stack[index - 1]),
            //     CalculatorInput::Divide => divide(stack[index - 2], stack[index - 1]),
            // }
            println!("stack[index-2]: {:#?}", stack[index - 2]);
        }
    }
    println!("stack: {:#?}", stack);
    None
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subtract(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn divide(num1: i32, num2: i32) -> i32 {
    num1 / num2
}

fn is_operator(input: CalculatorInput) -> Result<CalculatorInput, bool> {
    if matches!(input, CalculatorInput::Add)
        || matches!(input, CalculatorInput::Subtract)
        || matches!(input, CalculatorInput::Multiply)
        || matches!(input, CalculatorInput::Divide)
    {
        Ok(input)
    } else {
        Err(false)
    }
}

// fn is_a_value(input: CalculatorInput) -> bool {
//     // println!("input: {:#?}", input);
//     matches!(input, CalculatorInput::Value(_i32))
// }

#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}
