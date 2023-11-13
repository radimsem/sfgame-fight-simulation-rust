use rand::{self, Rng};

use crate::character::{Character, Roles, GameClasses};

fn handle_allowed_defending(characters: [&Character; 2]) -> bool {
    let mut allowed_defending = true;

    for character in characters {
        match character.get_class() {
            GameClasses::MAGE => allowed_defending = false,
            _ => ()
        }
    }

    allowed_defending
}

pub struct Game<'a> {
  player: Character<'a>,
  enemy: Character<'a>,
  first_attacks_count: u8,
  second_attacks_count: u8,
  allowed_defending: bool
}

impl<'a> Game<'a> {
  pub fn new(player: Character<'a>, enemy: Character<'a>) -> Self { 
      Self { 
          player, 
          enemy,
          first_attacks_count: 0,
          second_attacks_count: 0,
          allowed_defending: true
      }
  }
}

impl Game<'_> {
  fn attack(&self, attacker: &mut Character, defender: &mut Character) {
      let mut rng = rand::thread_rng();

      // critical damage & luck
      let mut critical_bonus = 1.0;
      let attack_luck = rng.gen_range(1..100);

      if attack_luck <= attacker.get_luck() {
          critical_bonus *= 2.0;
      }

      println!("{} attacking...", attacker.get_name());

      // dealing damage & defending
      let mut dealing_damage = attacker.get_damage() * critical_bonus;
      let mut defended = false;

      if self.allowed_defending {
          let defend_luck = rng.gen_range(1..100);

          if defend_luck <= defender.get_defend_chance() {
              dealing_damage = 0.0;
              defended = true;
          }
      }
    
      defender.set_health(defender.get_health() - dealing_damage);

      match defended {
          true => print!(
            "{} {}!", 
            defender.get_name(),
            match defender.get_class() {
                GameClasses::SCOUT => "evaded",
                GameClasses::WARRIOR => "blocked",
                _ => ""
            } 
          ),
          false => {
            print!("{} dealed damage {}", attacker.get_name(), dealing_damage);
            if critical_bonus > 1.0 {
                print!(" which was critical!");
            }
          }
      }
      println!();

      println!("Defender {}'s health: {}hp", defender.get_name(), defender.get_health());
      println!();

      attacker.set_damage(attacker.get_damage() * 1.25);
  }
}

impl Game<'_> {
  fn end_game(&self, winner: Character, winners_attacks_count: u8) {
      println!("{} won on {} attacks and with {}hp left!", winner.get_name(), winners_attacks_count, winner.get_health());
  }
}

impl Game<'_> {
  fn start_fight(&mut self, mut first_attacker: Character, mut second_attacker: Character) {
    
      while first_attacker.get_health() > 0.0 && second_attacker.get_health() > 0.0 {

          if self.first_attacks_count == self.second_attacks_count {

              self.attack(&mut first_attacker, &mut second_attacker);
              self.first_attacks_count += 1;

          } else if self.first_attacks_count > self.second_attacks_count {

              self.attack(&mut second_attacker, &mut first_attacker);
              self.second_attacks_count += 1;
          }
      }

      if second_attacker.get_health() <= 0.0 {
          self.end_game(first_attacker, self.first_attacks_count);
      } else {
          self.end_game(second_attacker, self.second_attacks_count);
      }
  }
}

impl Game<'_> {
  pub fn start_game(&mut self) {
      self.player.print_stats(Roles::PLAYER);

      println!("VS");
      println!();

      self.enemy.print_stats(Roles::ENEMY);

      println!("Checking if defending will be allowed...");
      self.allowed_defending = handle_allowed_defending([&self.player, &self.enemy]);

      match self.allowed_defending {
          true => println!("Defending is allowed!"),
          false => println!("Someone is Mage, so defending is not allowed!")
      }

      println!("Fight!");
      println!();

      let mut rng = rand::thread_rng();
      let first_attacker_decider = rng.gen_range(0..1);

      if first_attacker_decider == 0 {
          self.start_fight(self.player, self.enemy);
      } else {
          self.start_fight(self.enemy, self.player);
      }
  }
}