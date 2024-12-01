use std::collections::HashSet;

use itertools::Itertools;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    count: u32,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let tokens = value.trim().split(' ').collect_vec();
        let direction = match tokens[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Cannot parse direction!"),
        };
        let count = tokens[1].parse().expect("Cannot parse count!");

        Self { direction, count }
    }
}

impl Move {
    fn from_hex(hex: u32) -> Self {
        let direction = match hex & 0xF {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Cannot parse direction from hex value!"),
        };
        Self {
            direction,
            count: (hex >> 4) & 0xFFFFF,
        }
    }
}

#[derive(Debug)]
struct DigPlan {
    data: Vec<Move>,
}

impl From<&str> for DigPlan {
    fn from(value: &str) -> Self {
        Self {
            data: value.lines().map(|l| l.into()).collect(),
        }
    }
}

impl DigPlan {
    fn from_colours(input: &str) -> Self {
        let moves = input.lines().map(|l| {
            let colour = &l.split_once('#').unwrap().1[..6];
            let hex = u32::from_str_radix(colour, 16).unwrap();
            Move::from_hex(hex)
        });
        Self {
            data: moves.collect(),
        }
    }
}

type Position = (i64, i64);

impl DigPlan {
    fn dig_slow(&self) -> usize {
        let (height, width) = (1000, 1000);

        let mut interior_points: HashSet<Position> = (0..height as i64)
            .cartesian_product(0..width as i64)
            .collect();
        let mut boundary = HashSet::new();

        boundary.insert((0, 0));

        // dig out boundary
        let mut curr_pos = (0, 0);
        for m in self.data.iter() {
            let delta: (i64, i64) = match m.direction {
                Direction::Right => (0, 1),
                Direction::Left => (0, -1),
                Direction::Up => (-1, 0),
                Direction::Down => (1, 0),
            };

            let positions = (0..=m.count as i64)
                .map(|i| (curr_pos.0 + (delta.0 * i), curr_pos.1 + (delta.1 * i)))
                .collect_vec();

            boundary.extend(positions);

            curr_pos = (
                curr_pos.0 + delta.0 * m.count as i64,
                curr_pos.1 + delta.1 * m.count as i64,
            );
        }

        let min_row = boundary.iter().map(|&(r, _)| r).min().unwrap();
        let max_row = boundary.iter().map(|&(r, _)| r).max().unwrap();
        let min_col = boundary.iter().map(|&(_, c)| c).min().unwrap();
        let max_col = boundary.iter().map(|&(_, c)| c).max().unwrap();

        let height = (max_row - min_row + 1) as usize;
        let width = (max_col - min_col + 1) as usize;

        boundary = boundary
            .into_iter()
            .map(|(r, c)| (r - min_row, c - min_col))
            .collect();

        // remove boundary from set of all points
        interior_points.retain(|p| !boundary.contains(p));

        let mut remove_connected = |pos| {
            let mut stack = vec![];
            stack.push(pos);

            while let Some(pos) = stack.pop() {
                interior_points.remove(&pos);

                for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let new_pos = (pos.0 + delta.0, pos.1 + delta.1);
                    if interior_points.contains(&new_pos) {
                        stack.push(new_pos);
                    }
                }
            }
        };

        for c in 0..width as i64 {
            let pos_top = (0, c);
            let pos_bottom = (height as i64 - 1, c);
            if !boundary.contains(&pos_top) {
                remove_connected(pos_top);
            }
            if !boundary.contains(&pos_bottom) {
                remove_connected(pos_bottom);
            }
        }

        for r in 0..height as i64 {
            let pos_left = (r, 0);
            let pos_right = (r, width as i64 - 1);
            if !boundary.contains(&pos_left) {
                remove_connected(pos_left);
            }
            if !boundary.contains(&pos_right) {
                remove_connected(pos_right);
            }
        }

        // dig out interior
        let mut dug = vec![vec![false; width]; height];
        for pos in interior_points.union(&boundary) {
            let &(r, c) = pos;
            dug[r as usize][c as usize] = true;
        }

        // dug
        return boundary.union(&interior_points).count();
    }

    fn dig_fast(&self) -> usize {
        let mut area: i64 = 0;
        let mut height: i64 = 0;
        let mut perimeter: usize = 0;

        for mov in &self.data {
            match mov.direction {
                Direction::Down => height += mov.count as i64,
                Direction::Up => height -= mov.count as i64,
                Direction::Right => area -= height * mov.count as i64,
                Direction::Left => area += height * mov.count as i64,
            }
            perimeter += mov.count as usize;
        }

        area.unsigned_abs() as usize + perimeter / 2 + 1
    }
}

fn part1() {
    let input = include_str!("../../puzzle_input/d18").trim();
    let digplan: DigPlan = input.into();
    let dug = digplan.dig_slow();
    println!("{}", dug)
}

fn part2() {
    let input = include_str!("../../puzzle_input/d18").trim();
    let digplan = DigPlan::from_colours(input);
    let res = digplan.dig_fast();
    println!("{}", res);
}
