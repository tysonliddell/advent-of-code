use crate::{io, Solution};

pub struct Day2;

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

impl Solution for Day2 {
    fn part1_solution(&self) -> String {
        let reports = parse_input();

        let result = reports.into_iter().filter(is_safe).count();
        result.to_string()
    }

    fn part2_solution(&self) -> String {
        let reports = parse_input();

        let result = reports.into_iter().filter(is_safe_with_dampner).count();
        result.to_string()
    }
}
