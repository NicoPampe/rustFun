// pub mod character;

use character::Character;

pub struct Hero {
	pub name: &'static str,
	pub x: i32
	// race: race_classes::Race
	// class: &'static str,
}

impl Hero {
	// add code here
}

impl Character for Hero {
	fn new(name: &'static str, x: i32) -> Hero {
		Hero { name: name, x: x }
	} 

	fn name(&self) -> &'static str {
		self.name
	}

	fn pos(&self) -> i32 {
		self.x
	}

	// fn pos(x: i32) {
	// 	self.x = x;
	// }
}

