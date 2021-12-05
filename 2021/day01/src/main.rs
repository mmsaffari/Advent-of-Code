use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("{:?}", String::from_utf8(vec![b'='; 40]));
    solve_part_i();
    println!("{:?}", String::from_utf8(vec![b'='; 40]));
    solve_part_ii();
    println!("{:?}", String::from_utf8(vec![b'='; 40]));
}

fn solve_part_i() {
    let mut increments: i16 = 0;
    match read_lines("./data/input.txt") {
        Ok(lines) => {
            let depts: Vec<i32> = lines
                .into_iter()
                .map(|line| line.expect("Can't read this line"))
                .collect::<Vec<String>>()
                .into_iter()
                .map(|d| {
                    d.parse::<i32>()
                        .expect("Can't convert this line to a number.")
                })
                .collect();
            let fenestres = depts.windows(2);
            for f in fenestres {
                if &f[1] > &f[0] {
                    increments += 1;
                }
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    };
    println!("PART I - {} increments detected!", increments);
}

fn solve_part_ii() {
    let mut increments: i16 = 0;
    match read_lines("./data/input.txt") {
        Ok(lines) => {
            let depts: Vec<i32> = lines
                .into_iter()
                .map(|line| line.expect("Can't read this line"))
                .collect::<Vec<String>>()
                .into_iter()
                .map(|d| {
                    d.parse::<i32>()
                        .expect("Can't convert this line to a number.")
                })
                .collect();
            let sums: Vec<i32> = depts
                .windows(3)
                .into_iter()
                .map(|f| &f[0] + &f[1] + &f[2])
                .collect();
            let fenestres = sums.windows(2);
            for f in fenestres {
                if &f[1] > &f[0] {
                    increments += 1;
                }
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    }
    println!("PART II - {} increments detected!", increments);
}

fn read_lines(filename: impl AsRef<Path>) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
    match File::open(filename) {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(e) => Err(e),
    }
}
