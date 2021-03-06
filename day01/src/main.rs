use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut values: Vec<i64> = Vec::new();

    if let Ok(lines) = read_lines("data/input.txt") {
        for line in lines {
            let v = line.unwrap().parse::<i64>().unwrap();
            values.push(v);
        }
    }

    let mut current: i64;
    let mut increase_count: u64 = 0;

    let mut win_iter = values.windows(3);
    let mut prev = win_iter.next().unwrap().iter().sum();

    for win in win_iter {
        current = win.iter().sum();
        if current > prev {
            increase_count += 1;
        }
        prev = current;
    }

    println!("Total increased: {}", increase_count)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}