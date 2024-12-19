use std::collections::{HashSet, VecDeque};

use aoc_rust_2024::io;
use itertools::Itertools;
use partitions::{partition_vec, PartitionVec};

type BytePosition = (usize, usize);

const MAX_MEM_ROW: usize = 70;
const MAX_MEM_COL: usize = 70;

fn parse_input() -> Vec<BytePosition> {
    let input = io::get_puzzle_input(18);
    let input = input.trim();

    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

fn get_neighbours(pos: BytePosition, max_row: usize, max_col: usize) -> Vec<BytePosition> {
    let (row, col) = pos;
    let nbrs = vec![
        (row as i32 - 1, col as i32),
        (row as i32 + 1, col as i32),
        (row as i32, col as i32 - 1),
        (row as i32, col as i32 + 1),
    ];
    nbrs.into_iter()
        .filter(|(row, col)| {
            (0..=max_row as i32).contains(row) && (0..=max_col as i32).contains(col)
        })
        .map(|(row, col)| (row as usize, col as usize))
        .collect()
}

fn get_shortest_path_length(
    fallen_bytes: &[BytePosition],
    start: BytePosition,
    end: BytePosition,
    max_row: usize,
    max_col: usize,
) -> Option<usize> {
    let corrupted_bytes: HashSet<(usize, usize)> = HashSet::from_iter(fallen_bytes.iter().copied());

    let mut visited = HashSet::new();
    let mut q = VecDeque::from([(0, start)]);

    while let Some((distance, pos)) = q.pop_front() {
        if pos == end {
            return Some(distance);
        }

        let noncorrupted_neighbours = get_neighbours(pos, max_row, max_col)
            .into_iter()
            .filter(|pos| !corrupted_bytes.contains(pos));

        for n in noncorrupted_neighbours {
            if !visited.contains(&n) {
                visited.insert(n);
                q.push_back((distance + 1, n));
            }
        }
    }

    None
}

fn part1_solution() -> usize {
    let positions = parse_input();
    let start = (0, 0);
    let end = (MAX_MEM_ROW, MAX_MEM_COL);

    get_shortest_path_length(&positions[..1024], start, end, MAX_MEM_ROW, MAX_MEM_COL).unwrap()
}

fn part2_solution() -> (usize, usize) {
    let positions = parse_input();
    let start = (0, 0);
    let end = (MAX_MEM_ROW, MAX_MEM_COL);

    for i in 1024..10000 {
        if get_shortest_path_length(&positions[..i], start, end, MAX_MEM_ROW, MAX_MEM_COL).is_none()
        {
            return positions[i - 1];
        }
    }

    panic!("No solution found!");
}

fn get_9_cell_square(pos: BytePosition, max_row: usize, max_col: usize) -> Vec<BytePosition> {
    let (row, col) = (pos.0 as i32, pos.1 as i32);
    let nbrs = (row - 1..=row + 1).cartesian_product(col - 1..=col + 1);
    nbrs.into_iter()
        .filter(|(row, col)| {
            (0..=max_row as i32).contains(row) && (0..=max_col as i32).contains(col)
        })
        .map(|(row, col)| (row as usize, col as usize))
        .collect()
}

// uses a union-find data structure
fn part2_solution_fast() -> (usize, usize) {
    let positions = parse_input();
    let mut cells: PartitionVec<bool> = partition_vec![false; (MAX_MEM_ROW+1) * (MAX_MEM_COL+1)];

    for byte_pos in positions {
        let new_id = byte_pos.0 * (MAX_MEM_COL + 1) + byte_pos.1;
        if cells[new_id] {
            continue; // we've already added this falling byte, nothing to do
        }

        cells[new_id] = true;

        // Merge cells that are touching the new position
        for (row, col) in get_9_cell_square(byte_pos, MAX_MEM_ROW, MAX_MEM_COL) {
            let id = row * (MAX_MEM_COL + 1) + col;
            if cells[id] {
                cells.union(new_id, id);
            }
        }

        for region in cells.all_sets() {
            let (mut min_row, mut max_row) = (MAX_MEM_ROW, 0);
            let (mut min_col, mut max_col) = (MAX_MEM_COL, 0);

            for (index, _) in region {
                let row = index / (MAX_MEM_COL + 1);
                let col = index % (MAX_MEM_COL + 1);
                min_row = min_row.min(row);
                max_row = max_row.max(row);
                min_col = min_col.min(col);
                max_col = max_col.max(col);
            }
            if min_row == 0 && max_row == MAX_MEM_ROW {
                return byte_pos;
            }
        }
    }

    panic!("No solution found!");
}

fn main() {
    println!("{}", part1_solution());
    // println!("{:?}", part2_solution());
    println!("{:?}", part2_solution_fast());
}
