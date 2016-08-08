// pub enum Race {
// 	Human,
// 	Elf
// }

pub trait Character {
	fn new(name: &'static str) -> Self;

	// Instance methods, will return a string
	fn name(&self) -> &'static str;

	// Defualt methods
	fn talk(&self) {
		println!("{} says Hello!", self.name());
	}
}