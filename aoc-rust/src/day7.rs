use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

type Bid = u32;

lazy_static! {
    static ref CARDRANK: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('2', 1);
        m.insert('3', 2);
        m.insert('4', 3);
        m.insert('5', 4);
        m.insert('6', 5);
        m.insert('7', 6);
        m.insert('8', 7);
        m.insert('9', 8);
        m.insert('T', 9);
        m.insert('J', 10);
        m.insert('Q', 11);
        m.insert('K', 12);
        m.insert('A', 13);
        m
    };
}

#[derive(Clone, Eq, PartialEq)]
struct Hand {
    cards: String,
}

impl Hand {
    fn get_card_counts(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        for card in self.cards.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }
        counts
    }

    fn has_five_of_kind(&self) -> bool {
        let counts = self.get_card_counts();
        counts.values().filter(|&&v| v == 5).count() > 0
    }

    fn has_four_of_a_kind(&self) -> bool {
        let counts = self.get_card_counts();
        counts.values().filter(|&&v| v == 4).count() > 0
    }

    fn has_three_of_a_kind(&self) -> bool {
        let counts = self.get_card_counts();
        counts.values().filter(|&&v| v == 3).count() > 0
    }

    fn has_pair(&self) -> bool {
        let counts = self.get_card_counts();
        counts.values().filter(|&&v| v == 2).count() > 0
    }

    fn is_full_house(&self) -> bool {
        self.has_three_of_a_kind() && self.has_pair()
    }

    fn has_two_pair(&self) -> bool {
        let counts = self.get_card_counts();
        counts.values().filter(|&&v| v == 2).count() == 2
    }

    fn rank(&self) -> u32 {
        if self.has_five_of_kind() {
            7
        } else if self.has_four_of_a_kind() {
            6
        } else if self.is_full_house() {
            5
        } else if self.has_three_of_a_kind() {
            4
        } else if self.has_two_pair() {
            3
        } else if self.has_pair() {
            2
        } else {
            1
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let (r1, r2) = (self.rank(), other.rank());
        if r1 != r2 {
            return r1.cmp(&r2);
        }

        for (c1, c2) in self.cards.chars().zip(other.cards.chars()) {
            if c1 != c2 {
                return CARDRANK.get(&c1).cmp(&CARDRANK.get(&c2));
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hands() -> Vec<(Hand, Bid)> {
    let input = include_str!("../../puzzle_input/d7").trim();
    let hands = input.lines().map(|l| l.split_once(' ').unwrap());
    hands
        .map(|(h, b)| {
            (
                Hand {
                    cards: h.to_string(),
                },
                b.parse::<u32>().unwrap(),
            )
        })
        .collect()
}

fn part1() {
    let mut hands = parse_hands();
    hands.sort_by_key(|(h, _)| h.clone());

    let mut score = 0;
    for (rank, bid) in hands.into_iter().map(|(_, b)| b).enumerate() {
        score += (rank + 1) as u32 * bid;
    }
    println!("{}", score);
}

// PART 2
fn parse_hands_with_jokers() -> Vec<(HandWithJokers, Bid)> {
    let input = include_str!("../../puzzle_input/d7").trim();
    let hands = input.lines().map(|l| l.split_once(' ').unwrap());
    hands
        .map(|(h, b)| {
            (
                HandWithJokers {
                    cards: h.to_string(),
                },
                b.parse::<u32>().unwrap(),
            )
        })
        .collect()
}

lazy_static! {
    static ref CARDRANK_WITH_JOKERS: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('J', 0);
        m.insert('2', 1);
        m.insert('3', 2);
        m.insert('4', 3);
        m.insert('5', 4);
        m.insert('6', 5);
        m.insert('7', 6);
        m.insert('8', 7);
        m.insert('9', 8);
        m.insert('T', 9);
        m.insert('Q', 11);
        m.insert('K', 12);
        m.insert('A', 13);
        m
    };
}

#[derive(Clone, Eq, PartialEq)]
struct HandWithJokers {
    cards: String,
}

impl HandWithJokers {
    fn replace_jokers(&self) -> Hand {
        let num_jokers = self.cards.chars().filter(|&c| c == 'J').count();

        if num_jokers == 0 {
            Hand {
                cards: self.cards.clone(),
            }
        } else if num_jokers == 5 {
            Hand {
                cards: "AAAAA".to_string(),
            }
        } else {
            let counts = self.get_card_counts();
            let mut nonjoker_counts: Vec<_> =
                counts.iter().filter(|(&card, _)| card != 'J').collect();
            nonjoker_counts.sort_by_key(|(_, &count)| -(count as i32));

            let (&most_common_nonjoker, _) = nonjoker_counts[0];
            Hand {
                cards: self.cards.replace('J', &most_common_nonjoker.to_string()),
            }
        }
    }

    fn get_card_counts(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        for card in self.cards.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }
        counts
    }

    fn rank(&self) -> u32 {
        let best_hand = self.replace_jokers();
        best_hand.rank()
    }
}

impl Ord for HandWithJokers {
    fn cmp(&self, other: &Self) -> Ordering {
        let (r1, r2) = (self.rank(), other.rank());
        if r1 != r2 {
            return r1.cmp(&r2);
        }

        for (c1, c2) in self.cards.chars().zip(other.cards.chars()) {
            if c1 != c2 {
                return CARDRANK_WITH_JOKERS
                    .get(&c1)
                    .cmp(&CARDRANK_WITH_JOKERS.get(&c2));
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for HandWithJokers {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part2() {
    let mut hands = parse_hands_with_jokers();
    hands.sort_by_key(|(h, _)| h.clone());

    let mut score = 0;
    for (rank, bid) in hands.into_iter().map(|(_, b)| b).enumerate() {
        score += (rank + 1) as u32 * bid;
    }
    println!("{}", score);
}
