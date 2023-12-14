use itertools::Itertools;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

fn parse_rocks() -> Vec<Vec<char>> {
    let input = include_str!("../../puzzle_input/d14").trim();
    let rocks = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    rocks
}

fn tilt_north(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = rocks.len();
    let width = rocks[0].len();

    let mut res = vec![];
    for _ in 0..height {
        res.push(".".repeat(width).chars().collect_vec())
    }

    for col_i in 0..width {
        let mut curr_cube = -1;
        let col: String = rocks.iter().map(|r| r[col_i]).collect();

        let mut new_col = String::new();
        while curr_cube < height as i32 {
            // find next cube
            let next_cube_distance = col[(curr_cube + 1) as usize..].find('#');
            let next_cube = if let Some(dist) = next_cube_distance {
                curr_cube + 1 + dist as i32
            } else {
                height as i32
            };

            // count round rocks between curr_cube and next cube
            let round_rock_count = col[(curr_cube + 1) as usize..next_cube as usize]
                .chars()
                .filter(|&c| c == 'O')
                .count();
            let gaps_count = next_cube - curr_cube - 1 - round_rock_count as i32;

            // put those rocks against curr_cube
            new_col.push_str(&"O".repeat(round_rock_count));
            new_col.push_str(&".".repeat(gaps_count as usize));
            if next_cube < height as i32 {
                new_col.push('#');
            }

            // set curr_cube = next cube
            curr_cube = next_cube;
        }

        for (i, ch) in new_col.char_indices() {
            res[i][col_i] = ch;
        }
    }

    res
}

fn rotate_clockwise(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = rocks.len();
    let width = rocks[0].len();

    let mut res = vec![];
    for _ in 0..height {
        res.push(".".repeat(width).chars().collect_vec())
    }

    #[allow(clippy::needless_range_loop)]
    for new_row in 0..height {
        for new_col in 0..width {
            let old_row = height - new_col - 1;
            let old_col = new_row;
            res[new_row][new_col] = rocks[old_row][old_col];
        }
    }

    res
}

fn spin(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = rotate_clockwise(&tilt_north(rocks));
    res = rotate_clockwise(&tilt_north(&res));
    res = rotate_clockwise(&tilt_north(&res));
    rotate_clockwise(&tilt_north(&res))
}

fn part1() {
    let rocks = parse_rocks();
    let rocks = tilt_north(&rocks);

    let mut total = 0;
    for (distance, rocks) in rocks.iter().rev().enumerate() {
        total += (rocks.iter().filter(|&&c| c == 'O').count()) * (distance + 1);
    }
    println!("{}", total);
}

fn part2() {
    let mut history = vec![parse_rocks()];

    let cycle_start = loop {
        let spun = spin(history.last().unwrap());
        let pos = history.iter().position(|r| r == &spun);

        if let Some(pos) = pos {
            break pos;
        }

        history.push(spun);
    };

    let cycle_size = history.len() - cycle_start;
    let final_cycle_pos = (1000000000 - cycle_start) % cycle_size;
    let rocks = &history[cycle_start + final_cycle_pos];

    let mut total = 0;
    for (distance, rocks) in rocks.iter().rev().enumerate() {
        total += (rocks.iter().filter(|&&c| c == 'O').count()) * (distance + 1);
    }
    println!("{}", total);
}
