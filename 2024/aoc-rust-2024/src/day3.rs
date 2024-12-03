use regex::Regex;

use crate::{io, Solution};

pub struct Day3;

impl Solution for Day3 {
    fn part1_solution(&self) -> String {
        let input = io::get_puzzle_input(3);

        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        let data: Vec<(u32, u32)> = re
            .captures_iter(&input)
            .map(|c| {
                let (_, [x, y]) = c.extract();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        let result: u32 = data.iter().map(|(x, y)| x * y).sum();
        result.to_string()
    }

    fn part2_solution(&self) -> String {
        let input = io::get_puzzle_input(3);

        let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();

        let tokens_iter = re.find_iter(&input).map(|m| m.as_str().to_string());

        let mut good_tokens = vec![];
        let mut keep = true;
        for token in tokens_iter {
            match token.as_str() {
                "don't()" => {
                    keep = false;
                }
                "do()" => keep = true,
                _ if keep => {
                    good_tokens.push(token);
                }
                _ => {}
            }
        }

        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let result: u32 = good_tokens
            .into_iter()
            .map(|s| {
                let (_, [x, y]) = re.captures(&s).unwrap().extract();
                x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap()
            })
            .sum();

        result.to_string()
    }
}
