use std::iter::zip;

use crate::{helpers::make_counter, io, Solution};

pub struct Day1;

fn parse_input() -> Vec<(u32, u32)> {
    let input = io::get_puzzle_input(1);
    let input = input.trim();

    input
        .lines()
        .map(|line| {
            let (id1, id2) = line.split_once(' ').unwrap();
            (id1.parse().unwrap(), id2.trim().parse().unwrap())
        })
        .collect()
}

impl Solution for Day1 {
    fn part1_solution(&self) -> String {
        let lines = parse_input();

        let mut list1: Vec<_> = lines.iter().map(|line| line.0).collect();
        let mut list2: Vec<_> = lines.iter().map(|line| line.1).collect();

        list1.sort();
        list2.sort();

        let result: i32 = zip(list1, list2)
            .map(|(id1, id2)| (id1 as i32 - id2 as i32).abs())
            .sum();
        result.to_string()
    }

    fn part2_solution(&self) -> String {
        let lines = parse_input();

        let list1: Vec<_> = lines.iter().map(|line| line.0).collect();
        let list2: Vec<_> = lines.iter().map(|line| line.1).collect();

        let counter1 = make_counter(list1.into_iter());
        let counter2 = make_counter(list2.into_iter());

        let total: usize = counter1
            .into_iter()
            .map(|(id, count)| id as usize * counter2.get(&id).unwrap_or(&0) * count)
            .sum();

        total.to_string()
    }
}
