use std::{collections::HashSet, iter};

use aoc_rust_2024::io;

struct Map {
    data: Vec<Vec<u8>>,
    robot_pos: Position,
}

type Moves = Vec<u8>;
type Direction = u8;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Map {
    fn new(data: Vec<Vec<u8>>) -> Self {
        let (row, col) = data
            .iter()
            .enumerate()
            .filter(|(_, row)| row.contains(&b'@'))
            .map(|(i, row)| (i, row.iter().position(|&c| c == b'@').unwrap()))
            .next()
            .unwrap();
        Self {
            data,
            robot_pos: Position { row, col },
        }
    }

    fn iter_positions_in_front_of_robot(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = (Position, u8)> + '_ {
        let (height, width) = (self.data.len(), self.data[0].len());

        let it: Box<dyn Iterator<Item = _>> = match direction {
            b'v' => Box::new((self.robot_pos.row..height).zip(iter::repeat(self.robot_pos.col))),
            b'^' => Box::new(
                (0..=self.robot_pos.row)
                    .rev()
                    .zip(iter::repeat(self.robot_pos.col)),
            ),
            b'>' => Box::new(iter::repeat(self.robot_pos.row).zip(self.robot_pos.col..width)),
            b'<' => Box::new(iter::repeat(self.robot_pos.row).zip((0..=self.robot_pos.col).rev())),
            _ => panic!("Unexpected direction!"),
        };
        it.map(|(row, col)| (Position { row, col }, self.data[row][col]))
            .skip(1)
    }

    fn do_move(&mut self, direction: Direction) {
        let next_clear = self
            .iter_positions_in_front_of_robot(direction)
            .take_while(|(_, val)| *val != b'#')
            .find(|(_, val)| *val == b'.');
        if let Some((next_clear_pos, _)) = next_clear {
            let (robot_next, _) = self
                .iter_positions_in_front_of_robot(direction)
                .next()
                .unwrap();
            self.data[self.robot_pos.row][self.robot_pos.col] = b'.';
            self.data[robot_next.row][robot_next.col] = b'@';
            if next_clear_pos != robot_next {
                self.data[next_clear_pos.row][next_clear_pos.col] = b'O';
            }
            self.robot_pos = robot_next;
        }
    }

    fn move_touching_boxes_vert(&mut self, direction: Direction) {
        let mut stack = vec![self.robot_pos];
        let mut positions_to_move = vec![];
        let mut seen = HashSet::new();
        let step: i32 = if direction == b'^' { -1 } else { 1 };

        while let Some(pos) = stack.pop() {
            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);
            positions_to_move.push(pos);

            let next_row = (pos.row as i32 + step) as usize;
            let c = self.data[next_row][pos.col];
            if c == b'#' {
                return;
            } else if c == b']' {
                stack.push(Position {
                    row: next_row,
                    col: pos.col - 1,
                });
                stack.push(Position {
                    row: next_row,
                    col: pos.col,
                });
            } else if c == b'[' {
                stack.push(Position {
                    row: next_row,
                    col: pos.col,
                });
                stack.push(Position {
                    row: next_row,
                    col: pos.col + 1,
                });
            }
        }

        positions_to_move.sort_by(|a, b| ((b.row as i32 - a.row as i32) * step).cmp(&0));
        for pos in positions_to_move {
            self.data[(pos.row as i32 + step) as usize][pos.col] = self.data[pos.row][pos.col];
            self.data[pos.row][pos.col] = b'.';
        }
        self.robot_pos.row = (self.robot_pos.row as i32 + step) as usize;
    }

    fn move_touching_boxes_horiz(&mut self, direction: Direction) {
        let mut stack = vec![self.robot_pos.col];
        let step: i32 = if direction == b'<' { -1 } else { 1 };
        let row = self.robot_pos.row;

        loop {
            let next_col = (*stack.last().unwrap() as i32 + step) as usize;
            if self.data[row][next_col] == b'#' {
                return;
            } else if self.data[row][next_col] == b'.' {
                break;
            } else {
                stack.push(next_col);
            }
        }

        while let Some(col) = stack.pop() {
            self.data[row][(col as i32 + step) as usize] = self.data[row][col];
            self.data[row][col] = b'.';
        }
        self.robot_pos.col = (self.robot_pos.col as i32 + step) as usize;
    }

    fn do_move_wide(&mut self, direction: Direction) {
        let next = self.iter_positions_in_front_of_robot(direction).next();
        if let Some((in_front_position, val)) = next {
            if val == b'.' {
                self.data[self.robot_pos.row][self.robot_pos.col] = b'.';
                self.data[in_front_position.row][in_front_position.col] = b'@';
                self.robot_pos = in_front_position;
            } else if val == b'#' {
                return;
            } else if matches!(direction, b'^' | b'v') {
                self.move_touching_boxes_vert(direction);
            } else {
                self.move_touching_boxes_horiz(direction);
            }
        }
    }

    fn _print_map(&self) {
        for line in &self.data {
            println!("{}", String::from_utf8(line.clone()).unwrap());
        }
    }
}

fn parse_input(wide: bool) -> (Map, Moves) {
    let input = io::get_puzzle_input(15);
    let input = input.trim();

    let (map, moves) = input.split_once("\n\n").unwrap();

    let char_to_wide = |c: char| match c {
        '#' => "##",
        'O' => "[]",
        '.' => "..",
        '@' => "@.",
        _ => panic!("Unepxected char!"),
    };

    let map = if wide {
        map.trim()
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| char_to_wide(c).chars())
                    .map(|c| c as u8)
                    .collect()
            })
            .collect()
    } else {
        map.trim()
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect()
    };

    let moves = moves
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c as u8);

    (Map::new(map), moves.collect())
}

fn part1_solution() -> u32 {
    let (mut map, moves) = parse_input(false);

    for dir in moves {
        map.do_move(dir);
    }

    let mut gps_total = 0;
    for (i, row) in map.data.into_iter().enumerate() {
        for (j, val) in row.into_iter().enumerate() {
            if val == b'O' {
                gps_total += i * 100 + j;
            }
        }
    }
    gps_total as u32
}

fn part2_solution() -> u32 {
    let (mut map, moves) = parse_input(true);
    for dir in moves {
        map.do_move_wide(dir);
    }

    let mut gps_total = 0;
    for (i, row) in map.data.into_iter().enumerate() {
        for (j, val) in row.into_iter().enumerate() {
            if val == b'[' {
                gps_total += i * 100 + j;
            }
        }
    }
    gps_total as u32
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
