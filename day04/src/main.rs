
use std::result::Result;

const INPUT: &str = include_str!("../data/example.txt");
const BOARD_SIZE: usize = 5;

#[derive(Debug, PartialEq)]
enum MyError {
    ParseError,
    BoardSizeError,
    BoardCreationError,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    value: u32,
    called: bool
}

struct BingoBoard {
    board: [[Pos; BOARD_SIZE]; BOARD_SIZE],
}

impl BingoBoard {
    // Create board with zeros and all called = False
    fn new() -> BingoBoard {
        Self {
            board: [[Pos{value:0, called:false}; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    // Create board with values from string and all called = False
    fn parse(str_board: &str) -> Result<BingoBoard, MyError> {
        let mut new_board = BingoBoard::new();
    
        let mut row_iter = str_board.split('\n');
        for row in new_board.board.iter_mut() {
            let str_row = row_iter.next().unwrap();
            let mut str_value_iter = str_row.split_whitespace();
            for pos in row.iter_mut() {
                let str_value = match str_value_iter.next() {
                    Some(str_value) => str_value,
                    None => return Err(MyError::BoardCreationError),
                };
                match str_value.parse::<u32>() {
                    Ok(parsed_int) => pos.value = parsed_int,
                    Err(_) => return Err(MyError::BoardCreationError)
                };
            }
        }
        Ok(new_board)
    }
}

impl std::fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board {
            for item in row {
                if item.called {
                    write!(f, "({})\t", item.value)?;
                } else {
                    write!(f, "{}\t", item.value)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::ParseError => write!(f, "Failed to parse the given file"),
            _ => write!(f, "DNO MATE!"),
        }
    }
}

fn main() {
    let bingo_numbers = get_caller_numbers().unwrap();

    println!("BINGO NUMBERS: {:?}", bingo_numbers);
    let bingo_boards = get_all_bingo_boards().unwrap();

    for (i, bb) in bingo_boards.iter().enumerate(){
        println!("=== board num {} ===", i);
        println!("{}", bb);
        println!("=== | ===");
    }
}

fn get_caller_numbers_from_str(line: &str) -> Result<Vec<u32>, MyError> {
    let mut v = Vec::new();
    for str_value in line.split(",") {
        match str_value.parse::<u32>(){
            Ok(value) => v.push(value),
            Err(_) => return Err(MyError::ParseError)
        }
    }
    return Ok(v)
}

fn get_caller_numbers() -> Result<Vec<u32>, MyError> {

    let first_line = match INPUT.split_once('\n') {
        Some(values) => values.0,
        None => return Err(MyError::ParseError)
    };

    get_caller_numbers_from_str(first_line)
}


fn get_all_bingo_boards() -> Result<Vec<BingoBoard>, MyError> {
    // Create iterator and skip first line which are bingo numbers
    let remaining = match INPUT.split_once("\n\n") {
        Some(values) => values.1,
        None => return Err(MyError::ParseError)
    };

    let mut v = Vec::new();

    for board in remaining.split("\n\n") {
        v.push(BingoBoard::parse(board)?);
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_caller_numbers_from_str() {
        let test = "1,33,100,0,10";
        assert_eq!(get_caller_numbers_from_str(test).unwrap(), vec![1, 33, 100, 0, 10]);
    }

    #[test]
    fn test_create_new_board() {
        let test = "1 2 3 4 5\n1 2 3 4 5\n1 2 3 4 5\n1 2 3 4 5\n1 2 3 4 5\n";
        let expected_board = [[Pos{value:1, called:false}, Pos{value:2, called:false}, Pos{value:3, called:false}, Pos{value:4, called:false}, Pos{value:5, called:false}]; 5];
        assert_eq!(BingoBoard::parse(test).unwrap().board, expected_board);
    }
}