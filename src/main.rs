mod character;
mod game;
mod stats;

use character::Character;
use game::Game;

fn main() {
    let player = Character::new("Spectre", character::GameClasses::SCOUT);
    let enemy = Character::new("Banshee", character::GameClasses::WARRIOR);

    let mut game = Game::new(player, enemy);
    game.start_game();
}