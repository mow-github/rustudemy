/*
    Write a Rust program that checks whether a given positive integer is a palindrome or not.
    A palindrome number is a number that reads the same backward as forward.

    Your task is to implement a function is_palindrome_number that
    takes a positive integer as input
    and returns true if it is a palindrome and false otherwise.

    You should solve this without converting the number to a string.
*/

fn main() {

    println!("\n## Palindrom Program: START ## \n");

    // let my_number: i32 = 2;      
    // let my_number: i32 = 22;      
    // let my_number: i32 = 202;      
    let my_number: i32 = 224;      

    let my_flag = is_palindrome_number(my_number);
    if my_flag {
        println!("based on indata: {}..: TRUE", my_number);
    } else {
        println!("based on indata: {}..: FALSE", my_number);
    }
    println!("\n## Palindrom Program: END ## \n");

}

fn is_palindrome_number(my_number: i32) -> bool {

    if my_number < 10 {
        println!("single digits..return true\n");
        return true
    }

    let mut my_number_backward = 0;
    let mut my_number_not_rest = my_number;

    while my_number_not_rest != 0 {

        let my_number_rest = my_number_not_rest % 10;
        println!("my_number_rest: \t\t{}", my_number_rest);

        my_number_backward = my_number_backward * 10 + my_number_rest;
        println!("my_number_backward: \t\t{}", my_number_backward);

        my_number_not_rest /= 10;
        println!("my_number_not_rest: \t\t{}\n", my_number_not_rest);
    }

    println!("compare numbers: {} vs {}\n", my_number, my_number_backward);

    if my_number == my_number_backward {
        return true
    } else {
        return false
    }
}