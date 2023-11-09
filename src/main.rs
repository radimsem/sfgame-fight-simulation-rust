mod character;
mod game;

use character::Character;
use game::Game;

fn main() {
    let player = Character::new("Spectre");
    let enemy = Character::new("Banshee");

    let mut game = Game::new(player, enemy);
    game.start_game();
}