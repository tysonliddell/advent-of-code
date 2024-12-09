use std::iter;

use aoc_rust_2024::io;

type FileSystem = Vec<Option<usize>>;

fn parse_disk_map(disk_map: &str) -> FileSystem {
    let block_size_iter = disk_map.chars().step_by(2);
    let free_space_iter = disk_map.chars().skip(1).step_by(2).chain(iter::once('0'));

    let mut fs = Vec::new();
    for (block_id, (block_size, free_space)) in block_size_iter.zip(free_space_iter).enumerate() {
        let block_size = block_size.to_digit(10).unwrap() as usize;
        let free_space = free_space.to_digit(10).unwrap() as usize;
        fs.extend(iter::repeat(Some(block_id)).take(block_size));
        fs.extend(iter::repeat(None).take(free_space));
    }
    fs
}

fn defrag(fs: &mut FileSystem) {
    let advance = |mut start: usize, mut end: usize, fs: &FileSystem| {
        while fs[start].is_some() {
            start += 1;
        }
        while fs[end].is_none() {
            end -= 1;
        }
        (start, end)
    };

    let (mut start, mut end) = advance(0, fs.len() - 1, fs);

    // invariant: `start` is on free space, `end` is on a block
    while end > start {
        fs.swap(start, end);
        (start, end) = advance(start, end, fs);
    }
}

fn defrag_and_compact(fs: &mut FileSystem) {
    // invariant: `end` is one past the end of unprocessed data
    let mut end = fs.len();
    while end > 0 {
        end -= 1;

        // skip free space at end of unprocessed data
        while end > 0 && fs[end].is_none() {
            end -= 1;
        }

        // scan file
        let file_end = end;
        let file_id = fs[end];
        while end > 0 && fs[end - 1] == file_id {
            end -= 1;
        }
        let file_start = end;
        let file_size = file_end - file_start + 1;

        // find free space near start
        let mut i = 0;
        while i < file_start {
            while i < file_start && fs[i].is_some() {
                i += 1;
            }

            let free_start = i;
            while fs[i].is_none() {
                i += 1;
            }
            let free_size = i - free_start;
            if free_size >= file_size {
                fs.copy_within(file_start..=file_end, free_start);
                fs[file_start..=file_end].fill(None);
                break;
            }
        }
    }
}

fn checksum(fs: &FileSystem) -> usize {
    fs.iter()
        .enumerate()
        .filter_map(|(pos, &id)| id.map(|id| (pos, id)))
        .fold(0, |total, (pos, id)| total + pos * id)
}

fn part1_solution() -> usize {
    let input = io::get_puzzle_input(9);
    let input = input.trim();

    let mut fs = parse_disk_map(input);
    defrag(&mut fs);
    checksum(&fs)
}

fn part2_solution() -> usize {
    let input = io::get_puzzle_input(9);
    let input = input.trim();

    let mut fs = parse_disk_map(input);
    defrag_and_compact(&mut fs);
    checksum(&fs)
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
