use std::fmt;
use std::fmt::Display;
use std::io;
use std::process;
//use crate::encounters;
//use crate::hero;

mod encounters;
mod hero;

/******************************************************************************
 * main function
 *****************************************************************************/

fn main()  {
    
   let mut main_character = hero::Hero { name: String::from("Knight"), life: 3, damages: 0, weapon: 0, money: 0};
   let mut choice :i32 = 0;
   //println!("The hero : {}", main_character);
   choice = hero::add_choice(String::from("Grab the sword or Hammer ?\n 1: sword 2: hammer 3: leave all weapons"));
   
   if choice == 1 
   {
      main_character.damages = 3;
      main_character.weapon = 1;
      println!("You take the sword and feel more confident to face the dangers that lurks in the dark");
   } 
   else if choice == 2 
   {
      main_character.damages = 3;
      main_character.weapon = 2;
      println!("You take the hammer, it seems heavy and slow but very strong and sturdy too");
      main_character.damages = 0;
   } 
   else if choice == 3 
   {
      println!("You leave the sword and hammer behind, you prefer fighting naked!");
      main_character.damages = 0;
   } 
   else 
   {
      println!("Error");
      process::exit(1);
   }


   // First part of dungeon 
   for x in 0..9 
   {
      encounters::random_encounter(&mut main_character);
   }
   
}

