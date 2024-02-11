/*
    You are tasked with implementing a Rust program that performs various operations on an array of numbers. Your program should include functions for the following operations:

    Sum of Elements:
        Write a function sum_of_elements that takes an array of integers as input and returns the sum of all elements.

    Maximum Element:
        Write a function max_element that takes an array of integers as input and returns the maximum element.

    Minimum Element:
        Write a function min_element that takes an array of integers as input and returns the minimum element.

    Average of Elements:
        Write a function average_of_elements that takes an array of integers as input and returns the average of all elements.

    - The array will contain at least one element.
    - The array will only contain integers.        
*/

// use std::iter::Sum;
 
fn main() {

    println!("\n## Program: START ## \n");

    // Test your functions with a sample array

    let numbers = [10, 5, 8, 12, 7];

    let sum = sum_of_elements(&numbers);

    let max = max_element(&numbers);

    let min = min_element(&numbers);

    let average = average_of_elements(&numbers);

    println!("Sum: {}", sum);

    println!("Max: {}", max);

    println!("Min: {}", min);

    println!("Average: {}", average);    


    println!("\n## Program: END ## \n");

}

fn sum_of_elements(arr: &[i32]) -> i32 {
    
    // alt 1:
    // arr.iter().sum()

    // alt 2:
    //
    let mut sum = 0;
    for value in arr.iter() {
        sum += value;
    }
    sum    
}



fn max_element(arr: &[i32]) -> i32 {

    // alt 1:
    // arr.iter().copied().max().unwrap() 

    // alt 2:
    //
    let mut max = arr[0];
    for value in arr.iter().copied() {
        if max < value {
            max = value
        }
    }
    max
}



fn min_element(arr: &[i32]) -> i32 {

    // alt 1:
    // arr.iter().copied().min().unwrap()

    // alt 2:
    //
    let mut min = arr[0];
    for value in arr.iter().copied() {
        if min > value {
            min = value
        }
    }
    min
}



fn average_of_elements(arr: &[i32]) -> f64 {

    // alt 1:
    //
    // f64::sum(arr.iter().copied().map(f64::from)) / arr.len() as f64

    // alt 2:
    //
    let mut sum = 0;
    for value in arr.iter() {
        sum += value;
    }

    sum as f64 / arr.len() as f64
}