use std::collections::HashMap;

use cached::proc_macro::cached;

use aoc_rust_2024::{helpers::make_counter, io};

type Stone = u64;
type Stones = HashMap<Stone, usize>;

#[cached]
fn blink_stone(stone: Stone) -> (Stone, Option<Stone>) {
    if stone == 0 {
        return (1, None);
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (s1, s2) = stone_str.split_at(stone_str.len() / 2);
        let (s1, s2) = (s1.trim_start_matches('0'), s2.trim_start_matches('0'));
        let s1 = (if s1.is_empty() { "0" } else { s1 }).to_string();
        let s2 = (if s2.is_empty() { "0" } else { s2 }).to_string();
        (s1.parse().unwrap(), Some(s2.parse().unwrap()))
    } else {
        (stone * 2024, None)
    }
}

fn blink(stones: &mut Stones) {
    let mut new_stones = HashMap::new();
    for (&stone, &count) in stones.iter() {
        let (s1, s2) = blink_stone(stone);
        *new_stones.entry(s1).or_insert(0) += count;
        if let Some(s2) = s2 {
            *new_stones.entry(s2).or_insert(0) += count;
        }
    }
    *stones = new_stones;
}

fn parse_input() -> Vec<Stone> {
    let input = io::get_puzzle_input(11);
    let input = input.trim();

    input.split(' ').map(|n| n.parse().unwrap()).collect()
}

fn part1_solution() -> usize {
    let stones = parse_input();
    let mut stones = make_counter(stones.into_iter());

    for _ in 0..25 {
        blink(&mut stones);
    }
    stones.values().sum()
}

fn part2_solution() -> usize {
    let stones = parse_input();
    let mut stones = make_counter(stones.into_iter());

    for _ in 0..75 {
        blink(&mut stones);
    }
    stones.values().sum()
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
