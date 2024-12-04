use core::str;
use std::collections::HashMap;

use crate::{io, Solution};

pub struct Day4;

fn iter_rows<'a>(data: &'a Vec<&str>) -> impl Iterator<Item = String> + 'a {
    data.iter().map(|s| s.to_string())
}

fn iter_cols<'a>(data: &'a Vec<&str>) -> impl Iterator<Item = String> + 'a {
    let height = data.len();
    let width = data[0].len();

    (0..width).map(move |col| {
        (0..height)
            .map(move |row| char::from_u32(data[row].as_bytes()[col] as u32).unwrap())
            .collect()
    })
}

fn iter_diags(data: &Vec<&str>) -> impl Iterator<Item = String> {
    let height = data.len();
    let width = data[0].len();

    let mut diags_nw_to_se: HashMap<i32, String> = HashMap::new();
    let mut diags_ne_to_sw: HashMap<i32, String> = HashMap::new();

    for row in 0..height as i32 {
        for col in 0..width as i32 {
            let c: char = data[row as usize].as_bytes()[col as usize].into();
            diags_nw_to_se
                .entry(row - col) // row-col is constant on each NW->SE diagonal
                .or_default()
                .push(c);
            diags_ne_to_sw
                .entry(row + col) // row+col is constant on each NE->SW diagonal
                .or_default()
                .push(c);
        }
    }

    let diags_nw_to_se = diags_nw_to_se.into_values();
    let diags_ne_to_sw = diags_ne_to_sw.into_values();

    diags_nw_to_se.chain(diags_ne_to_sw)
}

fn get_xmas_count(s: &str) -> usize {
    s.match_indices("XMAS").count()
}

impl Solution for Day4 {
    fn part1_solution(&self) -> String {
        let input = io::get_puzzle_input(4);
        let data: Vec<_> = input.lines().collect();

        let rows_cols_and_diags = iter_rows(&data)
            .chain(iter_cols(&data))
            .chain(iter_diags(&data));

        let mut total = 0;
        for slice in rows_cols_and_diags {
            let slice_rev: String = slice.chars().rev().collect();
            total += get_xmas_count(&slice) + get_xmas_count(&slice_rev);
        }

        total.to_string()
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
