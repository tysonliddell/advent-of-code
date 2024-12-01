mod day1;
mod helpers;
mod io;

pub fn solution(day: u8, part: u8) -> String {
    let solution = match day {
        1 => Box::new(day1::Day1),
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
