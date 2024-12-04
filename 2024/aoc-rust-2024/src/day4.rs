use std::collections::HashMap;

use crate::{io, Solution};

pub struct Day4;

fn iter_rows<'a>(data: &'a Vec<&str>) -> impl Iterator<Item = &'a str> {
    data.iter().map(|s| *s)
}

fn iter_cols<'a>(data: &'a Vec<&str>) -> impl Iterator<Item = String> + 'a {
    let height = data.len();
    let width = data[0].len();
    let mut cols = vec![vec![0u8; height]; width];

    for i in 0..height {
        for j in 0..width {
            cols[j][i] = data[i].as_bytes()[j];
        }
    }

    cols.into_iter().map(|col| String::from_utf8(col).unwrap())
}

fn iter_diags<'a>(data: &'a Vec<&str>) -> impl Iterator<Item = String> + 'a {
    let height = data.len();
    let width = data[0].len();
    let mut diags_left_to_right: HashMap<i32, String> = HashMap::new();
    let mut diags_right_to_left: HashMap<i32, String> = HashMap::new();

    for i in 0..height as i32 {
        for j in 0..width as i32 {
            diags_left_to_right
                .entry(i - j)
                .or_default()
                .push(data[i as usize].as_bytes()[j as usize].into());
            diags_right_to_left
                .entry(i + j)
                .or_default()
                .push(data[i as usize].as_bytes()[j as usize].into());
        }
    }

    diags_left_to_right
        .into_values()
        .chain(diags_right_to_left.into_values())
}

fn get_xmas_count<'a>(chars: &'a str) -> usize {
    chars.match_indices("XMAS").count()
}

impl Solution for Day4 {
    fn part1_solution(&self) -> String {
        let input = io::get_puzzle_input(4);
        let data: Vec<_> = input.lines().collect();

        let count_rows: usize = iter_rows(&data).map(get_xmas_count).sum();
        let count_rows_reverse: usize = iter_rows(&data)
            .map(|s| {
                let rev: String = s.chars().rev().collect();
                get_xmas_count(&rev[..])
            })
            .sum();

        let count_cols: usize = iter_cols(&data).map(|s| get_xmas_count(&s[..])).sum();
        let count_cols_reverse: usize = iter_cols(&data)
            .map(|s| {
                let rev: String = s.chars().rev().collect();
                get_xmas_count(&rev[..])
            })
            .sum();

        let count_diags: usize = iter_diags(&data).map(|s| get_xmas_count(&s[..])).sum();
        let count_diags_reverse: usize = iter_diags(&data)
            .map(|s| {
                let rev: String = s.chars().rev().collect();
                get_xmas_count(&rev[..])
            })
            .sum();

        let result = count_rows
            + count_rows_reverse
            + count_cols
            + count_cols_reverse
            + count_diags
            + count_diags_reverse;

        result.to_string()
    }

    fn part2_solution(&self) -> String {
        let input = io::get_puzzle_input(4);
        let data: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();

        let height = data.len();
        let width = data[0].len();

        let is_x_mas = |(row, col): (usize, usize)| {
            if data[row][col] != b'A' {
                return false;
            }
            if (row == 0) || (col == 0) || (row == height - 1) || (col == width - 1) {
                return false;
            }

            let (tl, tr) = (data[row - 1][col - 1], data[row - 1][col + 1]);
            let (bl, br) = (data[row + 1][col - 1], data[row + 1][col + 1]);
            let has_diag_1 = (tl == b'M' && br == b'S') || (tl == b'S' && br == b'M');
            let has_diag_2 = (tr == b'M' && bl == b'S') || (tr == b'S' && bl == b'M');

            has_diag_1 && has_diag_2
        };

        let mut total = 0;
        for i in 0..height {
            for j in 0..width {
                if is_x_mas((i, j)) {
                    total += 1;
                }
            }
        }

        total.to_string()
    }
}
