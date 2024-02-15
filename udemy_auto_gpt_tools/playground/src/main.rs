// set mod-statments
//
// mod my_funcs;
// mod my_mod_folder;

// set use-statments
//
// use my_funcs::add_five;
// use crate::my_funcs::add_five;
// use crate::my_funcs::{add_five, subtract_five};
// use my_mod_folder::my_funcs::add_ten;



// -------- m1_enums START

mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetimes;
mod m6_patterns;
mod m7_async;
mod m8_collections;

// use m1_enums::

use m5_lifetimes::example_1_v3;

// -------
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let x: u32 = 50;
    // println!("x: {}", x);

    // let y = add_five(x);
    // println!("y: {}", y);

    // let z = subtract_five(x);
    // println!("z: {}", z);    

    // let q = add_ten(x);
    // println!("q: {}", q);

    example_1_v3(); 
}
