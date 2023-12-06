pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

#[derive(Clone, Copy)]
struct RaceRecord {
    time: u64,
    distance: u64,
}

fn parse_races() -> Vec<RaceRecord> {
    let input = include_str!("../../puzzle_input/d6").trim();

    let (times, distances) = input.split_once('\n').unwrap();
    let times = times.trim().split_once(':').unwrap().1;
    let distances = distances.trim().split_once(':').unwrap().1;
    let times = times.split_whitespace().map(|s| s.parse().unwrap());
    let distances = distances.split_whitespace().map(|s| s.parse().unwrap());

    let records: Vec<_> = times
        .zip(distances)
        .map(|(t, d)| RaceRecord {
            time: t,
            distance: d,
        })
        .collect();
    records
}

fn parse_races_p2() -> RaceRecord {
    let input = include_str!("../../puzzle_input/d6").trim();

    let (time, distance) = input.split_once('\n').unwrap();
    let time = time.trim().split_once(':').unwrap().1;
    let distance = distance.trim().split_once(':').unwrap().1;
    let time: u64 = time
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = distance
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    RaceRecord { time, distance }
}

fn get_record_beating_strategies(record: RaceRecord) -> Vec<RaceRecord> {
    let mut result = vec![];
    for time in 0..=record.time {
        let speed = time;
        let distance = (record.time - time) * speed;
        if distance > record.distance {
            result.push(RaceRecord { time, distance });
        }
    }

    result
}

fn part1() {
    let races = parse_races();
    let race_strategies = races.into_iter().map(get_record_beating_strategies);
    let number_of_ways_to_win = race_strategies.map(|v| v.len() as u64);
    let num_strat_combinations: u64 = number_of_ways_to_win.product();
    println!("{}", num_strat_combinations)
}

fn part2() {
    let record = parse_races_p2();
    let strats = get_record_beating_strategies(record);
    println!("{:?}", strats.len());
}
