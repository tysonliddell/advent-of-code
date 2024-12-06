use std::iter::zip;

use aoc_rust_2024::{helpers::make_counter, io};

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

fn part1_solution() -> u32 {
    let lines = parse_input();

    let mut list1: Vec<_> = lines.iter().map(|line| line.0).collect();
    let mut list2: Vec<_> = lines.iter().map(|line| line.1).collect();

    list1.sort();
    list2.sort();

    zip(list1, list2)
        .map(|(id1, id2)| (id1 as i32 - id2 as i32).unsigned_abs())
        .sum()
}

fn part2_solution() -> u32 {
    let lines = parse_input();

    let list1: Vec<_> = lines.iter().map(|line| line.0).collect();
    let list2: Vec<_> = lines.iter().map(|line| line.1).collect();

    let counter1 = make_counter(list1.into_iter());
    let counter2 = make_counter(list2.into_iter());

    counter1
        .into_iter()
        .map(|(id, count)| id * (counter2.get(&id).unwrap_or(&0) * count) as u32)
        .sum()
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
