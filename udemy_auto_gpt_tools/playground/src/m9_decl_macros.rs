//
//
//
// NOTE: u need "#[macro_export]" to export a macro and use in other files.. 
//
//
// Declarative macros   - etc. "mad_skills"-macro
// Procedural Macros    - allow creating syntax extensions as execution of a function. 3x variations: function, derive, attribute
//

#[cfg(test)]
mod test {


    macro_rules! mad_skills {
        ($x: expr) => {
            format!("u sent an expression: {}", $x)
        };
    }

    // macro_rules! mad_skills2 {
    //     ($x: ty) => {
    //         match stringify($x) {
    //             "i32" => "u sent an i32".to_string(),
    //             _ => "u sent something else".to_string(),
    //         }
    //     };
    // }    

    // vec min 1 unit inside..
    macro_rules! my_vec {
        ( $($x: expr), + ) => {
            {
                let mut temp_vec = Vec::new();

                $(
                    temp_vec.push($x);
                )+
                temp_vec
            }
        };
    }   

    // cargo test tests_decl_macros -- --nocapture
    #[test]
    fn tests_decl_macros() {
        dbg!("tests_decl_macros");

        println!("Hello 1");
        dbg!("Hello 2");
        let x: Vec<i32> = vec!(1, 2, 3);
        let formatted: String = format!("Hello 3 with vec {:?}", x);
        dbg!(formatted);    

    }    

    // cargo test tests_decl_macros_1 -- --nocapture
    #[test]
    fn tests_decl_macros_1() {
        dbg!("tests_decl_macros_1");

        let some_var = mad_skills!(1 + 2);
        dbg!(some_var);    

        // let some_var = mad_skills2!(i32);
        // dbg!(some_var);        

        let _x: Vec<i32> = vec!();
        let _y: Vec<i32> = my_vec!(1);    

    }    
}
