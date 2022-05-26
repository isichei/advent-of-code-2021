use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Direction {
    Up,
    Down,
    Forward
}

impl Direction {
    pub fn from_str(s: &str) -> Direction {
        match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            &_ => panic!("Whoops!")
        }
    }
}

struct Instruction {
    direction: Direction,
    amount: u32,
}

impl Instruction {
    pub fn new(line: String) -> Instruction {
        let mut iter = line.split(" ");
        let d = iter.next().unwrap();
        let a = iter.next().unwrap().parse::<u32>().unwrap();

        Instruction {
            direction: Direction::from_str(d),
            amount: a
        }
    }
}

fn main() {
    let mut h: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;
    if let Ok(lines) = read_lines("data/input.txt") {
        for line in lines {
            let i = Instruction::new(line.unwrap());
            match i.direction {
                Direction::Forward => {
                    h += i.amount;
                    depth += aim*i.amount;
                },
                Direction::Down => aim += i.amount,
                Direction::Up => aim -= i.amount,
            }
        }
    }
    println!("depth: {}, horizontal: {}, Ans: {}", depth, h, depth*h)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}