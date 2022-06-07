use rpn_calculator::{evaluate, CalculatorInput};

fn main() {
    let inputs = calculator_input("4 8 + 7 5 - /");
    evaluate(&inputs);
}

fn calculator_input(s: &str) -> Vec<CalculatorInput> {
    s.split_whitespace()
        .map(|s| match s {
            "+" => CalculatorInput::Add,
            "-" => CalculatorInput::Subtract,
            "*" => CalculatorInput::Multiply,
            "/" => CalculatorInput::Divide,
            n => CalculatorInput::Value(n.parse().unwrap()),
        })
        .collect()
}
