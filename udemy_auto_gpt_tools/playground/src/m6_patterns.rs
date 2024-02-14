#[derive(Debug)]
enum Message {
    Quit,
    ChangeColour(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("i quit")
        }
        Message::ChangeColour(red, green, blue) => {
            println!("{} {} {}", red, green, blue)
        }
        Message::Move { x, y: new_value } => {
            println!("{} {}", x, new_value)
        }
        Message::Write(text) => {
            println!("{}", text)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    // cargo test m6_patterns -- --nocapture

    #[test]
    fn m6_patterns() {
        dbg!("m6_patterns");

        let number: i32 = 20;

        let res: &str = match number {
            1 => "This is the first!",
            _ => "other...",
        };

        println!("{}", res)
    }

    // cargo test m6_match_option -- --nocapture
    #[test]
    fn m6_match_option() {
        // let some_num: Option<i32> = Some(10);
        // let probe_none: Option<i32> = None;

        // let res = match some_num {
        //     Some(i) => i,
        //     None => {
        //         panic!("err fix me");
        //     }
        // };

        // println!("{:?}", some_num);
        // println!("{}", res);

        // ---

        // let some_num: Option<i32> = Some(10);

        // let my_int = if let Some(i) = some_num {
        //     // println!("{}", i);
        //     i
        // } else {
        //     panic!("err again");
        // };

        // println!("{}", my_int);

        // ---

        // ---

        let some_res: Result<i32, &str> = Ok(50);
        let some_err: Result<i32, &str> = Err("err here");

        let my_int = match some_err {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };

        println!("{}", my_int);

        // ---
    }


    // cargo test test_large_enum -- --nocapture
    #[test]
    fn test_large_enum() {

        let my_quit: Message = Message::Quit;
        process_message(my_quit);

        let my_colour: Message = Message::ChangeColour(10, 22, 44);
        process_message(my_colour);        

    }


    // cargo test test_match_guard -- --nocapture
    #[test]
    fn test_match_guard() {

        let pair: (i32, i32) = (2, -2);
        match pair {
            (x, y) if x == y => println!("they match!"),
            (x, y) if x + y == 0 => println!("they zero"),
            (_, y) if y == 2 => println!("Y is indeed +2"),
            _ => println!("we are no bothered")
        };
    }    

    // cargo test test_match_struct -- --nocapture
    #[test]
    fn test_match_struct() {

        struct Location {
            x: i32,
            y: i32
        }

        let location = Location{x: 0, y: 20};

        match location {
            Location{x, y: 0} => println!("y is on the axis"),
            Location{x: 0, y} => println!("x is on the axis"),
            Location{x, y} => println!("none is on the axis"),
        };

        
    }       

}
