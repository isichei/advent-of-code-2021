use std::collections::VecDeque;

const INPUT: &str = include_str!("../data/input.txt");
const DAYS: usize = 256;
const VERBOSE: bool = false;

fn main() {
    let initial_fishes_iter = INPUT.split(",");
    let mut fish_counter: [u64; 9] = [0; 9];
    for i in initial_fishes_iter {
        let idx = i.parse::<usize>().unwrap();
        fish_counter[idx] += 1;
    }
    let mut deq = VecDeque::from(fish_counter);
    let mut current_day = 0;

    while current_day < DAYS {
        let new_fish_count = deq.pop_front().unwrap();
        deq[6] += new_fish_count;
        deq.push_back(new_fish_count);
        current_day += 1;
        if VERBOSE {
            println!("day {} has {} fish. {:?}", current_day, deq.iter().sum::<u64>(), deq);
        }
    }
    println!("day {} has {} fish. {:?}", current_day, deq.iter().sum::<u64>(), deq);
}