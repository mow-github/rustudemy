

trait Attacker {
    fn choose_style(&self) -> String;
    fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
#[allow(dead_code)]
enum Character {
    Warrior,
    Archer,
    Mage,
}


impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "warrStyle".to_string(),
            Character::Archer => "archerStyle".to_string(),
            Character::Mage => "mageStyle".to_string(),
        }
    }

    fn choose_weapon(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // cargo test tests_traits -- --nocapture

    #[test]
    fn tests_traits(){
        dbg!("Hiirrrrrrrrrr");

        let my_character: Character = Character::Archer;
        let style = my_character.choose_style();
        dbg!(style);


    }
}