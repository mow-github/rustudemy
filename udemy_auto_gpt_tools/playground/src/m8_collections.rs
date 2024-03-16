#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod test {
    use super::*;

    // cargo test tests_hashmap -- --nocapture
    #[test]
    fn tests_hashmap() {
        dbg!("tests_hashmap");

        // &str -> Person
        // u8 -> &str
        // &str -> u32

        let person_1 = "alice";
        let person_2 = "bob";

        let mut results_hm: HashMap<&str, u32> = HashMap::new();
        results_hm.insert(person_1, 55);
        results_hm.insert(person_2, 54);

        let test_score: Option<&u32> = results_hm.get(person_1);
        let d = test_score.unwrap();

        println!("{}", d);

        if results_hm.contains_key("bob") {
            dbg!("bob is here");
        }
    }    

    // cargo test tests_hashset -- --nocapture
    #[test]
    fn tests_hashset() {
        dbg!("tests_hashset");

        let mut results_hs: HashSet<&str> = HashSet::new();

        results_hs.insert("alice");
        results_hs.insert("bob");

        if results_hs.contains("bob") {
            dbg!("bob is here");
        }
    }     

  

}
