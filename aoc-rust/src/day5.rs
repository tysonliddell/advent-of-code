use itertools::Itertools;
use std::ops::RangeInclusive;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

type SeedRanges = Vec<RangeInclusive<u64>>;

struct RangeMap {
    in_range: RangeInclusive<u64>,
    out_range: RangeInclusive<u64>,
}

struct Map {
    // source: String,
    // destination: String,
    ranges: Vec<RangeMap>,
}

impl Map {
    fn get_range(&self, in_range: RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        let mut result = vec![];
        for range in &self.ranges {
            let overlap_start = *range.in_range.start().max(in_range.start());
            let overlap_end = *range.in_range.end().min(in_range.end());
            let overlap_size = overlap_end as i64 - overlap_start as i64 + 1;
            if overlap_size > 0 {
                let delta = overlap_start - range.in_range.start();
                let overlapping_out_start = range.out_range.start() + delta;
                let overlapping_out_end = overlapping_out_start + overlap_size as u64 - 1;
                result.push(overlapping_out_start..=overlapping_out_end);
            }
        }
        result
    }
}

fn parse_map(s: &str) -> Map {
    // let mut lines = s.lines();
    // let mut tokens = lines.next().unwrap().split_once(' ').unwrap().0.split('-');
    // let from = tokens.next().unwrap();
    // let to = tokens.last().unwrap();
    let lines = s.lines().skip(1);

    let mut ranges = vec![];
    for line in lines {
        let mut range = line.split(' ');
        let dest_start = range.next().unwrap().parse().unwrap();
        let source_start = range.next().unwrap().parse().unwrap();
        let length: u64 = range.next().unwrap().parse().unwrap();
        let range_map = RangeMap {
            in_range: source_start..=source_start + length - 1,
            out_range: dest_start..=dest_start + length - 1,
        };
        ranges.push(range_map);
    }

    // add missing identity map ranges to makes things easier later
    ranges.sort_by_key(|r| *r.in_range.start());
    let mut extra_ranges = vec![];
    let mut current = 0u64;
    for range in ranges.iter() {
        if *range.in_range.start() != current {
            let new_range_start = current;
            let new_range_end = range.in_range.start() - 1;
            extra_ranges.push(RangeMap {
                in_range: new_range_start..=new_range_end,
                out_range: new_range_start..=new_range_end,
            })
        }
        current = range.in_range.end() + 1;
    }
    extra_ranges.push(RangeMap {
        in_range: current..=u64::MAX,
        out_range: current..=u64::MAX,
    });
    ranges.extend(extra_ranges);
    ranges.sort_by_key(|r| *r.in_range.start());

    Map {
        // source: from.to_string(),
        // destination: to.to_string(),
        ranges,
    }
}

fn parse_maps_p2(seeds_are_ranges: bool) -> (SeedRanges, Vec<Map>) {
    let mut maps = vec![];
    let input = include_str!("../../puzzle_input/d5").trim();

    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap());

    let seed_ranges: Vec<RangeInclusive<u64>> = if seeds_are_ranges {
        seeds
            .chunks(2)
            .into_iter()
            .map(|mut chunk| {
                let start = chunk.next().unwrap();
                let size = chunk.next().unwrap();
                start..=start + size - 1
            })
            .collect()
    } else {
        seeds.map(|seed| seed..=seed).collect()
    };

    let maps_lines = input.split("\n\n").skip(1);
    for map in maps_lines {
        maps.push(parse_map(map));
    }

    (seed_ranges, maps)
}

fn ranges_traverse(
    input_range: RangeInclusive<u64>,
    maps: &Vec<Map>,
    level: usize,
) -> Vec<RangeInclusive<u64>> {
    if level == maps.len() {
        return vec![input_range];
    }
    let mut out_ranges = vec![];
    for range in maps[level].get_range(input_range) {
        out_ranges.extend(ranges_traverse(range, maps, level + 1));
    }
    out_ranges
}

fn seed_range_to_location_ranges(
    seed_range: RangeInclusive<u64>,
    maps: &Vec<Map>,
) -> Vec<RangeInclusive<u64>> {
    ranges_traverse(seed_range, maps, 0)
}

fn part1() {
    let (seed_ranges, maps) = parse_maps_p2(false);
    let locations = seed_ranges
        .into_iter()
        .flat_map(|r| seed_range_to_location_ranges(r.clone(), &maps))
        .map(|r| *r.start());

    println!("{}", locations.into_iter().min().unwrap());
}

fn part2() {
    let (seed_ranges, maps) = parse_maps_p2(true);
    let mut locations: Vec<_> = seed_ranges
        .into_iter()
        .flat_map(|r| seed_range_to_location_ranges(r.clone(), &maps))
        .map(|r| *r.start())
        .collect();
    locations.sort();

    let lowest_location = locations[0];
    println!("{:?}", lowest_location);
}
