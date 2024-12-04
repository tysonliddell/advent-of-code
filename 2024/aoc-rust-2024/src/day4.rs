use std::collections::HashMap;

use crate::{io, Solution};

pub struct Day4;

// Return an iterator of char iterators. Each char iterator is a row.
fn iter_rows<'a>(
    data: &'a Vec<&str>,
) -> impl Iterator<Item = impl DoubleEndedIterator<Item = char> + 'a> {
    data.iter().map(|s| s.chars())
}

// Return an iterator of char iterators. Each char iterator is a column.
fn iter_cols<'a>(
    data: &'a Vec<&str>,
) -> impl Iterator<Item = impl DoubleEndedIterator<Item = char> + 'a> {
    let height = data.len();
    let width = data[0].len();

    (0..width).map(move |col| {
        (0..height).map(move |row| char::from_u32(data[row].as_bytes()[col] as u32).unwrap())
    })
}

// Return an iterator of char iterators. Each char iterator is a diganonal.
fn iter_diags<'a>(
    data: &'a Vec<&str>,
) -> impl Iterator<Item = impl DoubleEndedIterator<Item = char> + 'a> + 'a {
    let height = data.len();
    let width = data[0].len();

    type Position = (usize, usize);
    let mut diags_nw_to_se: HashMap<i32, Vec<Position>> = HashMap::new();
    let mut diags_ne_to_sw: HashMap<i32, Vec<Position>> = HashMap::new();

    // Use `Vec`s to store all positions for the same diagonal, in order.
    for row in 0..height as i32 {
        for col in 0..width as i32 {
            diags_nw_to_se
                .entry(row - col) // row-col is constant on each NW->SE diagonal
                .or_default()
                .push((row as usize, col as usize));
            diags_ne_to_sw
                .entry(row + col) // row+col is constant on each NE->SW diagonal
                .or_default()
                .push((row as usize, col as usize));
        }
    }

    let diags_nw_to_se = diags_nw_to_se.into_values();
    let diags_ne_to_sw = diags_ne_to_sw.into_values();

    diags_nw_to_se.chain(diags_ne_to_sw).map(|diag| {
        diag.into_iter()
            .map(|(row, col)| data[row].as_bytes()[col].into())
    })
}

fn get_xmas_count(chars: impl Iterator<Item = char>) -> usize {
    chars.collect::<String>().match_indices("XMAS").count()
}

impl Solution for Day4 {
    fn part1_solution(&self) -> String {
        let input = io::get_puzzle_input(4);
        let data: Vec<_> = input.lines().collect();

        let count_rows: usize = iter_rows(&data).map(get_xmas_count).sum();
        let count_rows_reverse: usize = iter_rows(&data)
            .map(|chars| chars.rev())
            .map(get_xmas_count)
            .sum();

        let count_cols: usize = iter_cols(&data).map(get_xmas_count).sum();
        let count_cols_reverse: usize = iter_cols(&data)
            .map(|chars| chars.rev())
            .map(get_xmas_count)
            .sum();

        let count_diags: usize = iter_diags(&data).map(get_xmas_count).sum();
        let count_diags_reverse: usize = iter_diags(&data)
            .map(|chars| chars.rev())
            .map(get_xmas_count)
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
