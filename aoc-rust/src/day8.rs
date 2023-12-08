use itertools::Itertools;
use num::Integer;
use std::collections::HashMap;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

struct Node {
    key: String,
    next: (String, String),
}

fn parse_maps() -> (String, Vec<Node>) {
    let input = include_str!("../../puzzle_input/d8").trim();
    let (header, rest) = input.split_once("\n\n").unwrap();
    let nodes = rest
        .lines()
        .map(|l| {
            let (key, next) = l.split_once('=').unwrap();
            let next: String = next
                .chars()
                .filter(|&c| c.is_alphanumeric() || c == ',')
                .collect();
            let next = next.split_once(',').unwrap();
            Node {
                key: key.trim().to_string(),
                next: (next.0.to_string(), next.1.to_string()),
            }
        })
        .collect();
    (header.to_string(), nodes)
}

fn part1() {
    let (header, maps) = parse_maps();
    let mut node_map = HashMap::new();
    for node in maps {
        node_map.insert(node.key, node.next);
    }
    let node_map = node_map;

    let mut curr_key = "AAA".to_string();
    let mut step_count = 0;
    let mut steps = header.trim().chars().cycle();
    while curr_key != "ZZZ" {
        let dir = steps.next().unwrap();
        let (left, right) = node_map.get(&curr_key).unwrap();
        if dir == 'L' {
            curr_key = left.to_string();
        } else {
            curr_key = right.to_string();
        }
        step_count += 1;
    }
    println!("{}", step_count);
}

fn circuit_length(start: &str, header: &str, node_map: &HashMap<String, (String, String)>) -> u32 {
    let mut curr_node = start;
    let mut step_count = 0;
    let mut steps = header.chars().cycle();

    let is_at_end = |node: &str| node.ends_with('Z');

    while !is_at_end(curr_node) {
        let dir = steps.next().unwrap();
        let (left, right) = node_map.get(curr_node).unwrap();
        curr_node = if dir == 'L' { left } else { right };
        step_count += 1;
    }
    step_count
}

fn part2() {
    let (header, maps) = parse_maps();
    let mut node_map = HashMap::new();
    for node in maps {
        node_map.insert(node.key, node.next);
    }
    let node_map = node_map;

    let nodes = node_map.keys().filter(|k| k.ends_with('A')).collect_vec();
    let cycle_lengths = nodes
        .iter()
        .map(|n| circuit_length(n, &header, &node_map) as u64)
        .collect_vec();

    let lcm: u64 = cycle_lengths
        .into_iter()
        .reduce(|acc, e| acc.lcm(&e))
        .unwrap();

    println!("{}", lcm);
}
