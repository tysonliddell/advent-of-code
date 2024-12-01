use itertools::Itertools;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

type History = Vec<i64>;

fn parse_histories() -> Vec<History> {
    let input = include_str!("../../puzzle_input/d9").trim();
    let histories = input
        .lines()
        .map(|l| l.split(' ').map(|v| v.parse().unwrap()).collect_vec());
    histories.collect()
}

fn predict_future(history: &History) -> i64 {
    let diffs = history.windows(2).map(|w| w[1] - w[0]).collect_vec();
    let last = history.last().unwrap();

    if let Ok(&val) = diffs.iter().all_equal_value() {
        last + val
    } else {
        last + predict_future(&diffs)
    }
}

fn predict_past(history: &History) -> i64 {
    let diffs = history.windows(2).map(|w| w[1] - w[0]).collect_vec();
    let first = history.first().unwrap();

    if let Ok(&val) = diffs.iter().all_equal_value() {
        first - val
    } else {
        first - predict_past(&diffs)
    }
}

fn part1() {
    let histories = parse_histories();
    let future_values = histories.iter().map(predict_future);
    let result: i64 = future_values.sum();
    println!("{:?}", result);
}

fn part2() {
    let histories = parse_histories();
    let past_values = histories.iter().map(predict_past);
    let result: i64 = past_values.sum();
    println!("{:?}", result);
}
