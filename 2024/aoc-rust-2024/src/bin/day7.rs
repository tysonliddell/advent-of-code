use aoc_rust_2024::io;

struct Equation {
    total: i64,
    operands: Vec<i64>,
}

type Operation = fn(i64, i64) -> i64;

impl Equation {
    fn has_valid_operation(&self, ops: &[Operation]) -> bool {
        // store (running_total, depth) on a stack
        let mut stack = vec![(self.operands[0], 1)];
        while let Some((total, depth)) = stack.pop() {
            if depth == self.operands.len() {
                if total == self.total {
                    return true;
                }
            } else {
                for op in ops {
                    stack.push((op(total, self.operands[depth]), depth + 1));
                }
            }
        }
        false
    }
}

fn mul(x: i64, y: i64) -> i64 {
    x * y
}
fn add(x: i64, y: i64) -> i64 {
    x + y
}

fn concat(x: i64, y: i64) -> i64 {
    format!("{x}{y}").parse().unwrap()
}

fn parse_input() -> Vec<Equation> {
    let input = io::get_puzzle_input(7);

    input
        .trim()
        .lines()
        .map(|line| line.split_once(":").unwrap())
        .map(|(total, operands)| {
            (
                total.parse().unwrap(),
                operands.trim().split(' ').map(|v| v.parse().unwrap()),
            )
        })
        .map(|(total, operands)| Equation {
            total,
            operands: operands.collect(),
        })
        .collect()
}

fn part1_solution() -> i64 {
    let data = parse_input();
    let ops = [add, mul];

    data.into_iter()
        .filter(|eq| eq.has_valid_operation(&ops))
        .map(|eq| eq.total)
        .sum()
}

fn part2_solution() -> i64 {
    let data = parse_input();
    let ops = [add, mul, concat];

    data.into_iter()
        .filter(|eq| eq.has_valid_operation(&ops))
        .map(|eq| eq.total)
        .sum()
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
