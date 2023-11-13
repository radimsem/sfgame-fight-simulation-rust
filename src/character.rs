use rand::{self, Rng};

use crate::stats::{self};

pub enum Roles {
    PLAYER,
    ENEMY
}

#[derive(Clone, Copy)]
pub enum GameClasses {
    MAGE,
    SCOUT,
    WARRIOR
}

fn handle_game_class_prop<T>(game_class: GameClasses, val_mage: T, val_scout: T, val_warrior: T) -> T {
    match game_class {
        GameClasses::MAGE => val_mage,
        GameClasses::SCOUT => val_scout,
        GameClasses::WARRIOR => val_warrior,
    }
}

#[derive(Clone, Copy)]
pub struct Character<'a> {
    name: &'a str,
    game_class: GameClasses,
    main_attribute: u32,
    constitution: i32,
    weapon_damage: u32,
    damage: f32,
    health: f32,
    defend_chance: i8,
    luck: u8,
    level: u8,
}

impl<'a> Character<'a> {
    pub fn new(name: &'a str, game_class: GameClasses) -> Self {
        let mut rng = rand::thread_rng();

        let main_attribute = rng.gen_range(stats::MIN_MAIN_ATTRIBUTE..stats::MAX_MAIN_ATTRIBUTE);
        let constitution = rng.gen_range(stats::MIN_CONSTITUTION..stats::MAX_CONSTITUTION) as i32;
        let weapon_damage = handle_game_class_prop(
            game_class,
            rng.gen_range(stats::MIN_WEAPON_DAMAGE_MAGE..stats::MAX_WEAPON_DAMAGE_MAGE),
            rng.gen_range(stats::MIN_WEAPON_DAMAGE_SCOUT..stats::MAX_WEAPON_DAMAGE_SCOUT),
            rng.gen_range(stats::MIN_WEAPON_DAMAGE_WARRIOR..stats::MAX_WEAPON_DAMAGE_WARRIOR)
        );
        let level = rng.gen_range(stats::MIN_LEVEL..stats::MAX_LEVEL) as u8;

        Self { 
            name, 
            game_class,
            main_attribute,
            constitution,
            weapon_damage,
            damage: (weapon_damage * (1 + main_attribute / 10)) as f32,
            health: handle_game_class_prop(
                game_class, 
                constitution * 2 * (level + 1) as i32, 
                constitution * 4 * (level + 1) as i32, 
                constitution * 5 * (level + 1) as i32
            ) as f32,
            luck: rng.gen_range(stats::MIN_LUCK..stats::MAX_LUCK) as u8,
            level,
            defend_chance: handle_game_class_prop(
                game_class, 
                0, 
                50, 
                25
            )
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

        println!("Class: {}", match self.game_class {
            GameClasses::MAGE => "Mage",
            GameClasses::SCOUT => "Scout",
            GameClasses::WARRIOR => "Warrior",
        });

        println!("Damage: {}", &self.damage);
        println!("Health: {}", &self.health);
        println!("Luck: {}%", &self.luck);
        println!();
    }
}

impl Character<'_> {
    pub fn get_name(&self) -> &str {
        self.name
    }
    pub fn get_class(&self) -> GameClasses {
        self.game_class
    }
    pub fn get_damage(&self) -> f32 {
        self.damage
    }
    pub fn get_health(&self) -> f32 {
        self.health
    }
    pub fn get_defend_chance(&self) -> i8 {
        self.defend_chance
    }
    pub fn get_luck(&self) -> u8 {
        self.luck
    }
}

impl Character<'_> {
    pub fn set_damage(&mut self, val: f32) {
        self.damage = val;
    }
    pub fn set_health(&mut self, val: f32) {
        self.health = val;
    }
} 