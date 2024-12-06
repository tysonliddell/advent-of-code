use std::collections::HashSet;

use crate::{io, Solution};

use Direction::*;

pub struct Day6;

type Position = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<char>>,
    start_position: Position,
    start_direction: Direction,
    curr_position: Option<Position>,
    curr_direction: Direction,
    width: usize,
    height: usize,
}

impl Direction {
    fn turn_90_right(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

impl Map {
    pub fn new(input: &str) -> Self {
        let data: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = data.len();
        let width = data[0].len();

        let mut start_pos = (0, 0);
        for (i, row) in data.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == '^' {
                    start_pos = (i, j);
                }
            }
        }

        Self {
            data,
            start_position: start_pos,
            start_direction: North,
            curr_position: Some(start_pos),
            curr_direction: North,
            height,
            width,
        }
    }

    pub fn reset(&mut self) {
        self.curr_direction = self.start_direction;
        self.curr_position = Some(self.start_position);
    }

    pub fn take_step(&mut self) -> Option<Position> {
        if let Some(pos) = self.curr_position {
            let next_pos_unchecked = |direction: Direction| match direction {
                North if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
                North if pos.0 == 0 => None,
                South if pos.0 < self.height - 1 => Some((pos.0 + 1, pos.1)),
                South if pos.0 >= self.height - 1 => None,
                West if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
                West if pos.1 == 0 => None,
                East if pos.1 < self.width - 1 => Some((pos.0, pos.1 + 1)),
                East if pos.1 >= self.width - 1 => None,
                _ => panic!(),
            };

            let mut dir = self.curr_direction;
            loop {
                let new_pos = next_pos_unchecked(dir);
                match new_pos {
                    None => {
                        self.curr_direction = dir;
                        self.curr_position = None;
                        break;
                    }
                    Some((row, col)) => {
                        if self.data[row][col] != '#' {
                            self.curr_direction = dir;
                            self.curr_position = new_pos;
                            break;
                        } else {
                            dir = dir.turn_90_right();
                        }
                    }
                }
            }
        }
        self.curr_position
    }
}

impl Solution for Day6 {
    fn part1_solution(&self) -> String {
        let input = io::get_puzzle_input(6);
        let mut map = Map::new(&input);
        let mut visited = HashSet::new();

        visited.insert(map.curr_position.unwrap());
        while let Some(pos) = map.take_step() {
            visited.insert(pos);
        }

        visited.len().to_string()
    }

    fn part2_solution(&self) -> String {
        let input = io::get_puzzle_input(6);
        let mut map = Map::new(&input);
        let mut visited = HashSet::new();

        while let Some(pos) = map.take_step() {
            visited.insert(pos);
        }
        map.reset();

        let obstruction_causes_cycle = |obstruction_pos: &Position| -> bool {
            let &(row, col) = obstruction_pos;
            map.data[row][col] = '#';

            let mut visited_with_obstruction = HashSet::new();
            let mut cycle_detected = false;
            while let Some(pos) = map.take_step() {
                if visited_with_obstruction.contains(&(pos, map.curr_direction)) {
                    cycle_detected = true;
                    break;
                }
                visited_with_obstruction.insert((pos, map.curr_direction));
            }

            map.data[row][col] = '.';
            map.reset();
            cycle_detected
        };

        let cycle_count = visited.into_iter().filter(obstruction_causes_cycle).count();
        cycle_count.to_string()
    }
}
