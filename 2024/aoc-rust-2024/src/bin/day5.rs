use std::collections::{HashMap, HashSet};

use aoc_rust_2024::io;

type PageOrder = (u8, u8);
type Update = Vec<u8>;

fn parsed_input() -> (Vec<PageOrder>, Vec<Update>) {
    let input = io::get_puzzle_input(5);
    let (orderings, updates) = input.split_once("\n\n").unwrap();

    let orderings = orderings
        .lines()
        .map(|line| {
            line.split_once('|')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect();
    let updates = updates
        .lines()
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();

    (orderings, updates)
}

fn is_update_in_order(update: &Update, successors: &HashMap<u8, HashSet<u8>>) -> bool {
    let mut seen = HashSet::new();
    for &page in update {
        if seen.intersection(&successors[&page]).next().is_some() {
            return false;
        }
        seen.insert(page);
    }
    true
}

fn get_successors(orderings: Vec<PageOrder>) -> HashMap<u8, HashSet<u8>> {
    orderings.into_iter().fold(
        HashMap::<u8, HashSet<u8>>::new(),
        |mut orderings, (before, after)| {
            orderings.entry(before).or_default().insert(after);
            orderings
        },
    )
}

fn fix_update_ordering(update: Update, successors: &HashMap<u8, HashSet<u8>>) -> Update {
    let update: HashSet<_> = update.clone().into_iter().collect();
    let successors: HashMap<u8, HashSet<u8>> = update
        .clone()
        .into_iter()
        .map(|page| {
            (
                page,
                successors[&page].intersection(&update).cloned().collect(),
            )
        })
        .collect();

    let mut dependencies: HashMap<u8, HashSet<u8>> =
        update.iter().map(|&page| (page, HashSet::new())).collect();
    for (&before, after) in successors.iter() {
        for after in after {
            dependencies.get_mut(after).unwrap().insert(before);
        }
    }

    let mut dependency_free_pages: Vec<u8> = dependencies
        .iter()
        .filter(|(_, deps)| deps.is_empty())
        .map(|(&page, _)| page)
        .collect();

    let mut in_order = vec![];
    while let Some(page) = dependency_free_pages.pop() {
        in_order.push(page);
        for &later_page in &successors[&page] {
            let deps = dependencies.get_mut(&later_page).unwrap();
            deps.remove(&page);
            if deps.is_empty() {
                dependency_free_pages.push(later_page);
            }
        }
    }

    in_order
}

fn part1_solution() -> u32 {
    let (orderings, updates) = parsed_input();

    let successors = get_successors(orderings);

    updates
        .into_iter()
        .filter(|update| is_update_in_order(update, &successors))
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}

fn part2_solution() -> u32 {
    let (orderings, updates) = parsed_input();

    let successors = get_successors(orderings);

    updates
        .into_iter()
        .filter(|update| !is_update_in_order(update, &successors))
        .map(|update| fix_update_ordering(update, &successors))
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
