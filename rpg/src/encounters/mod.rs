use rand::Rng;
use std::process;

//mod hero;
use crate::hero;

pub fn encounter_goblin(main_character: &mut hero::Hero)
{
   let mut choice = hero::add_choice(String::from("You move to the next room and you encounter a goblin. What do you do?\n 1: try to flee 2: fight 3: surrender"));
   if choice == 1
   {
      println!("You run into the shadows, you successfully avoided the goblin. However in you hurry you hit a wall and lose 1 life");
      main_character.life = hero::lose_health(main_character.life, 1);
   }
   else if choice == 2
   {
      if main_character.weapon == 1 || main_character.weapon == 2
      {
         println!("You have defeated the Goblin and continue your adventure.");
         println!("You search the pockets of the body of you ennemy and find 2 gold");
         main_character.money = hero::add_gold(main_character.money, 2);
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

pub fn encounter_zombie(main_character: &mut hero::Hero)
{
   let mut choice = hero::add_choice(String::from("You move to the next room and you encounter a goblin. What do you do?\n 1: try to flee 2: fight 3: surrender"));
   if choice == 1
   {
      println!("You run into the shadows, you successfully avoided the goblin. However in you hurry you hit a wall and lose 1 life");
      main_character.life = hero::lose_health(main_character.life, 1);
   }
   else if choice == 2
   {
      if main_character.weapon == 1
      {
         println!("You have defeated the Zombie and continue your adventure.");
         println!("You search the pockets of the body of you ennemy and find 2 gold");
         main_character.money = hero::add_gold(main_character.money, 2);
      }
      else if main_character.weapon == 2
      {
         println!("The hammer is really heavy and you kill the zombie. However, you were so slow that it bite you and you lose one life");
         println!("You search the pockets of the body of you ennemy and find 2 gold");
         main_character.money = hero::add_gold(main_character.money, 2);
         main_character.life = hero::lose_health(main_character.life, 1);
         process::exit(0);
      }
   }
   else
   {
      println!("Really? Surrendering, you died and deserved it.");
      process::exit(0);
   }
}
pub fn encounter_troll(main_character: &mut hero::Hero)
{
   let mut choice = hero::add_choice(String::from("You move to the next room and you encounter a troll. What do you do?\n 1: try to flee 2: fight 3: surrender"));
   if choice == 1
   {
      println!("You run into the shadows, you successfully avoided the goblin. However in you hurry you hit a wall and lose 1 life");
      main_character.life = hero::lose_health(main_character.life, 1);
   }
   else if choice == 2
   {
      if main_character.weapon == 2
      {
         println!("You have defeated the troll and continue your adventure.");
         println!("You search the pockets of the body of you ennemy and find 2 gold");
         main_character.money = hero::add_gold(main_character.money, 2);
      }
      if main_character.weapon != 2
      {
         println!("You don't have a Hammer and can't deal any damages to the troll. While you didge an attack you get hit and lose one life. You then run and escape");
         main_character.life = hero::lose_health(main_character.life, 1);
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
pub fn monster_encounter(main_character: &mut hero::Hero)
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

pub fn random_encounter(main_character: &mut hero::Hero)
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

