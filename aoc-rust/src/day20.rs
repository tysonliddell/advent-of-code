use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    combinator::opt,
    multi::separated_list0,
    sequence::{pair, separated_pair},
    IResult,
};
use num::Integer;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Polarity {
    Low,
    High,
}

trait StatefulModule {
    fn on_pulse(&mut self, pulse: Polarity, from: &str) -> Option<Polarity>;
    fn destination_modules(&self) -> &[String];

    fn add_input(&mut self, _input: &str) {}
}

struct FlipFlop {
    is_on: bool,
    destinations: Vec<String>,
    last: Option<Polarity>,
}

impl FlipFlop {
    fn from_destinations(value: &[&str]) -> Self {
        Self {
            is_on: false,
            destinations: value.iter().map(|s| s.to_string()).collect(),
            last: None,
        }
    }
}

impl StatefulModule for FlipFlop {
    fn on_pulse(&mut self, pulse: Polarity, _from: &str) -> Option<Polarity> {
        self.last = Some(pulse);

        match pulse {
            Polarity::Low => {
                self.is_on = !self.is_on;
                Some(if self.is_on {
                    Polarity::High
                } else {
                    Polarity::Low
                })
            }
            Polarity::High => None,
        }
    }

    fn destination_modules(&self) -> &[String] {
        &self.destinations
    }
}

struct Conjunction {
    prev_input_pulses: HashMap<String, Polarity>,
    destinations: Vec<String>,
    last: Option<Polarity>,
}

impl Conjunction {
    fn from_destinations(destinations: &[&str]) -> Self {
        let destinations = destinations.iter().map(|d| d.to_string()).collect_vec();
        Self {
            prev_input_pulses: HashMap::new(),
            destinations,
            last: None,
        }
    }
}

impl StatefulModule for Conjunction {
    fn on_pulse(&mut self, pulse: Polarity, from: &str) -> Option<Polarity> {
        self.last = Some(pulse);

        *self.prev_input_pulses.get_mut(from).unwrap() = pulse;

        if self.prev_input_pulses.values().contains(&Polarity::Low) {
            Some(Polarity::High)
        } else {
            Some(Polarity::Low)
        }
    }

    fn destination_modules(&self) -> &[String] {
        &self.destinations
    }

    fn add_input(&mut self, input: &str) {
        self.prev_input_pulses
            .insert(input.to_string(), Polarity::Low);
    }
}

struct Broadcaster {
    destinations: Vec<String>,
    last: Option<Polarity>,
}

impl From<&[&str]> for Broadcaster {
    fn from(value: &[&str]) -> Self {
        Self {
            destinations: value.iter().map(|s| s.to_string()).collect(),
            last: None,
        }
    }
}

impl StatefulModule for Broadcaster {
    fn on_pulse(&mut self, pulse: Polarity, _from: &str) -> Option<Polarity> {
        self.last = Some(pulse);
        Some(pulse)
    }

    fn destination_modules(&self) -> &[String] {
        &self.destinations
    }
}

type ModuleConfig = HashMap<String, Box<dyn StatefulModule>>;

fn push_button(module_config: &mut ModuleConfig) -> (u64, u64) {
    let (mut low_count, mut high_count) = (0, 0);
    let mut q = VecDeque::from([(
        "button".to_string(),
        "broadcaster".to_string(),
        Polarity::Low,
    )]);

    while let Some((module_from_id, module_to_id, pulse)) = q.pop_front() {
        match pulse {
            Polarity::Low => low_count += 1,
            Polarity::High => high_count += 1,
        }

        if let Some(module_to) = module_config.get_mut(&module_to_id) {
            let out_pulse = module_to.on_pulse(pulse, &module_from_id);
            if let Some(polarity) = out_pulse {
                for dest_id in module_to.destination_modules() {
                    q.push_back((module_to_id.clone(), dest_id.clone(), polarity));
                }
            }
        }
    }

    (low_count, high_count)
}

fn parse_module(input: &str) -> IResult<&str, (&str, Box<dyn StatefulModule>)> {
    let name_parser = pair(opt(one_of("%&")), alphanumeric1);
    let destinations_parser = separated_list0(tag(", "), alphanumeric1);

    let (i, ((mod_type, mod_name), dest_ids)) =
        separated_pair(name_parser, tag(" -> "), destinations_parser)(input)?;

    let module: Box<dyn StatefulModule> = match mod_type {
        Some('%') => Box::new(FlipFlop::from_destinations(&dest_ids[..])),
        Some('&') => Box::new(Conjunction::from_destinations(&dest_ids[..])),
        _ => Box::new(Broadcaster::from(&dest_ids[..])),
    };

    Ok((i, (mod_name, module)))
}

fn parse_module_config() -> ModuleConfig {
    let input = include_str!("../../puzzle_input/d20").trim();
    let modules: Result<HashMap<_, _>, _> = input
        .lines()
        .map(|l| parse_module(l).map(|(_, (n, m))| (n.to_string(), m)))
        .collect();
    let mut modules = modules.unwrap();

    let mut mapping = vec![];
    for (name, module) in &modules {
        for dest in module.destination_modules() {
            mapping.push((dest.clone(), name.clone()));
        }
    }

    for (to, from) in mapping {
        if let Some(module) = modules.get_mut(&to) {
            module.add_input(&from);
        }
    }
    modules
}

fn generate_mermaid() {
    let input = include_str!("../../puzzle_input/d20").trim();
    let names: HashSet<_> = input.lines().map(|l| &l[0..3]).collect();
    let get_name_with_symbol = |name: &str| -> String {
        names
            .iter()
            .find(|n| &n[1..] == name)
            .map(|x| x.to_string())
            .unwrap_or(name.to_string())
    };

    for line in input.lines() {
        let (from, rest) = line.split_once(" -> ").unwrap();
        let dests = rest.split(", ").collect_vec();
        for d in dests {
            println!("{}-->{}", from, get_name_with_symbol(d));
        }
    }
}

fn part1() {
    let mut modules = parse_module_config();

    let (mut low_count, mut high_count) = (0, 0);
    for _ in 0..1000 {
        let (low, high) = push_button(&mut modules);
        low_count += low;
        high_count += high;
    }

    println!("{}", low_count * high_count);
}

fn part2() {
    generate_mermaid();
    println!("-------");

    // Obtained these values by inspecting the mermaid plot of the system. The
    // system has 4 parts, which each count in binary. Once each value is reach,
    // one part of the system has its accumulator with all 1s, which triggers a
    // hight pulse on the final accumulator (through an inverter). When this
    // happens, the counter resets to 0 and starts counting again. Therefore, we
    // need to lcm of all of these values. This is the first moment they are all
    // triggering the final accumulator at the same time.
    let vals = vec![
        0b111011010101usize,
        0b111110100011,
        0b111101001111,
        0b111010011011,
    ];

    let val = vals.into_iter().reduce(|acc, v| acc.lcm(&v)).unwrap();
    println!("{}", val);
}
