use std::env::{args, Args};

fn operate(operator: &str, first_number: f32, second_numer: f32) -> f32 {
    match operator {
        "+" => first_number + second_numer,
        "-" => first_number - second_numer,
        "*" | "X" | "x" => first_number * second_numer,
        "/" => first_number / second_numer,
        _ => panic!("Invalid operator"),
    }
}

fn output(operator: &str, first_number: f32, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
      )
}

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(&operator, first_number, second_number);

    println!("{}", output(&operator, first_number, second_number, result));
}
