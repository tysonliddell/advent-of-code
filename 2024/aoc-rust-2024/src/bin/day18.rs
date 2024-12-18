use std::collections::{HashSet, VecDeque};

use aoc_rust_2024::io;

type BytePosition = (usize, usize);

const MAX_MEM_ROW: usize = 70;
const MAX_MEM_COL: usize = 70;

fn parse_input() -> Vec<BytePosition> {
    let input = io::get_puzzle_input(18);
    let input = input.trim();

    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

fn get_neighbours(pos: BytePosition, max_row: usize, max_col: usize) -> Vec<BytePosition> {
    let (row, col) = pos;
    let nbrs = vec![
        (row as i32 - 1, col as i32),
        (row as i32 + 1, col as i32),
        (row as i32, col as i32 - 1),
        (row as i32, col as i32 + 1),
    ];
    nbrs.into_iter()
        .filter(|(row, col)| {
            (0..=max_row as i32).contains(row) && (0..=max_col as i32).contains(col)
        })
        .map(|(row, col)| (row as usize, col as usize))
        .collect()
}

fn get_shortest_path_length(
    fallen_bytes: &[BytePosition],
    start: BytePosition,
    end: BytePosition,
    max_row: usize,
    max_col: usize,
) -> Option<usize> {
    let corrupted_bytes: HashSet<(usize, usize)> = HashSet::from_iter(fallen_bytes.iter().copied());

    let mut visited = HashSet::new();
    let mut q = VecDeque::from([(0, start)]);

    while let Some((distance, pos)) = q.pop_front() {
        if pos == end {
            return Some(distance);
        }

        let noncorrupted_neighbours = get_neighbours(pos, max_row, max_col)
            .into_iter()
            .filter(|pos| !corrupted_bytes.contains(pos));

        for n in noncorrupted_neighbours {
            if !visited.contains(&n) {
                visited.insert(n);
                q.push_back((distance + 1, n));
            }
        }
    }

    None
}

fn part1_solution() -> usize {
    let positions = parse_input();
    let start = (0, 0);
    let end = (MAX_MEM_ROW, MAX_MEM_COL);

    get_shortest_path_length(&positions[..1024], start, end, MAX_MEM_ROW, MAX_MEM_COL).unwrap()
}

fn part2_solution() -> (usize, usize) {
    let positions = parse_input();
    let start = (0, 0);
    let end = (MAX_MEM_ROW, MAX_MEM_COL);

    for i in 1024..10000 {
        if get_shortest_path_length(&positions[..i], start, end, MAX_MEM_ROW, MAX_MEM_COL).is_none()
        {
            return positions[i - 1];
        }
    }

    panic!("No solution found!");
}

fn main() {
    println!("{}", part1_solution());
    println!("{:?}", part2_solution());
}
