use aoc_rust_2024::io;
use itertools::Itertools;

#[derive(Debug)]
struct Cpu {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    program: Vec<u8>,
}

impl Cpu {
    fn reset(&mut self, a: u64, b: u64, c: u64) {
        self.a = a;
        self.b = b;
        self.c = c;
        self.ip = 0;
    }

    fn combo_value(&self) -> u64 {
        let combo_param = self.program[self.ip + 1];
        match combo_param {
            0..=3 => combo_param as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Unexpected combo operand!"),
        }
    }

    fn step(&mut self) -> Option<u8> {
        let op = self.program[self.ip];
        let param = self.program[self.ip + 1];

        let mut out = None;

        match op {
            0 => self.a /= 2_u64.pow(self.combo_value() as u32),
            1 => self.b ^= param as u64,
            2 => self.b = self.combo_value() & 0x7,
            3 if self.a == 0 => {} // do nothing
            3 if self.a != 0 => {
                self.ip = param as usize;
                return None;
            }
            4 => self.b ^= self.c,
            5 => out = Some((self.combo_value() & 0x7) as u8),
            6 => self.b = self.a / 2_u64.pow(self.combo_value() as u32),
            7 => self.c = self.a / 2_u64.pow(self.combo_value() as u32),
            _ => panic!("Unexpected opcode!"),
        }

        self.ip += 2;
        out
    }

    fn run(&mut self, debugging: bool) -> Vec<u8> {
        let mut output = Vec::new();

        while !self.is_finised() {
            if let Some(out) = self.step() {
                if debugging
                    && (output.len() >= self.program.len() || self.program[output.len()] != out)
                {
                    return output;
                }
                output.push(out);
            }
        }

        output
    }

    fn is_finised(&self) -> bool {
        self.ip >= self.program.len()
    }
}

fn parse_input() -> Cpu {
    let input = io::get_puzzle_input(17);
    let input: Vec<_> = input.trim().lines().collect();

    let (_, a) = input[0].split_once(": ").unwrap();
    let (_, b) = input[1].split_once(": ").unwrap();
    let (_, c) = input[2].split_once(": ").unwrap();
    let (_, program) = input[4].split_once(": ").unwrap();

    Cpu {
        a: a.parse().unwrap(),
        b: b.parse().unwrap(),
        c: c.parse().unwrap(),
        ip: 0,
        program: program.split(',').map(|i| i.parse().unwrap()).collect(),
    }
}

fn part1_solution() -> String {
    let mut cpu = parse_input();
    let output = cpu.run(false);
    output.iter().join(",")
}

fn _part2_solution_slow() -> u64 {
    let mut cpu = parse_input();
    let (b, c) = (cpu.b, cpu.c);

    // takes about 5 mins to find the solution
    for a in 0..u64::MAX {
        let x = 0b0011110000001111;

        let a = (a << 16) + x;
        cpu.reset(a, b, c);
        let output = cpu.run(true);
        if output.len() > 12 {
            println!("A: {:064b}, OUTPUT: {:?}", a, output);
        }
        if output == cpu.program {
            return a;
        }
    }
    panic!("No solution!")
}

fn part2_solution_fast() -> u64 {
    let mut cpu = parse_input();
    let mut stack = Vec::from([(0, cpu.program.len())]);

    while let Some((acc, values_left)) = stack.pop() {
        if values_left == 0 {
            return acc;
        }
        let target = cpu.program[values_left - 1];
        for bits in (0..8).rev() {
            let val = (acc << 3) + bits;
            cpu.reset(val, 0, 0);
            if cpu.run(false)[0] == target {
                stack.push((val, values_left - 1));
            }
        }
    }
    panic!("No solution!")
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution_fast());
}
