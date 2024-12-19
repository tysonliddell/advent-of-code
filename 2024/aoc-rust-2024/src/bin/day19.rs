use aoc_rust_2024::io;
use cached::{proc_macro::cached, SizedCache};
use trie_rs::{inc_search::Answer, Trie, TrieBuilder};

fn parse_input() -> (Vec<String>, Vec<String>) {
    let input = io::get_puzzle_input(19);
    let input = input.trim();

    let (available_towels, designs) = input.split_once("\n\n").unwrap();
    let available_towels = available_towels
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let designs = designs.trim().lines().map(|s| s.to_string()).collect();
    (available_towels, designs)
}

#[cached(
    ty = "SizedCache<String, u64>",
    create = "{ SizedCache::with_size(10000) }",
    convert = r#"{ format!("{}", design) }"#
)]
fn num_possible_designs(design: &str, trie: &Trie<u8>) -> u64 {
    if design.is_empty() {
        return 1;
    }

    let mut inc_search = trie.inc_search();
    let mut good_prefix_lengths = vec![];
    for i in 0..design.len() {
        match inc_search.query(&design.as_bytes()[i]) {
            None => break,
            Some(Answer::Match | Answer::PrefixAndMatch) => good_prefix_lengths.push(i + 1),
            Some(_) => continue,
        }
    }

    let mut total_possible = 0;
    for prefix_length in good_prefix_lengths {
        total_possible += num_possible_designs(&design[prefix_length..], trie);
    }

    return total_possible;
}

fn part1_solution() -> usize {
    let (available_towels, designs) = parse_input();

    let mut trie_builder = TrieBuilder::new();
    for towel in available_towels {
        trie_builder.push(&towel);
    }
    let trie = trie_builder.build();

    designs
        .into_iter()
        .filter(|design| num_possible_designs(design, &trie) > 0)
        .count()
}

fn part2_solution() -> u64 {
    let (available_towels, designs) = parse_input();

    let mut trie_builder = TrieBuilder::new();
    for towel in available_towels {
        trie_builder.push(&towel);
    }
    let trie = trie_builder.build();

    designs
        .into_iter()
        .map(|design| num_possible_designs(&design, &trie))
        .sum()
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
