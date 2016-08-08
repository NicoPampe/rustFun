pub mod hero;
pub mod character;

use hero::Hero;
use character::Character;

use std::io;
use std::string::String;
use std::convert::AsRef;


fn main() {
    println!("Hello, world!");

    let mut kurt: Hero = Character::new("Kurt");

    kurt.talk();

    loop {
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();

        match user_input.as_ref() {
            "list" => println!("It's a list"),
            "exit" => {
                println!("Done!");
                break;
            },
            _ => println!("nothing was entered {}", user_input),
        }
    }
}
