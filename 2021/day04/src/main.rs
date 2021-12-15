mod internals;

use std::{
	fs::File,
	io::{self, BufRead, BufReader, Error, Lines},
	path::Path,
};

use internals::{Cell, Board};
fn main() {
	println!("{:}", String::from_utf8(vec![b'='; 40]).unwrap());
	solve_part_i();
	println!("{:}", String::from_utf8(vec![b'='; 40]).unwrap());
	solve_part_ii();
	println!("{:}", String::from_utf8(vec![b'='; 40]).unwrap());
}

fn read_lines(filename: impl AsRef<Path>) -> Result<Lines<BufReader<File>>, Error> {
	match File::open(filename) {
		Ok(file) => Ok(io::BufReader::new(file).lines()),
		Err(e) => Err(e),
	}
}

fn solve_part_i(){

}

fn solve_part_ii(){}