

#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E)
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T)
}

fn create_car_colour_blue() -> CarColour {
    let my_car_colour: CarColour = CarColour::Blue;
    my_car_colour
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Err...Bad boy".to_string())
    }
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

fn remainder_zero_builtin(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Option::Some(remainder)
    } else {
        Option::None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // cargo test tests_enums -- --nocapture

    #[test]
    fn tests_enums(){
        // dbg!("Hii");

        let car_colour = create_car_colour_blue();
        dbg!(car_colour);

        let under_five_rs = check_under_five(3);
        dbg!(under_five_rs);

        let remainder = remainder_zero(12.2);
        dbg!(remainder);

        let remainder_builtin = remainder_zero_builtin(2.2);
        let value = remainder_builtin.unwrap();

    }
}