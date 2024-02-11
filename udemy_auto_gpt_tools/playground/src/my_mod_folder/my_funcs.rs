

pub fn add_ten(num: u32) -> u32 {
    num + 10
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_ten_test(){

        /*
            cargo test                      // run tests
            cargo test -- --nocapture       // run test and also output println-statments
            cargo test adds_ten_test        // run test with name: <name>

            RUST_BACKTRACE=1 cargo test     // run test with a stacktrace 
                - NOTE: $env:RUST_BACKTRACE=1 ..if using pwsh
        */


        let x: u32 = 100;
        let y = add_ten(x);
        print!("x and y are from test: {} {}", x, y);
        assert_eq!(y, 111)
    }
}