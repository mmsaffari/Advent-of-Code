mod int_utils;

//use crate::int_utils::Utils;
use std::{
	fmt::Debug,
	fs::File,
	io::{self, BufRead},
	path::Path,
};

fn main() {
	println!("{:}", String::from_utf8(vec![b'='; 40]).unwrap());
	solve_part_i();
	println!("{:}", String::from_utf8(vec![b'='; 40]).unwrap());
	solve_part_ii();
	println!("{:}", String::from_utf8(vec![b'='; 40]).unwrap());
}

fn read_lines(filename: impl AsRef<Path>) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
	match File::open(filename) {
		Ok(file) => Ok(io::BufReader::new(file).lines()),
		Err(e) => Err(e),
	}
}

fn solve_part_i() {
	#[derive(Debug, Clone, Copy)]
	struct Counter {
		ones: i16,
		zeros: i16,
	}

	let mut assessment: Vec<Counter> = Vec::new();

	match read_lines("./data/input.txt") {
		Ok(lines) => {
			let diag_lines_str: Vec<String> = lines
				.into_iter()
				.map(|line| -> String { line.expect("Can't read this line") })
				.collect();

			let diag_lines: Vec<Vec<char>> = diag_lines_str
				.iter()
				.map(|d| -> Vec<char> { d.chars().collect() })
				.collect();

			assessment = vec![Counter { ones: 0, zeros: 0 }; diag_lines[0].len()];

			for diag in diag_lines {
				for idx in 0..diag.len() {
					match diag[idx] {
						'0' => assessment[idx].zeros += 1,
						'1' => assessment[idx].ones += 1,
						_ => {
							println!("ERROR");
						}
					}
				}
			}
		}
		Err(e) => {
			println!("ERROR: {}", e);
		}
	};
	let mut gama_rate: i32 = 0;
	let mut epsilon_rate: i32 = 0;
	let ass_len = assessment.len();
	for idx in 0..ass_len {
		if assessment[idx].ones > assessment[idx].zeros {
			gama_rate = gama_rate | (1 << (ass_len - idx - 1));
		} else {
			epsilon_rate = epsilon_rate | (1 << (ass_len - idx - 1));
		}
	}

	let power_consumption: i32 = gama_rate * epsilon_rate;
	println!("PART I - Gama Rate:                {:016b}\t\t{}", gama_rate, gama_rate);
	println!(
		"PART I - Epsilon Rate:             {:016b}\t\t{}",
		epsilon_rate, epsilon_rate
	);
	println!("PART I - The power consumption is: {}", power_consumption);
}

fn solve_part_ii() {
	let oxygen_generator_rating: i16;
	let co2_scrubber_rating: i16;
	match read_lines("./data/input.txt") {
		Ok(lines) => {
			let diag_lines_str: Vec<String> = lines
				.into_iter()
				.map(|line| -> String { line.expect("Can't read this line") })
				.collect();

			let diag_len: usize = diag_lines_str[0].len();
			let mut ogr_diag_lines = diag_lines_str.clone();
			let mut csr_diag_lines = diag_lines_str.clone();
			for idx in 0..diag_len {
				if ogr_diag_lines.len() > 1 {
					let ogr_ones_in_idx: Vec<String> = ogr_diag_lines
						.iter()
						.filter(|d| d.chars().nth(idx).unwrap() == '1')
						.cloned()
						.collect();
					if ogr_ones_in_idx.len() >= ogr_diag_lines.len() - ogr_ones_in_idx.len() {
						ogr_diag_lines.retain(|d| d.chars().nth(idx).unwrap() == '1');
					} else {
						ogr_diag_lines.retain(|d| d.chars().nth(idx).unwrap() == '0');
					}
				}

				if csr_diag_lines.len() > 1 {
					let csr_ones_in_idx: Vec<String> = csr_diag_lines
						.iter()
						.filter(|d| d.chars().nth(idx).unwrap() == '1')
						.cloned()
						.collect();
					if csr_ones_in_idx.len() < csr_diag_lines.len() - csr_ones_in_idx.len() {
						csr_diag_lines.retain(|d| d.chars().nth(idx).unwrap() == '1');
					} else {
						csr_diag_lines.retain(|d| d.chars().nth(idx).unwrap() == '0');
					}
				}
			}

			oxygen_generator_rating = i16::from_str_radix(ogr_diag_lines[0].as_str(), 2).unwrap();
			co2_scrubber_rating = i16::from_str_radix(csr_diag_lines[0].as_str(), 2).unwrap();
		}
		Err(e) => {
			println!("ERROR: {}", e);
			oxygen_generator_rating = -1;
			co2_scrubber_rating = -1;
		}
	};

	println!("Part II - Oxygen generator rating: {0:#b} = {0}", oxygen_generator_rating);
	println!("Part II - CO2 scrubber rating: {0:#b} = {0}", co2_scrubber_rating);
	let life_support_rating: i32 = i32::from(oxygen_generator_rating) * i32::from(co2_scrubber_rating);
	println!("Part II - The life support rating is: {:#}", life_support_rating);
}
