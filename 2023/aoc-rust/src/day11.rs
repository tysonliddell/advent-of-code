use std::fmt::Display;

use itertools::Itertools;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

type GalPosition = (usize, usize);

struct Galaxies {
    data: Vec<Vec<char>>,
}

impl From<&str> for Galaxies {
    fn from(value: &str) -> Self {
        let data = value.trim().lines().map(|l| l.chars().collect()).collect();
        Self { data }
    }
}

impl Display for Galaxies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .data
            .iter()
            .map(|r| r.iter().collect::<String>())
            .join("\n");
        write!(f, "{}", s)
    }
}

impl Galaxies {
    fn height(&self) -> usize {
        self.data.len()
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn expand(&mut self) {
        let mut new_data = vec![];

        // expand rows
        for row in self.data.iter() {
            new_data.push(row.clone());
            if row.iter().filter(|&&c| c != '.').count() == 0 {
                new_data.push(row.clone());
            }
        }
        self.data = new_data;

        // expand cols
        let mut new_data: Vec<Vec<char>> = vec![vec![]; self.height()];
        let mut copy_col = |col: &Vec<char>| {
            for (row, &new_c) in new_data.iter_mut().zip(col) {
                row.push(new_c);
            }
        };
        for c in 0..self.width() {
            let col = self.data.iter().map(|r| r[c]).collect();
            copy_col(&col);
            if self.data.iter().map(|r| r[c]).filter(|&c| c != '.').count() == 0 {
                copy_col(&col);
            }
        }
        self.data = new_data;
    }

    fn galaxy_positions(&self) -> Vec<GalPosition> {
        let mut positions = vec![];
        for (r, row) in self.data.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                if val == '#' {
                    positions.push((r, c));
                }
            }
        }
        positions
    }

    fn empty_rows(&self) -> Vec<usize> {
        let mut res = vec![];
        for (r, row) in self.data.iter().enumerate() {
            if row.iter().filter(|&&c| c == '#').count() == 0 {
                res.push(r);
            }
        }
        res
    }

    fn empty_cols(&self) -> Vec<usize> {
        let mut res = vec![];
        for c in 0..self.height() {
            let col = self.data.iter().map(|r| r[c]).collect_vec();
            if col.iter().filter(|&&c| c == '#').count() == 0 {
                res.push(c);
            }
        }
        res
    }

    fn shortest_path_lengths(&self) -> Vec<usize> {
        let mut res = vec![];
        let positions = self.galaxy_positions();
        for i in 0..positions.len() {
            let start_pos = &positions[i];
            for end_pos in &positions[i + 1..positions.len()] {
                res.push(
                    ((start_pos.0 as i32 - end_pos.0 as i32).abs()
                        + (start_pos.1 as i32 - end_pos.1 as i32).abs())
                        as usize,
                );
            }
        }
        res
    }

    fn shortest_path_lengths_after_expansion(&self, expansion_length: usize) -> Vec<usize> {
        let mut res = vec![];
        let positions = self.galaxy_positions();
        let empty_rows = self.empty_rows();
        let empty_cols = self.empty_cols();
        for i in 0..positions.len() {
            let start_pos = &positions[i];
            for end_pos in &positions[i + 1..positions.len()] {
                let (min_r, max_r) = (start_pos.0.min(end_pos.0), start_pos.0.max(end_pos.0));
                let (min_c, max_c) = (start_pos.1.min(end_pos.1), start_pos.1.max(end_pos.1));
                let empty_rows_crossed = empty_rows
                    .iter()
                    .filter(|r| (min_r..max_r).contains(r))
                    .count();
                let empty_cols_crossed = empty_cols
                    .iter()
                    .filter(|c| (min_c..max_c).contains(c))
                    .count();

                let mut row_distance =
                    (start_pos.0 as i32 - end_pos.0 as i32).unsigned_abs() as usize;
                row_distance += empty_rows_crossed * (expansion_length - 1);

                let mut col_distance =
                    (start_pos.1 as i32 - end_pos.1 as i32).unsigned_abs() as usize;
                col_distance += empty_cols_crossed * (expansion_length - 1);

                res.push(row_distance + col_distance);
            }
        }
        res
    }
}

fn part1() {
    let input = include_str!("../../puzzle_input/d11").trim();
    let mut galaxies = Galaxies::from(input);
    galaxies.expand();
    println!("{}", galaxies.shortest_path_lengths().iter().sum::<usize>());
}

fn part2() {
    let input = include_str!("../../puzzle_input/d11").trim();
    let galaxies = Galaxies::from(input);
    println!(
        "{}",
        galaxies
            .shortest_path_lengths_after_expansion(1000000)
            .iter()
            .sum::<usize>()
    );
}
