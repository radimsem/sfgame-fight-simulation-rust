use rand::{self, Rng};

use crate::character::{Character, Roles};

pub struct Game<'a> {
  player: Character<'a>,
  enemy: Character<'a>,
  first_attacks_count: u8,
  second_attacks_count: u8
}

impl<'a> Game<'a> {
  pub fn new(player: Character<'a>, enemy: Character<'a>) -> Self { 
      Self { 
          player, 
          enemy,
          first_attacks_count: 0,
          second_attacks_count: 0
      } 
  }
}

impl Game<'_> {
  fn attack(&self, attacker: &mut Character, defender: &mut Character) {
      let mut rng = rand::thread_rng();

      // critical damage & luck
      let mut critical_bonus = 1.0;
      let attack_luck = rng.gen_range(1..100);

      if attack_luck <= attacker.luck {
          critical_bonus *= 2.0;
      }

      // dealing damage
      let total_damage = attacker.damage * critical_bonus;
      defender.health -= total_damage;

      print!("Attacker {} dealed damage {}", attacker.name, total_damage);

      if critical_bonus > 1.0 {
          print!(" which was critical!");
      }
      println!();

      println!("Defender {}'s health: {}hp", defender.name, defender.health);
      println!();

      attacker.damage *= 1.25;
  }
}

impl Game<'_> {
  fn end_game(&self, winner: Character, winners_attacks_count: u8) {
      println!("{} won on {} attacks and with {}hp left!", winner.name, winners_attacks_count, winner.health);
  }
}

impl Game<'_> {
  fn start_fight(&mut self, mut first_attacker: Character, mut second_attacker: Character) {
    
      while first_attacker.health > 0.0 && second_attacker.health > 0.0 {

          if self.first_attacks_count == self.second_attacks_count {

              self.attack(&mut first_attacker, &mut second_attacker);
              self.first_attacks_count += 1;

          } else if self.first_attacks_count > self.second_attacks_count {

              self.attack(&mut second_attacker, &mut first_attacker);
              self.second_attacks_count += 1;
          }
      }

      if second_attacker.health <= 0.0 {
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