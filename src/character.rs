use rand::{self, Rng};

pub enum Roles {
    PLAYER,
    ENEMY
}

#[derive(Clone, Copy)]
pub struct Character<'a> {
    pub name: &'a str,
    pub damage: f32,
    pub health: f32,
    pub luck: u8
}

impl<'a> Character<'a> {
    pub fn new(name: &'a str) -> Self {
        let mut rng = rand::thread_rng();

        Self { 
            name, 
            damage: rng.gen_range(100..200) as f32,
            health: rng.gen_range(900..1400) as f32,
            luck: rng.gen_range(10..35)
        } 
    }
}

impl Character<'_> {
    pub fn print_stats(&self, role: Roles) {
        let role_name: &str;
        match role {
            Roles::PLAYER => role_name = "Player",
            Roles::ENEMY => role_name = "Enemy"
        }

        println!("{}: {}", role_name, &self.name);
        println!("Damage: {}", &self.damage);
        println!("Health: {}", &self.health);
        println!("Luck: {}%", &self.luck);
        println!();
    }
}