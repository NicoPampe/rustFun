// pub enum Race {
// 	Human,
// 	Elf
// }

pub trait Character {
	fn new(name: &'static str, x: i32) -> Self;

	// Instance methods, will return a string
	fn name(&self) -> &'static str;
	fn pos(&self) -> i32;

	// Defualt methods
	fn talk(&self) {
		println!("{} says Hello! They are at: {}", self.name(), self.pos());
	}
}