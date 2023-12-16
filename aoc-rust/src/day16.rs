use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

struct Contraption {
    board: Vec<Vec<char>>,
    // beams: Vec<Beam>,
    beams: Vec<Beam>,
    energised: HashSet<Position>,
}

impl From<&str> for Contraption {
    fn from(value: &str) -> Self {
        let data = value.trim().lines().map(|l| l.chars().collect()).collect();
        Self {
            board: data,
            beams: vec![Beam::default()],
            energised: HashSet::new(),
        }
    }
}

impl Display for Contraption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = self.board.clone();
        for beam in self.beams.iter() {
            output[beam.pos.0 as usize][beam.pos.1 as usize] =
                format!("{}", beam.dir).chars().next().unwrap();
        }
        let s = output.into_iter().map(|row| row.iter().join("")).join("\n");
        write!(f, "{}", s)
    }
}

impl Contraption {
    fn height(&self) -> usize {
        self.board.len()
    }

    fn width(&self) -> usize {
        self.board[0].len()
    }

    fn step(&mut self) {
        // energise
        for beam in self.beams.iter() {
            self.energised.insert(beam.pos);
        }

        // split/bounce beams
        let mut new_beams = vec![];
        for beam in self.beams.iter_mut() {
            let (row, col) = beam.pos;
            let tile = self.board[row as usize][col as usize];
            if beam.is_split(tile) {
                let mut new_beam = *beam;
                new_beam.dir = new_beam.dir.rotate_90_anticlockwise();
                beam.dir = beam.dir.rotate_90_clockwise();
                new_beams.push(new_beam);
            } else if let Some(dir) = beam.get_bounce(tile) {
                beam.dir = dir;
            }
        }
        self.beams.extend(new_beams);

        // move each beam
        for beam in self.beams.iter_mut() {
            match beam.dir {
                Direction::North => beam.pos.0 -= 1,
                Direction::South => beam.pos.0 += 1,
                Direction::West => beam.pos.1 -= 1,
                Direction::East => beam.pos.1 += 1,
            }
        }

        // remove escaped beams
        let (height, width) = (self.height(), self.width());
        self.beams.retain(|b| {
            (0..height as i32).contains(&b.pos.0) && (0..width as i32).contains(&b.pos.1)
        });
    }

    // fn print_energised(&self) {
    //     let mut output = self.board.clone();
    //     for &(row, col) in self.energised.iter() {
    //         output[row as usize][col as usize] = '#';
    //     }
    //     for row in output {
    //         println!("{}", row.into_iter().join(""))
    //     }
    // }
}

type Position = (i32, i32);

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_90_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    fn rotate_90_anticlockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::North => '^',
            Self::South => 'v',
            Self::East => '>',
            Self::West => '<',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone, Copy)]
struct Beam {
    pos: Position,
    dir: Direction,
}

impl Default for Beam {
    fn default() -> Self {
        Beam {
            pos: (0, 0),
            dir: Direction::East,
        }
    }
}

impl Beam {
    fn is_split(&self, tile: char) -> bool {
        match tile {
            '|' if matches!(self.dir, Direction::East) => true,
            '|' if matches!(self.dir, Direction::West) => true,
            '-' if matches!(self.dir, Direction::North) => true,
            '-' if matches!(self.dir, Direction::South) => true,
            _ => false,
        }
    }

    fn get_bounce(&self, tile: char) -> Option<Direction> {
        match tile {
            '/' if matches!(self.dir, Direction::North) => Some(Direction::East),
            '/' if matches!(self.dir, Direction::South) => Some(Direction::West),
            '/' if matches!(self.dir, Direction::East) => Some(Direction::North),
            '/' if matches!(self.dir, Direction::West) => Some(Direction::South),
            '\\' if matches!(self.dir, Direction::North) => Some(Direction::West),
            '\\' if matches!(self.dir, Direction::South) => Some(Direction::East),
            '\\' if matches!(self.dir, Direction::East) => Some(Direction::South),
            '\\' if matches!(self.dir, Direction::West) => Some(Direction::North),
            _ => None,
        }
    }
}

fn get_enerergised(mut contraption: Contraption) -> HashSet<Position> {
    for _ in 0..(contraption.width() * contraption.height() * 4) {
        // for _ in 0..100 {
        contraption.step();
        println!("{}", contraption.beams.len());
        // println!("{}", contraption);
        // contraption.print_energised();
        // println!();
    }

    contraption.energised
}

fn part1() {
    let input = include_str!("../../puzzle_input/test").trim();
    let contraption: Contraption = input.into();
    // println!("{}", contraption);

    let energised = get_enerergised(contraption);
    println!("{}", energised.len());
}

fn part2() {}
