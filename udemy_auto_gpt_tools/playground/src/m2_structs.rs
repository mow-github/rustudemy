
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

impl User {
    fn increment_sign_count(&mut self, nr: u64){
        self.sign_in_count += nr;    
    }

    fn change_email(&mut self){
        self.email = "new email set".to_string();    
    }
}

fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}

#[cfg(test)]
mod test {
    use super::*;

    // cargo test tests_structs -- --nocapture

    #[test]
    fn tests_structs(){
        dbg!("Hiirrrrrrrrrr");

        let mut user_1 = User{
            username: String::from("username1"),
            email: String::from("email1"),
            active: true,
            sign_in_count: 1
        };

        user_1.username = "annother username".to_string();

        change_username(&mut user_1, "annother username AGAIN");

        dbg!(user_1);

        let mut user_2 = User{
            username: String::from("username2"),
            email: String::from("email2"),
            active: false,
            sign_in_count: 1
        };
        dbg!(&user_2);

        user_2.change_email();
        user_2.increment_sign_count(4);

        dbg!(&user_2);


    }
}