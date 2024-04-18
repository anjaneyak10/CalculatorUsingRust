mod operations;
mod utils;

use utils::get_input_number;

fn main() {
    println!("Calculator");
    println!("Enter 'add' for addition");
    println!("Enter 'sub' for subtraction");
    println!("Enter 'mul' for multiplication");
    println!("Enter 'div' for division");
    println!("Enter 'exit' to exit");
    println!("Enter 'clear' to reset the current number");
    println!();

    let mut current_number: i32 = get_input_number();

    println!("Current number: {}", current_number);

    loop {
        let operation = utils::get_input_string().trim().to_lowercase();

        match operation.as_str() {
            "exit" => break,
            "add" => current_number = operations::add(current_number),
            "sub" => current_number = operations::sub(current_number),
            "mul" => current_number = operations::mul(current_number),
            "div" => current_number = operations::div(current_number),
            "clear" => current_number = 0,
            _ => println!("Invalid operation"),
        }

        println!("Current number: {}", current_number);
    }
}
