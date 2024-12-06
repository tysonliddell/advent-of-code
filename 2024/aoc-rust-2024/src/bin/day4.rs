use std::collections::HashMap;

use aoc_rust_2024::io;

type WordSearch<'a> = Vec<&'a str>;

fn get_nth_char(s: &str, n: usize) -> Option<char> {
    char::from_u32(s.as_bytes()[n] as u32)
}

fn iter_rows<'a>(data: &'a WordSearch) -> impl Iterator<Item = String> + 'a {
    data.iter().map(|s| s.to_string())
}

fn iter_cols<'a>(data: &'a WordSearch) -> impl Iterator<Item = String> + 'a {
    let height = data.len();
    let width = data[0].len();

    (0..width).map(move |col| {
        (0..height)
            .map(|row| get_nth_char(data[row], col).unwrap())
            .collect()
    })
}

fn iter_diags(data: &WordSearch) -> impl Iterator<Item = String> {
    let height = data.len();
    let width = data[0].len();

    let mut diags_nw_to_se: HashMap<i32, String> = HashMap::new();
    let mut diags_ne_to_sw: HashMap<i32, String> = HashMap::new();

    for row in 0..height as i32 {
        for col in 0..width as i32 {
            let c = get_nth_char(data[row as usize], col as usize).unwrap();
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

fn part1_solution() -> u32 {
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

    total as u32
}

fn part2_solution() -> u32 {
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

    total as u32
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
