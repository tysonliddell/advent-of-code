use aoc_rust_2024::io;
use itertools::Itertools;

#[derive(Debug)]
struct Game {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn parse_input() -> Vec<Game> {
    let input = io::get_puzzle_input(13);
    let input = input.trim();

    let mut games = Vec::new();
    for game in input.split("\n\n") {
        let (a, b, prize): (&str, &str, &str) = game.lines().collect_tuple().unwrap();
        let a: (u64, u64) = a
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|s| s.split_once('+').unwrap().1.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let b: (u64, u64) = b
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|s| s.split_once('+').unwrap().1.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let prize: (u64, u64) = prize
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|s| s.split_once('=').unwrap().1.parse().unwrap())
            .collect_tuple()
            .unwrap();
        games.push(Game {
            button_a: a,
            button_b: b,
            prize,
        });
    }

    games
}

fn get_min_tokens(game: &Game) -> Option<u64> {
    let (a, b, c, d) = (
        game.button_a.0 as i64,
        game.button_a.1 as i64,
        game.button_b.0 as i64,
        game.button_b.1 as i64,
    );
    let det = a * d - c * b;
    assert_ne!(det, 0, "zero determinant!");

    let (y1, y2) = (game.prize.0 as i64, game.prize.1 as i64);

    if (d * y1 - c * y2) % det != 0 || (-b * y1 + a * y2) % det != 0 {
        return None;
    }

    let t1 = (d * y1 - c * y2) / det;
    let t2 = (-b * y1 + a * y2) / det;

    assert!(t1 > 0);
    assert!(t2 > 0);

    Some((t1 * 3 + t2) as u64)
}

fn part1_solution() -> u64 {
    let games = parse_input();
    games.into_iter().flat_map(|g| get_min_tokens(&g)).sum()
}

fn part2_solution() -> u64 {
    const PART_2_OFFSET: u64 = 10000000000000;
    let mut games = parse_input();
    for game in &mut games {
        game.prize.0 += PART_2_OFFSET;
        game.prize.1 += PART_2_OFFSET;
    }
    games.into_iter().flat_map(|g| get_min_tokens(&g)).sum()
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
