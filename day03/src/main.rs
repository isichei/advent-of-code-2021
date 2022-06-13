use std::fs::File;
use std::io;
use std::path::Path;
use std::convert::TryInto;
use std::io::BufRead;

#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    DowncastError(std::num::TryFromIntError)
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::IoError(io_err) => io_err.fmt(f),
            MyError::DowncastError(down_err) => down_err.fmt(f),
        }
    }
}

impl std::error::Error for MyError {}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::IoError(err)
    }
}
impl From<std::num::TryFromIntError> for MyError {
    fn from(err: std::num::TryFromIntError) -> Self {
        MyError::DowncastError(err)
    }
}

#[derive(Debug)]
struct Reading {
    reading: Vec<bool>,
    on: bool
}

impl Reading {
    fn select(mut self, pos: usize, b: bool) {
        if self.reading[pos] != b {
            self.on = false;
        }
    }

    fn reset(mut self) {
        self.on = true;
    }

    pub fn new(line: String) -> Reading {
        Reading { reading: create_arr(line), on: true }
    }

    pub fn calculate_dec(&self) -> u64 {
        let mut factor: u64 = u64::pow(2, (self.reading.len() - 1).try_into().unwrap());
        let mut calc: u64 = 0;
        for r in self.reading.iter() {
            if *r {
                calc += factor;
            }
            factor /= 2;
        }
        calc
    }
}

fn search(readings: &mut [Reading], pos: usize, b: bool) {
    for r in readings.iter_mut() {
        if r.reading[pos] != b {
            r.on = false;
        }
    }
}

fn most_common(readings: &[Reading], pos: usize) -> bool {
    let counter = count_remaining_readings(readings);
    let mut common_true = readings.iter().map(|r| if r.on && r.reading[pos] {1} else {0}).sum::<u32>();
    common_true *= 2;
    // println!("\t ... {}, {}", common_true, counter);
    common_true >= counter
}

fn count_remaining_readings(readings: &[Reading]) -> u32 {
    readings.iter().map(|r| if r.on {1} else {0}).sum::<u32>()
}

fn print_remaining_readings(readings: &[Reading]){
    for r in readings.iter() {
        if r.on {
            println!("{:?}", r.reading);
        }
    }
    println!("");
}

fn main() { 
    let file_path = "data/input.txt";
    let array_len = match get_array_len(file_path) {
        Ok(a) => a,
        Err(MyError::IoError(err)) => {
            println!("IO ERROR: {}", err);
            return 
        }
        Err(MyError::DowncastError(err)) => {
            println!("CASTING ERROR: {}", err);
            return 
        }
    };

    // Don't actually need this janky error handling anymore 
    // but as it took so long!
    println!("Running with array lengths of {}", array_len);

    let lines = match read_lines(file_path) {
        Ok(lines) => lines,
        Err(_) => panic!("Something went wrong reading the file...")
    };

    let mut readings: Vec<Reading> = Vec::new();

    for line in lines {
        readings.push(Reading::new(line.unwrap()));
    }

    let oxygen: u64;
    let co2: u64;

    // Get the Oxygen Reading
    let mut i:usize = 0;
    let mut filter_by: bool;
    while count_remaining_readings(&readings) > 1 {
        filter_by = most_common(&readings, i);
        search(&mut readings, i, filter_by);
    
        // println!("i: {}\t| mc: {}\t| remaining: {}", i, filter_by, count_remaining_readings(&readings));
        // print_remaining_readings(&readings);
        i += 1;
    }

    oxygen = readings.iter().filter(|r| r.on).next().unwrap().calculate_dec();
    
    // Reset
    for r in readings.iter_mut(){
        r.on = true;
    }
    i = 0;

    // Get the CO2 Reading
    while count_remaining_readings(&readings) > 1 {
        filter_by = !most_common(&readings, i);
        search(&mut readings, i, filter_by);
        i += 1;
    }

    co2 = readings.iter().filter(|r| r.on).next().unwrap().calculate_dec();
    println!("Oxygen:{}\t| CO2: {}\t| Life Support {}", oxygen, co2, oxygen*co2);
}



fn create_arr(line: String) -> Vec<bool> {
    line.chars().map(|c| c == '1').collect()
}


fn get_array_len<P: AsRef<Path>>(filename: P) -> Result<u64, MyError>
{
    let file = File::open(filename)?;
    let mut buf = String::new();
    io::BufReader::new(file).read_line(&mut buf)?;
    let out: u64 = buf.len().try_into()?;
    Ok(out - 1)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_stats_from_readings(line: String, array_len: usize) -> (Vec<u32>, u32) {
    let mut counter: Vec<u32> = vec![0; array_len];
    let mut line_num: u32 = 0;
    let reading = create_arr(line);
    for (i, r) in reading.iter().enumerate() {
        if *r { counter[i] += 1 }
    }
    line_num += 1;
    (counter, line_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_array() {
        let test = String::from("11010");

        assert_eq!(create_arr(test), [true, true, false, true, false]);
    }

    #[test]
    fn get_array_len_from_file() {
        assert_eq!(get_array_len("data/example.txt").unwrap(), 5);
    }
}