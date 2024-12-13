use aoc_rust_2024::io;
use regex::Regex;

#[derive(Debug)]
struct Game {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn parse_input() -> Vec<Game> {
    let input = io::get_puzzle_input(13);
    let input = input.trim();

    let number_pair_regex = Regex::new(r"(\d+)\D+(\d+)").unwrap();

    let mut games = Vec::new();
    for game in input.split("\n\n") {
        let number_pairs: Vec<(u64, u64)> = number_pair_regex
            .captures_iter(game)
            .map(|v| v.extract())
            .map(|(_, [v1, v2])| (v1.parse().unwrap(), v2.parse().unwrap()))
            .collect();
        games.push(Game {
            button_a: number_pairs[0],
            button_b: number_pairs[1],
            prize: number_pairs[2],
        });
    }

    games
}

fn get_min_tokens(game: &Game) -> Option<u64> {
    // We want to solve the linear system:
    //
    //  t1*a + t2*b = y1
    //  t1*c + t2*d = y2
    //
    // with positive integer solutions in y1,y2. This can be represented by the (matrix)
    // equation:
    //
    //    [t1 t2] [a b] = [y1 y2]
    //            [c d]
    //
    // which can be solved explicitly with
    //
    //     [t1 t2] = [y1 y2][a b]^-1
    //                      [c d]
    //             = [y1 y2]([ d -b] / det)
    //                      ([-c  a]      )
    //
    // We are exploiting the fact both row vectors (buttons) in each "game" are linearly
    // independent, resulting in a non-zero determinant and a uniqe solution.
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
        // can't use this determinant, would result in non-integer solution for t1 or t2
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
