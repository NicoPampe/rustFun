pub mod hero;
pub mod character;

use hero::Hero;
use character::Character;

fn main() {
    println!("Hello, world!");

    let mut kurt: Hero = Character::new("Kurt");

    kurt.talk();
}
