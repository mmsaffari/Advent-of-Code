#[derive(Debug, Clone)]
pub struct Cell {
	pub number: u8,
	pub marked: bool,
}

impl Cell {
	fn new(number: u8, marked: bool) -> Self {
		Self { number, marked }
	}
}
