use std::io::{self, Write}; // Import necessary modules for input/output handling

fn main() {
    loop {
        let mut input_line = String::new(); // Create a new string to hold user input

        // Prompt the user for the first integer (x)
        print!("Enter the value of x: ");
        io::stdout().flush().expect("Failed to flush stdout."); // Ensure the prompt is displayed immediately
        io::stdin().read_line(&mut input_line).expect("Failed to read input."); // Read user input
        let x: i32 = input_line.trim().parse().expect("Input not an integer."); // Parse input to integer

        input_line.clear(); // Clear the input buffer for the next input

        // Prompt the user for the second integer (y)
        print!("Enter the value of y: ");
        io::stdout().flush().expect("Failed to flush stdout."); // Ensure the prompt is displayed immediately
        io::stdin().read_line(&mut input_line).expect("Failed to read input."); // Read user input
        let y: i32 = input_line.trim().parse().expect("Input not an integer."); // Parse input to integer

        input_line.clear(); // Clear the input buffer again for the next input

        // Display the menu of operations to the user
        println!("1. Add\n2. Difference\n3. Divide\n4. Multiply\n5. Exit");
        io::stdin().read_line(&mut input_line).expect("Failed to read input."); // Read user choice
        let choice: i32 = input_line.trim().parse().expect("Input not an integer."); // Parse choice to integer

        // Perform the selected operation based on user input
        match choice {
            1 => println!("Sum of {x} and {y}: {}", x + y), // Addition
            2 => println!("Difference of {x} and {y}: {}", x - y), // Subtraction
            3 => { 
                // Division with a check for division by zero
                if y != 0 {
                    println!("Division of {x} by {y}: {}", x / y); // Division
                } else {
                    println!("Cannot divide by zero."); // Error message for division by zero
                }
            }
            4 => println!("Multiply of {x} and {y}: {}", x * y), // Multiplication
            5 => break, // Exit the loop if user chooses to exit
            _ => println!("Invalid choice. Please try again."), // Handle invalid choices
        }
    }
}
