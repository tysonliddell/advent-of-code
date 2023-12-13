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
    cols: Vec<u32>, // vector of column bit strings.
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let rows = value.trim().split('\n').collect_vec();
        let width = rows[0].len();

        let row_bitstrings = rows.into_iter().map(str_to_bitstring).collect_vec();
        let col_bitstrings = (0..width)
            .rev() // puzzle numbers columns msb to lsb (first col is msb)
            .map(|c| {
                row_bitstrings
                    .iter()
                    .map(|r| if r & (1 << c) == 0 { 0 } else { 1 })
                    .fold(0, |acc, v| (acc << 1) | v)
            })
            .collect_vec();

        Self {
            rows: row_bitstrings,
            cols: col_bitstrings,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .rows
            .iter()
            .map(|r| format!("{:0width$b}", r, width = self.cols.len()))
            .join("\n");
        write!(f, "{}", s)
    }
}

impl Board {
    fn index_of_symmetry(&self) -> usize {
        if let Some(val) = get_line_of_symmetry(&self.rows) {
            100 * val
        } else {
            get_line_of_symmetry(&self.cols).expect("Should find a line of symmetry!")
        }
    }

    fn index_of_smudged_symmetry(&self) -> usize {
        if let Some(val) = get_smudged_line_of_symmetry(&self.rows) {
            100 * val
        } else {
            get_smudged_line_of_symmetry(&self.cols)
                .expect("Should find a line of smudged symmetry!")
        }
    }
}

fn get_line_of_symmetry(bitstrings: &Vec<u32>) -> Option<usize> {
    let is_line_of_symmetry = |line_num| {
        let rows_before = bitstrings.iter().rev().skip(bitstrings.len() - line_num);
        let rows_after = bitstrings.iter().skip(line_num);
        for (row_before, row_after) in rows_before.zip(rows_after) {
            if row_before != row_after {
                return false;
            }
        }
        true
    };

    (1..bitstrings.len()).find(|&line| is_line_of_symmetry(line))
}

fn get_smudged_line_of_symmetry(bitstrings: &Vec<u32>) -> Option<usize> {
    let is_smudged_line_of_symmetry = |line_num| {
        let rows_before = bitstrings.iter().rev().skip(bitstrings.len() - line_num);
        let rows_after = bitstrings.iter().skip(line_num);

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

    (1..bitstrings.len()).find(|&line| is_smudged_line_of_symmetry(line))
}

fn parse_boards() -> Vec<Board> {
    let input = include_str!("../../puzzle_input/d13").trim();
    let boards = input.split("\n\n").map_into().collect_vec();
    boards
}

fn part1() {
    let boards = parse_boards();
    let total = boards.iter().map(|b| b.index_of_symmetry()).sum::<usize>();
    println!("{}", total);
}

fn part2() {
    let boards = parse_boards();
    let total = boards
        .iter()
        .map(|b| b.index_of_smudged_symmetry())
        .sum::<usize>();
    println!("{}", total);
}
