use std::fmt;
use std::fmt::Display;
use std::io;
use std::process;

/******************************************************************************
 *                                   Structure Hero
 ****************************************************************************/
pub struct Hero {
    pub name : String,
    pub life : u8,
    pub damages : u8,
    pub weapon: u8,
    pub money: u8
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
pub fn add_choice(text: String) -> i32 {
   
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
pub fn add_health(health: u8, points: u8) -> u8 {
   return health + points;
}

pub fn add_gold(gold: u8, points: u8) -> u8 {
    return gold + points;
}
/******************************************************************************
 * fn lose_health(health: u8, points: u8) ->
 * \brief function that decrease player health and checks if he/she is not dead
 * @health : the total health point before the damages are dealt
 * @points : the damages to be dealt
 * @returns : the remaining health points if the player is not dead
 ****************************************************************************/
pub fn lose_health(health: u8, points: u8) -> u8 {
   if health - points <= 0
   {
      println!("You died and your body is left behind to be eaten by the rats.");
      process::exit(0);
   }
   return health - points;
}

pub fn lose_gold(gold: u8, points: u8) -> u8 {
   let mut res: u8 = gold - points;
   if res <= 0
   {
       res = 0;
   }
   return res;
}
