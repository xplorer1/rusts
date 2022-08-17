use std::env::{args, Args};

fn main() {
    println!("==============");

    let mut arg: Args = args();
    let first_operand: String = arg.nth(1).unwrap();
    let operator: char = arg.nth(0).unwrap().chars().next().unwrap();
    let second_operand: String = arg.nth(0).unwrap();

    let first_operand = first_operand.parse::<i32>().unwrap();
    let second_operand = second_operand.parse::<i32>().unwrap();
    let result = operate(operator, first_operand, second_operand);

    println!("{}", format_result(result, first_operand, second_operand, operator));
    println!("==============");
}

fn operate(operator: char, first_number: i32, second_number: i32) -> i32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _ => panic!("Give me a valid input you piece of shit.")
    }
}

fn format_result (result: i32, first_number: i32, second_number: i32, operator: char) -> String {

    format!("{} {} {} = {} ", first_number, operator, second_number, result)

}

