// lifetime and references...pointing to a thing that goes out of scope... lifetime error..
//
// #[allow(dead_code, unused_variables)]
// fn example_0() {
//     let r: &i32;

//     {
//         let x: i32 = 5;
//         r = &x;
//     }

//     println!("r {}", r);
// }

#[allow(dead_code, unused_variables)]
fn example_1() {
    let highest_age: i32;

    let alice_age: i32 = 20;    // 'a
    let bob_age: i32 = 21;      // 'b       OR:     'b: 'a

    highest_age = largest(&alice_age, &bob_age);

    println!("highest_age {}", highest_age);

    fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1
        } else {
            *compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_1_v2() {
    let highest_age: &i32;

    let alice_age: i32 = 20;    // 'a
    let bob_age: i32 = 21;      // 'b       OR:     'b: 'a

    highest_age = largest(&alice_age, &bob_age);

    println!("highest_age {}", highest_age);

    // example with generic lifetime 'a...args start at same scope aka. same lifetime..
    fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
pub fn example_1_v3() {
    let highest_age: &i32;
    let new_value: i32;

    let alice_age: i32 = 20;    // 'a
    
    {
        let bob_age: i32 = 21;      // 'b       NOTE: 'b goes out of scope after }... NOT SAME LIFETIME as 'a...

        highest_age = largest(&alice_age, &bob_age);
        // set new_value..thus avoid problem with out of scope
        new_value = *highest_age;
    }

    println!("highest_age {}", new_value);

    // ..now bobs var lifetime is created later...and disposed before 'a
    // fn takes normally the shortest lifetime of the args yes ?
    fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
pub fn example_1_v4() {
    let highest_age: &i32;
    let new_value: i32;

    let alice_age: i32 = 20;    // 'a
    
    {
        let bob_age: i32 = 21;      // 'b       NOTE: 'b goes out of scope after }... NOT SAME LIFETIME as 'a...

        highest_age = largest::<i32>(&alice_age, &bob_age);
        new_value = *highest_age;
    }

    println!("highest_age {}", new_value);

    // generic method and lifetime
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

// examplw struct with a lifetime incl fn below
struct Person<'p> {
    name: &'p str,
    points: &'p f32
}

#[allow(dead_code, unused_variables)]
pub fn example_1_v5() {
    let highest_age: &f32;
    let new_value: f32;

    let alice = Person{ name: "alice", points: &50.2};
    
    {
        let bob = Person{ name: "bob", points: &40.5};

        highest_age = largest::<f32>(alice.points, bob.points);
        new_value = *highest_age;
    }

    println!("highest_age {}", new_value);

    // generic method and lifetime
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     // cargo test tests_lifetimes -- --nocapture

//     #[test]
//     fn tests_lifetimes(){
//         dbg!("tests_lifetimes");

//     }
// }
