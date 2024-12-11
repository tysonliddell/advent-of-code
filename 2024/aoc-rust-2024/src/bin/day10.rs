use std::collections::HashSet;

use itertools::Itertools;

use aoc_rust_2024::io;

type Map = Vec<Vec<u32>>;

#[derive(PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn get_neighbours(&self, map: &Map) -> Vec<Self> {
        let height = map.len() as i32;
        let width = map[0].len() as i32;

        let nbhrs = [
            (self.row as i32 - 1, self.col as i32),
            (self.row as i32 + 1, self.col as i32),
            (self.row as i32, self.col as i32 - 1),
            (self.row as i32, self.col as i32 + 1),
        ]
        .into_iter()
        .filter(|(row, col)| (0..height).contains(row) && (0..width).contains(col))
        .map(|(row, col)| Position {
            row: row as usize,
            col: col as usize,
        });
        nbhrs.collect()
    }
}

fn get_reachable_summits(map: &Map, position: Position) -> HashSet<Position> {
    let current_height = map[position.row][position.col];
    if current_height == 9 {
        return HashSet::from([position]);
    }

    let nbrs = position.get_neighbours(map);
    nbrs.into_iter()
        .filter(|pos| map[pos.row][pos.col] == current_height + 1)
        .flat_map(|pos| get_reachable_summits(map, pos).into_iter())
        .collect()
}

fn get_rating(map: &Map, position: Position) -> usize {
    let current_height = map[position.row][position.col];
    if current_height == 9 {
        return 1;
    }

    let nbrs = position.get_neighbours(map);
    nbrs.into_iter()
        .filter(|pos| map[pos.row][pos.col] == current_height + 1)
        .map(|pos| get_rating(map, pos))
        .sum()
}

fn iter_map_positions(map: &Map) -> impl Iterator<Item = Position> {
    let height = map.len();
    let width = map[0].len();

    (0..height)
        .cartesian_product(0..width)
        .map(|(row, col)| Position { row, col })
}

fn parse_input() -> Map {
    let input = io::get_puzzle_input(10);
    let input = input.trim();

    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(99)).collect())
        .collect()
}

fn part1_solution() -> usize {
    let map = parse_input();

    iter_map_positions(&map)
        .filter(|pos| map[pos.row][pos.col] == 0)
        .map(|pos| get_reachable_summits(&map, pos).len())
        .sum()
}

fn part2_solution() -> usize {
    let map = parse_input();

    iter_map_positions(&map)
        .filter(|pos| map[pos.row][pos.col] == 0)
        .map(|pos| get_rating(&map, pos))
        .sum()
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
