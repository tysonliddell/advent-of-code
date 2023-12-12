use cached::proc_macro::cached;
use itertools::Itertools;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

struct Record {
    rec: String,
    check: Vec<usize>,
}

struct Records {
    data: Vec<Record>,
}

impl From<&str> for Records {
    fn from(value: &str) -> Self {
        let mut records = vec![];
        for line in value.trim().split('\n') {
            let (s, check) = line.split_once(' ').unwrap();
            records.push(Record {
                rec: s.to_string(),
                check: check.split(',').map(|v| v.parse().unwrap()).collect_vec(),
            })
        }
        Self { data: records }
    }
}

#[cached]
fn num_arrangements(prev_char: char, suffix: String, counts: Vec<usize>) -> usize {
    let can_place_cluster = |s: &str, cluster_size: usize| -> bool {
        let following_char = s.chars().nth(cluster_size);
        s.len() >= cluster_size && !s[..cluster_size].contains('.') && following_char != Some('#')
    };

    if suffix.is_empty() {
        return if counts.is_empty() { 1 } else { 0 };
    } else if counts.is_empty() {
        return if suffix.contains('#') { 0 } else { 1 };
    } else if &suffix[0..1] == "." {
        return num_arrangements('.', suffix[1..].to_string(), counts);
    } else if &suffix[0..1] == "#" {
        if can_place_cluster(&suffix, counts[0]) {
            return num_arrangements('#', suffix[counts[0]..].to_string(), counts[1..].to_vec());
        } else {
            return 0;
        }
    } else {
        let mut num_possible_arrangements = 0;

        let cluster_size = counts[0];
        if prev_char != '#' && can_place_cluster(&suffix, cluster_size) {
            num_possible_arrangements += num_arrangements(
                '#',
                suffix[cluster_size..].to_string(),
                counts[1..].to_vec(),
            );
        }

        // case where we put a dot down instead of the cluster
        num_possible_arrangements +=
            num_arrangements('.', suffix[1..].to_string(), counts.to_vec());

        return num_possible_arrangements;
    }
}

impl Record {
    fn possible_arrangement_count(&self) -> usize {
        num_arrangements('.', self.rec.to_string(), self.check.to_vec())
    }

    fn unfold(&mut self) {
        self.rec = format!(
            "{}?{}?{}?{}?{}",
            self.rec, self.rec, self.rec, self.rec, self.rec
        );
        let mut new_check = vec![];
        new_check.extend(self.check.clone());
        new_check.extend(self.check.clone());
        new_check.extend(self.check.clone());
        new_check.extend(self.check.clone());
        new_check.extend(self.check.clone());
        self.check = new_check;
    }
}

fn part1() {
    let input = include_str!("../../puzzle_input/d12").trim();
    let records = Records::from(input);
    println!(
        "{}",
        records
            .data
            .iter()
            .map(|r| r.possible_arrangement_count())
            .sum::<usize>()
    );
}

fn part2() {
    let input = include_str!("../../puzzle_input/d12").trim();
    let mut records = Records::from(input);
    for r in records.data.iter_mut() {
        r.unfold();
    }

    println!(
        "{}",
        records
            .data
            .iter()
            .map(|r| r.possible_arrangement_count())
            .sum::<usize>()
    );
}
