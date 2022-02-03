use std::fmt;
use std::fmt::Display;
use std::io;
use std::process;

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

fn add_choice(text: String) -> i32 {
   
   let mut guess = String::new();
   let mut choice: i32 = 0;
   println!("{}", text);
   io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
   //choice = guess.parse.trim()::<u32>().wrap();
   choice = guess.trim().parse().expect("Not a string number");
   return choice;
}

fn lose_health(health: u8, points: u8) -> u8 {
   if health - points <= 0
   {
      println!("You died and your body is left behind to be eaten by the rats.");
      process::exit(0);
   }
   return health - points;
}

fn main()  {
    
   let mut main_character = Hero { name: String::from("Knight"), life: 3, damages: 0 };
   let mut choice :i32 = 0;
   //println!("The hero : {}", main_character);
   choice = add_choice(String::from("Grab the sword ?\n 1: yes 2: no"));
   
   if choice == 1 
   {
      main_character.damages = 3;
      println!("You take the sword and feel more confident to face the dangers that lurks in the dark");
   } 
   else if choice == 2 
   {
      println!("You leave the sword behind, you prefer fighting naked!");
      main_character.damages = 0;
   } 
   else 
   {
      println!("Error");
      process::exit(1);
   }


   choice = add_choice(String::from("You move sfitly into the shadows and suddenly your attacked by a goblin. What do you do?\n 1: try to flee 2: fight 3: surrender"));
   if choice == 1
   {
      println!("You run into the shadows, you successfully avoided the goblin. However in you hurry you hit a wall and lose 1 life");
      main_character.life = lose_health(main_character.life, 1);
   }
   else if choice == 2
   {
      if main_character.damages == 3
      {
         println!("You have defeated the Goblin and continue your adventure.");
      }
      else
      {
         println!("You died. Maybe pick the sword next time");
         process::exit(0);
      }
   }
   else
   {
      println!("Really? Surrendering, you died and deserved it.");
      process::exit(0);
   }

   
}

