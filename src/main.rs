use std::env::{args, Args};

fn operate(operator: &str, first: f32, second: f32) -> f32 {
    match operator {
        "+" => first + second,
        "-" => first - second,
        "*" | "X" | "x" => first * second,
        "/" => first / second,
        _ => panic!("Invalid operator"),
    }
}

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(&operator, first_number, second_number);

    println!("{} {} {}", first_number, operator, second_number)
}
