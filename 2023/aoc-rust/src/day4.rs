use std::collections::HashSet;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

type Card = (Vec<u32>, Vec<u32>);

trait CardLogic {
    fn winning_numbers(&self) -> Vec<u32>;
    fn number_of_wins(&self) -> u32;
    fn score(&self) -> u32;
}

impl CardLogic for Card {
    fn winning_numbers(&self) -> Vec<u32> {
        let winning_numbers: HashSet<u32> = HashSet::from_iter(self.0.clone());
        let all_numbers: HashSet<u32> = HashSet::from_iter(self.1.clone());
        Vec::from_iter(winning_numbers.intersection(&all_numbers).cloned())
    }

    fn number_of_wins(&self) -> u32 {
        self.winning_numbers().len() as u32
    }

    fn score(&self) -> u32 {
        if self.winning_numbers().is_empty() {
            0
        } else {
            2u32.pow(self.winning_numbers().len() as u32 - 1)
        }
    }
}

fn parse_cards() -> Vec<Card> {
    let mut cards = Vec::new();
    let input = include_str!("../../puzzle_input/d4").trim();

    for line in input.lines() {
        let nums = line.split_once(':').unwrap().1;
        let (card_nums, winning_nums) = nums.split_once('|').unwrap();
        let card_nums = card_nums
            .trim()
            .split(' ')
            .filter_map(|n| n.parse().ok())
            .collect();
        let winning_nums = winning_nums
            .trim()
            .split(' ')
            .filter_map(|n| n.parse().ok())
            .collect();
        cards.push((card_nums, winning_nums));
    }
    cards
}

fn part1() {
    let cards = parse_cards();
    let scores = cards.into_iter().map(|c| c.score());
    let total_score: u32 = scores.sum();
    println!("{}", total_score)
}

fn part2() {
    let cards = parse_cards();
    let num_wins: Vec<_> = cards.into_iter().map(|c| c.number_of_wins()).collect();
    let mut card_counts = vec![1; num_wins.len()];

    for (i, number_of_wins) in num_wins.into_iter().enumerate() {
        let number_of_wins = number_of_wins as usize;
        let cards_left = card_counts.len() - i - 1;

        // copy cards
        for c in (i + 1)..=(i + number_of_wins.min(cards_left)) {
            card_counts[c] += card_counts[i];
        }
    }

    let total_score: u32 = card_counts.into_iter().sum();
    println!("{}", total_score);
}
