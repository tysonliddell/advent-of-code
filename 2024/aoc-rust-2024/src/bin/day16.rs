use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

use aoc_rust_2024::io;

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn find_shortest_paths(&self) -> (u32, u32) {
        let height = self.data.len();
        let width = self.data[0].len();
        let mut best_distance = None;
        let mut distances = vec![vec![[u32::MAX, u32::MAX, u32::MAX, u32::MAX]; width]; height];
        let mut prev_nodes = vec![
            vec![
                [
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                ];
                width
            ];
            height
        ];

        let mut nodes = BinaryHeap::new();
        nodes.push((0i32, self.start_pos, None));

        let dir_to_slot = |dir: Direction| match dir {
            North => 0,
            South => 1,
            East => 2,
            West => 3,
        };

        while let Some((distance, pos, prev_pos)) = nodes.pop() {
            let distance = distance.unsigned_abs(); // using a max-heap, so negative distances are stored

            if best_distance.is_some_and(|best_distance| distance > best_distance) {
                // We've hit the first node after iterating all paths with distance <= best_distance
                // time to aggreate results and return.
                let mut best_positions = HashSet::new();
                let mut stack: Vec<ReindeerPosition> = vec![
                    ReindeerPosition {
                        location: self.end_pos,
                        direction: North,
                    },
                    ReindeerPosition {
                        location: self.end_pos,
                        direction: South,
                    },
                    ReindeerPosition {
                        location: self.end_pos,
                        direction: East,
                    },
                    ReindeerPosition {
                        location: self.end_pos,
                        direction: West,
                    },
                ];

                while let Some(pos) = stack.pop() {
                    best_positions.insert(pos.location);

                    let (row, col) = pos.location;
                    let slot = dir_to_slot(pos.direction);
                    for &prev in &prev_nodes[row][col][slot] {
                        stack.push(prev);
                    }
                }
                return (best_distance.unwrap(), best_positions.len() as u32);
            }

            let (row, col) = pos.location;
            let slot = dir_to_slot(pos.direction);
            if pos.location == self.end_pos {
                prev_nodes[row][col][slot].insert(prev_pos.unwrap());
                if best_distance.is_none() {
                    best_distance = Some(distance);
                }
            } else if distance <= distances[row][col][slot] {
                if distance < distances[row][col][slot] {
                    // new best distance to this node
                    prev_nodes[row][col][slot].clear();
                    distances[row][col][slot] = distance;
                }
                if let Some(prev) = prev_pos {
                    prev_nodes[row][col][slot].insert(prev);
                }

                let long_distance = -((distance + 1000) as i32);
                let short_distance = -((distance + 1) as i32);
                nodes.push((long_distance, pos.turn_clockwise(), Some(pos)));
                nodes.push((long_distance, pos.turn_anticlockwise(), Some(pos)));

                let forward_pos = pos.move_forward();
                let (fp_row, fp_col) = forward_pos.location;
                if self.data[fp_row][fp_col] != b'#' {
                    nodes.push((short_distance, pos.move_forward(), Some(pos)));
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
    let (_, num_best_spots_to_sit) = map.find_shortest_paths();
    num_best_spots_to_sit
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
