/*
    Write a Rust program that calculates the sum of the digits of a given positive integer.
    The sum of digits is the result of adding up all individual digits of the number.

    Your task is to implement a function sum_of_digits
    that takes a positive integer as input and
    returns the sum of its digits.

    - The input number will be a positive integer.
    - You should use a loop to calculate the sum of digits.
    - You are not allowed to convert the number to a string for this calculation.    
*/

fn main() {

    println!("\n## Program: START ## \n");

    // let number: i32 = -1;
    // let number: i32 = 0;
    // let number: i32 = 482;
    let number: i32 = 12345;

    let result = sum_of_digits(number);

    println!("\nresult: {}", result);


    println!("\n## Program: END ## \n");

}

fn sum_of_digits(num: i32) -> i32 {

    if num < 0 {
        println!("not a positive value..return 0");
        return 0; 
    } else if num < 10 && num >= 0 {
        println!("single digit.. return value as sum"); 
        return num;
    } else {
        let mut sum = 0;
        let mut my_number_not_rest = num;
        while my_number_not_rest != 0 {

            let my_number_rest = my_number_not_rest % 10;
            println!("rest_value {}", my_number_rest); 

            sum += my_number_rest;
    
            my_number_not_rest /= 10;
            println!("my_number_not_rest: \t\t{}\n", my_number_not_rest);
        }

        return sum;
    }

}



