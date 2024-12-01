use std::{fs, path::Path};

pub mod day1;
pub mod helpers;

pub fn solution(day: u8, part: u8) -> String {
    let solution = match day {
        1 => Box::new(day1::Day1),
        _ => unimplemented!("Day {} is not implemented!", day),
    };

    match part {
        1 => solution.part1(),
        2 => solution.part2(),
        _ => panic!("Puzzles only part 1 and part 2!"),
    }
}

trait Solution {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

fn get_puzzle_input(day: u8) -> String {
    const PUZZLE_DIR: &str = "../puzzle_input";

    let filename = format!("d{}", day);
    let puzzle_input_path = Path::new(PUZZLE_DIR).join(filename);
    let input = fs::read_to_string(puzzle_input_path).unwrap();
    input
}
