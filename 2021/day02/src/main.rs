use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

mod submarine_command;
use submarine_command::Command;

fn main() {
    println!("{}", String::from_utf8(vec![b'='; 40]).unwrap());
    solve_part_i();
    println!("{}", String::from_utf8(vec![b'='; 40]).unwrap());
    solve_part_ii();
    println!("{}", String::from_utf8(vec![b'='; 40]).unwrap());
}

fn read_lines(filename: impl AsRef<Path>) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
    match File::open(filename) {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(e) => Err(e),
    }
}

fn solve_part_i() {
    let mut depth: i16 = 0;
    let mut distance: i16 = 0;
    match read_lines("./data/input.txt") {
        Ok(lines) => {
            let commands: Vec<Command> = lines
                .into_iter()
                .map(|line| line.expect("Can't read this line"))
                .collect::<Vec<String>>()
                .into_iter()
                .map(|sc| {
                    sc.parse()
                        .expect("Can't convert this line to a submarine command.")
                })
                .collect();

            for cmd in commands {
                match cmd.direction.as_str() {
                    "forward" => distance += cmd.distance as i16,
                    "up" => depth -= cmd.distance as i16,
                    "down" => depth += cmd.distance as i16,
					_ => panic!("You'll never see this; but, just in case, Please note that there are unknown commands here!")
                }
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    };
    println!(
        "PART I - The multiplication result is {}",
        depth as i32 * distance as i32
    );
}

fn solve_part_ii() {
    let mut depth: i32 = 0;
    let mut position: i16 = 0;
    let mut aim: i16 = 0;
    match read_lines("./data/input.txt") {
        Ok(lines) => {
            let commands: Vec<Command> = lines
                .into_iter()
                .map(|line| line.expect("Can't read this line"))
                .collect::<Vec<String>>()
                .into_iter()
                .map(|sc| {
                    sc.parse()
                        .expect("Can't convert this line to a submarine command.")
                })
                .collect();

            for cmd in commands {
                match cmd.direction.as_str() {
                    "forward" => {
						position += cmd.distance as i16;
						depth += aim as i32 * cmd.distance as i32;
					},
                    "up" => aim -= cmd.distance as i16,
                    "down" => aim += cmd.distance as i16,
					_ => panic!("You'll never see this; but, just in case, Please note that there are unknown commands here!")
                }
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    };
    println!(
        "PART I - The multiplication result is {}",
        depth as i32 * position as i32
    );
}
