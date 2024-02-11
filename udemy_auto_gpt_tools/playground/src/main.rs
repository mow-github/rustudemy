// set mod-statments
//
mod my_funcs;
mod my_mod_folder;

// set use-statments
//
// use my_funcs::add_five;
// use crate::my_funcs::add_five;
use crate::my_funcs::{add_five, subtract_five};
use my_mod_folder::my_funcs::add_ten;

fn main() {
    println!("Hello, world!");

    let x: u32 = 50;
    println!("x: {}", x);

    let y = add_five(x);
    println!("y: {}", y);

    let z = subtract_five(x);
    println!("z: {}", z);    

    let q = add_ten(x);
    println!("q: {}", q); 
}
