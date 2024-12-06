use aoc_rust_2024::io;

type Report = Vec<u32>;

fn parse_input() -> Vec<Vec<u32>> {
    let input = io::get_puzzle_input(2);
    let input = input.trim();

    input
        .lines()
        .map(|line| line.split(' ').map(|val| val.parse().unwrap()).collect())
        .collect()
}

fn is_safe(report: &Report) -> bool {
    let sign = (report[1] as i32 - report[0] as i32).signum();

    report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(&curr, &next)| {
            let difference = next as i32 - curr as i32;
            difference.signum() == sign && (1..=3).contains(&difference.abs())
        })
}

fn is_safe_with_dampner(report: &Report) -> bool {
    (0..report.len()).any(|index_to_skip| {
        let skipped: Vec<_> = report
            .iter()
            .enumerate()
            .filter_map(|(i, &val)| if i != index_to_skip { Some(val) } else { None })
            .collect();
        is_safe(&skipped)
    })
}

fn part1_solution() -> u32 {
    let reports = parse_input();

    reports.into_iter().filter(is_safe).count() as u32
}

fn part2_solution() -> u32 {
    let reports = parse_input();

    reports.into_iter().filter(is_safe_with_dampner).count() as u32
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
