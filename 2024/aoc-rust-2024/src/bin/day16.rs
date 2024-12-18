use std::collections::{BinaryHeap, HashSet};

use aoc_rust_2024::io;

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ReindeerPosition {
    location: (usize, usize),
    direction: Direction,
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<u8>>,
    start_pos: ReindeerPosition,
    end_pos: (usize, usize),
}

impl ReindeerPosition {
    fn turn_clockwise(&self) -> Self {
        let new_dir = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };

        Self {
            location: self.location,
            direction: new_dir,
        }
    }

    fn turn_anticlockwise(&self) -> Self {
        let new_dir = match self.direction {
            North => West,
            West => South,
            South => East,
            East => North,
        };

        Self {
            location: self.location,
            direction: new_dir,
        }
    }

    fn move_forward(&self) -> Self {
        let mut new_pos = self.location;
        match self.direction {
            North => new_pos.0 -= 1,
            South => new_pos.0 += 1,
            West => new_pos.1 -= 1,
            East => new_pos.1 += 1,
        }

        Self {
            location: new_pos,
            direction: self.direction,
        }
    }
}

impl Map {
    fn find_shortest_paths(&self) -> (u32, Vec<Vec<ReindeerPosition>>) {
        let height = self.data.len();
        let width = self.data[0].len();
        let mut distances = vec![vec![[u32::MAX, u32::MAX, u32::MAX, u32::MAX]; width]; height];
        let mut best_distance = u32::MAX;
        let mut best_paths = vec![];

        let mut nodes = BinaryHeap::new();
        nodes.push((0i32, self.start_pos, vec![]));

        let dir_to_slot = |dir: Direction| match dir {
            North => 0,
            South => 1,
            East => 2,
            West => 3,
        };

        while let Some((distance, pos, mut path)) = nodes.pop() {
            let distance = distance.unsigned_abs(); // using a max-heap, so negative distances are stored
            let (row, col) = pos.location;
            let slot = dir_to_slot(pos.direction);
            path.push(pos);

            if pos.location == self.end_pos {
                if best_distance == u32::MAX {
                    best_distance = distance;
                } else if distance > best_distance {
                    return (best_distance, best_paths);
                }
                best_paths.push(path);
            } else if distance <= distances[row][col][slot] {
                distances[row][col][slot] = distance;

                // only add next nodes if it's the first time we've visited this node
                let long_distance = -((distance + 1000) as i32);
                let short_distance = -((distance + 1) as i32);
                path.push(pos);
                nodes.push((long_distance, pos.turn_clockwise(), path.clone()));
                nodes.push((long_distance, pos.turn_anticlockwise(), path.clone()));

                let forward_pos = pos.move_forward();
                let (fp_row, fp_col) = forward_pos.location;
                if self.data[fp_row][fp_col] != b'#' {
                    nodes.push((short_distance, pos.move_forward(), path.clone()));
                }
            }
        }

        panic!("No path!");
    }
}

fn parse_input() -> Map {
    let input = io::get_puzzle_input(16);
    let input = input.trim();

    let data: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let (mut start_pos, mut end_pos) = ((0, 0), (0, 0));
    for (row, line) in data.iter().enumerate() {
        for (col, v) in line.iter().enumerate() {
            match v {
                b'S' => start_pos = (row, col),
                b'E' => end_pos = (row, col),
                _ => {}
            }
        }
    }

    Map {
        data,
        start_pos: ReindeerPosition {
            location: start_pos,
            direction: East,
        },
        end_pos,
    }
}

fn part1_solution() -> u32 {
    let map = parse_input();
    let (distance, _) = map.find_shortest_paths();
    distance
}

fn part2_solution() -> u32 {
    let map = parse_input();
    let (_, paths) = map.find_shortest_paths();
    let best_spots: HashSet<_> = paths
        .iter()
        .flat_map(|path| path.iter().map(|pos| pos.location))
        .collect();
    best_spots.len() as u32
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
