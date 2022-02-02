use std::fmt;
use std::fmt::Display;
use std::io;

// Structure Hero
struct Hero {
    name : String,
    life : u8,
    damages : u8
}

impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name : {}, Life : {}, Damages : {}", self.name, self.life, self.damages)
    }
}

fn add_choice(text: String) -> u32 {
   
   let mut guess = String::new();
   let mut choice: u32 = 0;
   io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
   choice = guess.parse::<u32>().unwrap();
   return choice;
}

fn main() {
    
   let main_character = Hero { name: String::from("Knight"), life: 3, damages: 0 };
   let mut choice :u32 = 0;
   //println!("The hero : {}", main_character);
   choice = add_choice(String::from("Grab the sword ?\n 1: yes 2: no"));
   
    
    
}
