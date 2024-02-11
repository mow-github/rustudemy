/*
    Write a Rust program that takes daily temperature inputs for a week and calculates the average temperature for that week.      

    The program should take daily temperature inputs for each day of the week (e.g., Monday to Sunday).
    The temperatures can be integers or floating-point numbers.    

    The program should calculate and print the average temperature for the week.

    - Efficient use of loops and arrays to handle daily temperature inputs.
    - Proper handling of user input and error cases.
*/

use std::io;

const DAYS_IN_WEEK: usize = 7;



fn main() {

    println!("\n## Program: START ## \n");

    let mut temperatures = [0.0; DAYS_IN_WEEK]; // Array to store daily temperatures



    // Input daily temperatures

    for i in 0..DAYS_IN_WEEK {

        println!("Enter temperature for day {}: ", i + 1);

        temperatures[i] = get_user_input(i + 1);

        println!("You entered {} for day: {} \n", temperatures[i], i + 1);
    }



    // Calculate average temperature

    let average_temperature = calculate_average_temperature(&temperatures);



    // Output result

    println!("Average temperature for the week: {:.2}", average_temperature);

    println!("\n## Program: END ## \n");
}



fn get_user_input(day_nr: usize) -> f64 {
    loop {
        let mut input_text = String::new();

        if io::stdin().read_line(&mut input_text).is_err() {
            println!("\tFailed to read line for day: {}. Please try again.", day_nr);
            continue;
        }

        match input_text.trim().parse() {
            Ok(number) => return number,
            Err(_) => {
                println!("\tInvalid input for day: {}. Please enter a valid floating-point number.", day_nr);
                continue;
            }
        }
    }
}



fn calculate_average_temperature(temperatures: &[f64; DAYS_IN_WEEK]) -> f64 {

    let mut tot = 0 as f64;
    for &number in temperatures {
        println!("{}", number);
        tot += &number;
    }
    
    tot / temperatures.len() as f64
}