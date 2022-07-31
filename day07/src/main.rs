use std::collections::HashMap;

const INPUT: &str = include_str!("../data/input.txt");

fn calculate_move_p1(positions: &Vec<u32>, move_to: u32) -> u32 {
    positions.iter().map(|p| if p > &move_to {p - &move_to} else {&move_to - p}).sum::<u32>()
}

fn calculate_move_p2(positions: &Vec<u32>, move_to: u32) -> u32 {
    let mut cost: u32 = 0;
    for p in positions {
        let dist = if p > &move_to {
            p - &move_to
        } else {
            &move_to - p
        };
        if dist > 0 {
            cost += (dist * (dist + 1))/2;
        }
    }
    cost
}

fn main() {
    let mut crab_scores: HashMap<u32, u32> = HashMap::new();
    let original_crab_positions: Vec<u32> = INPUT.split(",").map(|p| p.parse::<u32>().unwrap()).collect();
    let max_h = *original_crab_positions.iter().max().unwrap();
    let mut lowest: (u32, u32) = (0, std::u32::MAX);
    let mut crab_key = 0;

    for origin_pos in &original_crab_positions {
        crab_scores.entry(*origin_pos).or_insert(0);
    }

    while crab_key <= max_h {
        let score = calculate_move_p2(&original_crab_positions, crab_key);
        crab_scores.insert(crab_key, score);
        if score < lowest.1 {
            lowest = (crab_key, score);
        }
        crab_key += 1;
    }

    println!("{:?}", crab_scores);
    println!("{:?}", lowest);
}
