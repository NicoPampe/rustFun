// pub mod character;

use character::Character;

pub struct Hero {
	name: &'static str
	// race: race_classes::Race
	// class: &'static str,
}

impl Hero {
	// add code here
}

impl Character for Hero {
	fn new(name: &'static str) -> Hero {
		Hero { name: name }
	} 

	fn name(&self) -> &'static str {
		self.name
	}
}

