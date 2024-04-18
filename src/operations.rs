use crate::utils;

pub fn add(current_number:i32)->i32{
    let num2 = utils::get_input_number();
    current_number + num2
}

pub fn sub(current_number:i32)->i32{
    let num2 = utils::get_input_number();;
    current_number - num2
}

pub fn mul(current_number:i32)->i32{
    let num2 = utils::get_input_number();;
    current_number * num2
}

pub fn div(current_number:i32)->i32{
    let num2 = utils::get_input_number();;
    if num2 == 0 {
        println!("Cannot divide by zero Clearing the current number");
        return div(current_number);
    }
    current_number / num2
}