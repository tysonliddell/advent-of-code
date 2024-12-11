use std::collections::{HashMap, HashSet};

use aoc_rust_2024::io;

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Antenna {
    position: Position,
    _frequency: char,
}

impl Antenna {
    fn get_antinode_positions(&self, other: &Self) -> Option<(Position, Position)> {
        let dy = other.position.y - self.position.y;
        let dx = other.position.x - self.position.x;

        Some((
            Position {
                x: self.position.x + 2 * dx,
                y: self.position.y + 2 * dy,
            },
            Position {
                x: self.position.x - dx,
                y: self.position.y - dy,
            },
        ))
    }

    fn get_antinode_positions_with_harmonics(
        &self,
        other: &Self,
        bounds: (usize, usize),
    ) -> HashSet<Position> {
        let (height, width) = bounds;
        let dy = other.position.y - self.position.y;
        let dx = other.position.x - self.position.x;

        let is_pos_in_bounds =
            |(x, y): (i32, i32)| x >= 0 && x < width as i32 && y >= 0 && y < height as i32;

        let mut positions = HashSet::new();

        // walk forward
        let mut pos = (self.position.x, self.position.y);
        while is_pos_in_bounds(pos) {
            positions.insert(Position { x: pos.0, y: pos.1 });
            pos = (pos.0 + dx, pos.1 + dy);
        }

        // walk backward
        let mut pos = (self.position.x, self.position.y);
        while is_pos_in_bounds(pos) {
            positions.insert(Position { x: pos.0, y: pos.1 });
            pos = (pos.0 - dx, pos.1 - dy);
        }

        positions
    }
}

fn parse_input() -> (HashMap<char, Vec<Antenna>>, (usize, usize)) {
    let input = io::get_puzzle_input(8);
    let input: Vec<_> = input.trim().lines().collect();

    let height = input.len();
    let width = input[0].len();

    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    for (row, line) in input.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_alphanumeric() {
                antennas.entry(c).or_default().push(Antenna {
                    position: Position {
                        x: col as i32,
                        y: row as i32,
                    },
                    _frequency: c,
                });
            }
        }
    }
    (antennas, (height, width))
}

fn part1_solution() -> usize {
    let (mut antennas, (height, width)) = parse_input();

    let mut antinodes = HashSet::new();
    for (_, antennas) in antennas.iter_mut() {
        while let Some(antenna) = antennas.pop() {
            for other in antennas.iter() {
                let (antinode1, antinode2) = antenna.get_antinode_positions(other).unwrap();
                antinodes.insert(antinode1);
                antinodes.insert(antinode2);
            }
        }
    }

    antinodes
        .into_iter()
        .filter(|pos| (0..height as i32).contains(&pos.y) && (0..width as i32).contains(&pos.x))
        .count()
}

fn part2_solution() -> usize {
    let (mut antennas, (height, width)) = parse_input();

    let mut antinodes = HashSet::new();
    for (_, antennas) in antennas.iter_mut() {
        while let Some(antenna) = antennas.pop() {
            for other in antennas.iter() {
                let positions =
                    antenna.get_antinode_positions_with_harmonics(other, (height, width));
                antinodes.extend(positions);
            }
        }
    }

    antinodes.len()
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
