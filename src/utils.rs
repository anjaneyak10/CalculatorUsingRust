use std::io;

pub fn get_input_number() -> i32 {
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Please type a number!");
    return input;
}

pub fn get_input_string() -> String {
    println!("Enter operation: ");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    return operation;
}