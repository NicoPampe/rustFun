// extern crate rand;

// use std::io;	
// use std::cmp::Ordering;
// use rand::Rng;

struct Hero {
	name: String,
	age: i32,
}

trait Charater {
	// fn new(name: &'static)
	// http://rustbyexample.com/trait.html
}

fn main() {
    println!("Hello, world!");

    let mut player = Hero { name: "Nico".to_string(), age: 21 };
    println!("({}, {})", player.name, player.age);
}
