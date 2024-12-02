mod day1;
mod day2;
mod day3;
mod helpers;
mod io;

pub fn solution(day: u8, part: u8) -> String {
    let solution: Box<dyn Solution> = match day {
        1 => Box::new(day1::Day1),
        2 => Box::new(day2::Day2),
        3 => Box::new(day3::Day3),
        _ => unimplemented!("Day {} is not implemented!", day),
    };

    match part {
        1 => solution.part1_solution(),
        2 => solution.part2_solution(),
        _ => panic!("Puzzles only have part 1 and part 2!"),
    }
}

trait Solution {
    fn part1_solution(&self) -> String;
    fn part2_solution(&self) -> String;
}
