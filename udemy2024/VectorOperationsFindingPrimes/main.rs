/*
    You are tasked with implementing a Rust program that works with vectors to find prime numbers. Your program should include functions for the following operations: 

    Generate Primes:
        Write a function generate_primes that takes a positive integer n as input and returns a vector containing all prime numbers less than or equal to n.
        Use the Sieve of Eratosthenes algorithm for efficient prime generation.

    Count Primes:
        Write a function count_primes that takes a vector of integers as input and returns the count of prime numbers in the vector.

    Filter Primes:
        Write a function filter_primes that takes a vector of integers as input and returns a new vector containing only the prime numbers from the input vector.

    - The input for generating primes will be a positive integer.
    - The input vector for counting and filtering primes will contain at least one element.
    - Use the Sieve of Eratosthenes algorithm for prime generation.

*/

fn generate_sieve_of_eratosthenes_part1_vec_of_bools(n: u32) -> Vec<bool> {
    let n = n as usize;
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;

    let mut p = 2;
    while p * p <= n {
        if primes[p] {
            let mut i = p * p;
            while i <= n {
                primes[i] = false;
                i += p;
            }
        }
        p += 1;
    }

    primes
}

fn sieve_of_eratosthenes(n: u32) -> Vec<u32> {

    let primes = generate_sieve_of_eratosthenes_part1_vec_of_bools(n);

    let mut prime_numbers = Vec::new();
    for (num, &is_prime) in primes.iter().enumerate() {
        if is_prime {
            prime_numbers.push(num as u32);
        }
    }

    prime_numbers
}

fn sieve_of_eratosthenes_sorted_vec(n: u32, sorted_vec: &Vec<u32>) -> Vec<u32> {

    let primes = generate_sieve_of_eratosthenes_part1_vec_of_bools(n);

    let mut prime_numbers = Vec::new();
    for (num, &is_prime) in primes.iter().enumerate() {
        if is_prime && sorted_vec.contains(&(num as u32)) {
            prime_numbers.push(num as u32);
        }
    }

    prime_numbers
}

fn generate_primes(n: u32) -> Vec<u32> {

    let prime_numbers = sieve_of_eratosthenes(n);

    prime_numbers
}

fn filter_primes(vec: &Vec<u32>) -> Vec<u32> {

    let new_sorted_vector = &new_sorted_vector(vec);
    let prime_numbers = sieve_of_eratosthenes_sorted_vec(
        max_value_in_sorted_vector(new_sorted_vector),
        new_sorted_vector
    );
    
    prime_numbers
}

fn count_primes(vec: &Vec<u32>) -> usize {
    filter_primes(vec).len()
}

fn new_sorted_vector(vec: &Vec<u32>) -> Vec<u32> {
    let mut vec_sorted = vec.clone();
    vec_sorted.sort();
    vec_sorted
}

fn max_value_in_sorted_vector(vec: &Vec<u32>) -> u32 {
    let highest_value = vec.last().copied().unwrap();
    highest_value
}


fn main() {

    println!("\n## Program: START ## \n");

    // Test your functions with sample inputs

    let limit = 20;

    let primes = generate_primes(limit);

    println!("Prime numbers up to {}: {:?}", limit, primes);

    let numbers = vec![10, 5, 8, 12, 7, 17];

    let count = count_primes(&numbers);

    let filtered_primes = filter_primes(&numbers);

    println!("Count of primes in the vector: {}", count);

    println!("Filtered primes: {:?}", filtered_primes);

    println!("\n## Program: END ## \n");
}