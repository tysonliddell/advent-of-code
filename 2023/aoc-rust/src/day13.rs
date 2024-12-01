use itertools::Itertools;
use std::fmt::Display;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

fn str_to_bitstring(s: &str) -> u32 {
    s.chars()
        .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
}

struct Board {
    rows: Vec<u32>, // vector of row bit strings.
    width: usize,
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let rows = value.trim().split('\n').collect_vec();
        let width = rows[0].len();

        let row_bitstrings = rows.into_iter().map(str_to_bitstring).collect_vec();
        Self {
            rows: row_bitstrings,
            width,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .rows
            .iter()
            .map(|r| format!("{:0width$b}", r, width = self.width))
            .join("\n");
        write!(f, "{}", s)
    }
}

impl Board {
    fn row_of_symmetry(&self) -> Option<usize> {
        let rows = &self.rows;
        let is_line_of_symmetry = |line_num| {
            let rows_before = rows.iter().rev().skip(rows.len() - line_num);
            let rows_after = rows.iter().skip(line_num);
            for (row_before, row_after) in rows_before.zip(rows_after) {
                if row_before != row_after {
                    return false;
                }
            }
            true
        };

        (1..rows.len()).find(|&line| is_line_of_symmetry(line))
    }

    fn row_of_smudged_symmetry(&self) -> Option<usize> {
        let rows = &self.rows;
        let is_smudged_line_of_symmetry = |line_num| {
            let rows_before = rows.iter().rev().skip(rows.len() - line_num);
            let rows_after = rows.iter().skip(line_num);

            let mut seen_smudged_bit = false;
            for (row_before, row_after) in rows_before.zip(rows_after) {
                let num_bits_different = (row_before ^ row_after).count_ones();

                #[allow(clippy::comparison_chain)]
                if num_bits_different > 1 {
                    return false;
                } else if num_bits_different == 1 {
                    if seen_smudged_bit {
                        return false;
                    }
                    seen_smudged_bit = true;
                }
            }

            seen_smudged_bit
        };

        (1..rows.len()).find(|&line| is_smudged_line_of_symmetry(line))
    }

    fn rotate(&self) -> Self {
        let col_bitstrings = (0..self.width)
            .rev() // puzzle numbers columns msb to lsb (first col is msb)
            .map(|c| {
                self.rows
                    .iter()
                    .map(|r| if r & (1 << c) == 0 { 0 } else { 1 })
                    .fold(0, |acc, v| (acc << 1) | v)
            })
            .collect_vec();
        Self {
            rows: col_bitstrings,
            width: self.rows.len(),
        }
    }
}

fn parse_boards() -> Vec<Board> {
    let input = include_str!("../../puzzle_input/d13").trim();
    let boards = input.split("\n\n").map_into().collect_vec();
    boards
}

fn part1() {
    let boards = parse_boards();
    let mut total = 0;
    for board in boards {
        if let Some(row) = board.row_of_symmetry() {
            total += 100 * row;
        } else if let Some(row) = board.rotate().row_of_symmetry() {
            total += row;
        } else {
            panic!("No line of symmetry found!");
        }
    }
    println!("{}", total);
}

fn part2() {
    let boards = parse_boards();
    let mut total = 0;
    for board in boards {
        if let Some(row) = board.row_of_smudged_symmetry() {
            total += 100 * row;
        } else if let Some(row) = board.rotate().row_of_smudged_symmetry() {
            total += row;
        } else {
            panic!("No line of smudged symmetry found!");
        }
    }
    println!("{}", total);
}
