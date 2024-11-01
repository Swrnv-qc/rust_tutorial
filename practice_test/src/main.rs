use std::io::{self,Write};
mod even_odd;
mod calculator;

fn main() {
    // Program to calculate sum, substraction, divide and multiply 
    // calculator::calculator();
    // Even Odd checker
    let mut input_line = String::new();
    print!("Enter the number:");
    io::stdout().flush().expect("Failed to do a flush of stdout");
    io::stdin().read_line(&mut input_line).expect("Failed to read input.");
    let num :i32 =input_line.trim().parse().expect("The input is not an integer.");
    even_odd::even_odd(num);
}
