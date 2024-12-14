use aoc_rust_2024::{helpers::make_counter, io};
use regex::Regex;

const TREE_MARKER: &str = "XXXXXXXXXX";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    start_pos: Vec2,
    velocity: Vec2,
}

impl Robot {
    fn get_new_pos(&self, time_secs: u32, corner: Vec2) -> Vec2 {
        let time_secs = time_secs as i32;
        let Vec2 { x: max_x, y: max_y } = corner;

        Vec2 {
            x: (self.start_pos.x + time_secs * self.velocity.x).rem_euclid(max_x),
            y: (self.start_pos.y + time_secs * self.velocity.y).rem_euclid(max_y),
        }
    }
}

fn parse_input() -> Vec<Robot> {
    let input = io::get_puzzle_input(14);
    let input = input.trim();

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    re.captures_iter(input)
        .map(|v| v.extract())
        .map(|(_, [px, py, vx, vy])| Robot {
            start_pos: Vec2 {
                x: px.parse().unwrap(),
                y: py.parse().unwrap(),
            },
            velocity: Vec2 {
                x: vx.parse().unwrap(),
                y: vy.parse().unwrap(),
            },
        })
        .collect()
}

fn position_to_quadrant(pos: Vec2, corner: Vec2) -> Option<u8> {
    let mid_x = corner.x / 2;
    let mid_y = corner.y / 2;
    if pos.x == mid_x || pos.y == mid_y {
        None
    } else if pos.x < mid_x && pos.y < mid_y {
        Some(1)
    } else if pos.x > mid_x && pos.y < mid_y {
        Some(2)
    } else if pos.x < mid_x && pos.y > mid_y {
        Some(3)
    } else {
        Some(4)
    }
}

// fn get_noise_score(lines: &Vec<String>) -> usize {
//     // a higher score means less noise
//     let mut score = 0;

//     for line in lines {
//         score += line.chars().filter(|&c| c == '.').count();
//     }
//     score
// }

fn print_robots(robots: &[Robot], time_secs: u32, corner: Vec2) -> bool {
    let Vec2 { x: max_x, y: max_y } = corner;

    let new_positions: Vec<Vec2> = robots
        .iter()
        .map(|r| r.get_new_pos(time_secs, corner))
        .collect();

    let counts = make_counter(new_positions.into_iter());
    let mut lines = Vec::new();
    let mut found = false;
    for y in 0..max_y {
        let line: String = (0..max_x)
            .map(|x| {
                if counts.contains_key(&Vec2 { x, y }) {
                    'X'
                } else {
                    '.'
                }
            })
            .collect();
        if line.contains(TREE_MARKER) {
            found = true;
        }
        lines.push(line);
    }

    if found {
        for line in lines {
            println!("{}", line);
        }
    }
    found
}

fn part1_solution() -> u32 {
    let robots = parse_input();
    let corner = Vec2 { x: 101, y: 103 };

    let new_positions: Vec<u8> = robots
        .into_iter()
        .map(|r| r.get_new_pos(100, corner))
        .filter_map(|pos| position_to_quadrant(pos, corner))
        .collect();

    let counts = make_counter(new_positions.into_iter());
    println!("{:?}", counts);
    let result: usize = counts.values().product();
    result as u32
}

fn part2_solution() -> u32 {
    let robots = parse_input();
    let corner = Vec2 { x: 101, y: 103 };

    for t in 1..101 * 103 {
        if print_robots(&robots, t, corner) {
            return t;
        }
    }
    panic!("NO SOLUTION!");
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
