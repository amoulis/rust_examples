use std::fmt;
use std::fmt::Display;
use std::io;
use std::process;
use rand::Rng;



/******************************************************************************
 *                                   Structure Hero
 ****************************************************************************/
struct Hero {
    name : String,
    life : u8,
    damages : u8,
    weapon: u8,
    money: u8
}

impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name : {}, Life : {}, Damages : {} weapon: {}, money: {}", self.name, self.life, self.damages, self.weapon, self.money)
    }
}

/******************************************************************************
 * fn add_choice(text: String) -> i32
 * \brief function that returns usr choice
 * @text : the text to display explaining what happends in the game
 * @returns : the choice
 ****************************************************************************/
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

/******************************************************************************
 * fn add_health(health: u8, points: u8) ->
 * \brief function that decrease player health and checks if he/she is not dead
 * @health : the total health point before the healing
 * @points : the number of points to heal
 * @returns : the remaining health points
 ****************************************************************************/
fn add_health(health: u8, points: u8) -> u8 {
   return health + points;
}

fn add_gold(gold: u8, points: u8) -> u8 {
    return gold + points;
}
/******************************************************************************
 * fn lose_health(health: u8, points: u8) ->
 * \brief function that decrease player health and checks if he/she is not dead
 * @health : the total health point before the damages are dealt
 * @points : the damages to be dealt
 * @returns : the remaining health points if the player is not dead
 ****************************************************************************/
fn lose_health(health: u8, points: u8) -> u8 {
   if health - points <= 0
   {
      println!("You died and your body is left behind to be eaten by the rats.");
      process::exit(0);
   }
   return health - points;
}

fn lose_gold(gold: u8, points: u8) -> u8 {
   let mut res: u8 = gold - points;
   if res <= 0
   {
       res = 0;
   }
   return res;
}
fn encounter_goblin(main_character: &mut Hero)
{
   let mut choice = add_choice(String::from("You move to the next room and you encounter a goblin. What do you do?\n 1: try to flee 2: fight 3: surrender"));
   if choice == 1
   {
      println!("You run into the shadows, you successfully avoided the goblin. However in you hurry you hit a wall and lose 1 life");
      main_character.life = lose_health(main_character.life, 1);
   }
   else if choice == 2
   {
      if main_character.damages >= 3
      {
         println!("You have defeated the Goblin and continue your adventure.");
         println!("You search the pockets of the body of you ennemy and find 2 gold");
         main_character.money = add_gold(main_character.money, 2);
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

fn encounter_zombie(main_character: &mut Hero)
{
   let mut choice = add_choice(String::from("You move to the next room and you encounter a goblin. What do you do?\n 1: try to flee 2: fight 3: surrender"));
   if choice == 1
   {
      println!("You run into the shadows, you successfully avoided the goblin. However in you hurry you hit a wall and lose 1 life");
      main_character.life = lose_health(main_character.life, 1);
   }
   else if choice == 2
   {
      if main_character.weapon == 1
      {
         println!("You have defeated the Zombie and continue your adventure.");
         println!("You search the pockets of the body of you ennemy and find 2 gold");
         main_character.money = add_gold(main_character.money, 2);
      }
      else if main_character.weapon == 2
      {
         println!("The hammer is really heavy and you kill the zombie. However, you were so slow that it bite you and you lose one life");
         println!("You search the pockets of the body of you ennemy and find 2 gold");
         main_character.money = add_gold(main_character.money, 2);
         main_character.life = lose_health(main_character.life, 1);
         process::exit(0);
      }
   }
   else
   {
      println!("Really? Surrendering, you died and deserved it.");
      process::exit(0);
   }
}
fn encounter_troll(main_character: &mut Hero)
{
   let mut choice = add_choice(String::from("You move to the next room and you encounter a troll. What do you do?\n 1: try to flee 2: fight 3: surrender"));
   if choice == 1
   {
      println!("You run into the shadows, you successfully avoided the goblin. However in you hurry you hit a wall and lose 1 life");
      main_character.life = lose_health(main_character.life, 1);
   }
   else if choice == 2
   {
      if main_character.weapon == 2
      {
         println!("You have defeated the troll and continue your adventure.");
         println!("You search the pockets of the body of you ennemy and find 2 gold");
         main_character.money = add_gold(main_character.money, 2);
      }
      if main_character.weapon != 2
      {
         println!("You don't have a Hammer and can't deal any damages to the troll. While you didge an attack you get hit and lose one life. You then run and escape");
         main_character.life = lose_health(main_character.life, 1);
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
fn monster_encounter(main_character: &mut Hero)
{
   let mut rng = rand::thread_rng();
   let encounter = rng.gen_range(1..6);

   if encounter >= 1 && encounter < 4 // Goblin
   {
      encounter_goblin(main_character);
   }
   else if encounter >= 4 && encounter <= 5 // Zombies
   {

   }
   else if encounter == 6 // Troll
   {
      encounter_troll(main_character);
   }
   else
   {
      println!("Error : gen monster");
   }
}

fn random_encounter(main_character: &mut Hero)
{
   let mut rng = rand::thread_rng();
   let encounter = rng.gen_range(1..10);

   if  encounter <= 5 && encounter >= 1  // Monster
   {
      monster_encounter(main_character);
   }
   else if encounter > 5 && encounter <= 7 // merchant
   {

      println!("TODO : implement merchant");
   }
   else if encounter > 7 && encounter <= 9 // trap
   {

      println!("TODO : implement trap");
   }
   else if encounter  == 10 // heal shrine
   {

      println!("TODO : implement heal shrine");
   }
   else 
   {
      println!("Error : gen encounter nb: {}", encounter);
   }

}

/******************************************************************************
 * main function
 *****************************************************************************/

fn main()  {
    
   let mut main_character = Hero { name: String::from("Knight"), life: 3, damages: 0, weapon: 0, money: 0};
   let mut choice :i32 = 0;
   //println!("The hero : {}", main_character);
   choice = add_choice(String::from("Grab the sword or Hammer ?\n 1: sword 2: hammer 3: leave all weapons"));
   
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

   // First part of dungeon 
   for x in 0..9 
   {
      random_encounter(&mut main_character);
   }
   
}

