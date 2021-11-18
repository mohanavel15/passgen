use rand::Rng;
use regex::Regex;

pub struct PasswordGenerator {
    length: i32,
    alphabets: bool,
    numbers: bool,
    symbols: bool,
    caps: bool,
}

impl PasswordGenerator {
    pub fn new(length: i32, alphabets: bool, numbers: bool, symbols: bool, caps: bool) -> PasswordGenerator {
        PasswordGenerator {
            length: length,
            alphabets: alphabets,
            numbers: numbers,
            symbols: symbols,
            caps: caps,
        }
    }

    pub fn generate(&self) -> String {
        let mut password = String::from("");
        let mut strongpasswd = false;
        while strongpasswd == false {
            password = self.generate_password();
            if self.password_check(password.clone()) == true {
                strongpasswd = true;
            }
        }
        password
    }

    fn generate_password(&self) -> String {
        let characters = self.get_characters();
        let mut password = String::from("");
        for _ in 0..self.length {
            let character = self.get_character(characters.clone());
            password.push(character);
        }
        password
    }

    fn get_character(&self, characters: String) -> char {
        characters.chars().nth(rand::thread_rng().gen_range(0..characters.len())).unwrap()
    }

    fn get_characters(&self) -> String {
        let alphabets = "abcdefghijklmnopqrstuvwxyz";
        let numbers   = "0123456789";
        let symbols   = "*&^%$#@!~-";

        let mut characters = String::from("");
        if self.alphabets == true {
            characters.push_str(alphabets);
            if self.caps == true { characters.push_str(&alphabets.to_uppercase()) };
        };
        
        if self.numbers == true { characters.push_str(numbers) };
        if self.symbols == true { characters.push_str(symbols) };

        characters
    }

    fn password_check(&self, password: String) -> bool {
        if self.alphabets == true && Regex::new(r"[a-z]").unwrap().is_match(&password) == false {
            false
        }

        else if self.alphabets == true && self.caps == true && Regex::new(r"[A-Z]").unwrap().is_match(&password) == false {
            false
        }

        else if self.numbers == true && Regex::new(r"[0-9]").unwrap().is_match(&password) == false {
            false
        }

        else if self.symbols == true && Regex::new(r"[*&^%$#@!~-]").unwrap().is_match(&password) == false {
            false
        }
        
        else { 
            true
        }
    }
}