use std::collections::HashSet;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

struct Contraption {
    board: Vec<Vec<char>>,
    beams: Vec<Beam>,
    energised: HashSet<Position>,
    seen_beams: HashSet<Beam>,
}

impl From<&str> for Contraption {
    fn from(value: &str) -> Self {
        let data = value.trim().lines().map(|l| l.chars().collect()).collect();
        Self {
            board: data,
            beams: vec![Beam::default()],
            energised: HashSet::new(),
            seen_beams: HashSet::new(),
        }
    }
}

impl Contraption {
    fn height(&self) -> usize {
        self.board.len()
    }

    fn width(&self) -> usize {
        self.board[0].len()
    }

    fn reset(&mut self, start_beam: Beam) {
        self.beams = vec![start_beam];
        self.energised = HashSet::new();
        self.seen_beams = HashSet::new();
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
                if !self.seen_beams.contains(&new_beam) {
                    new_beams.push(new_beam);
                    self.seen_beams.insert(new_beam);
                }
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

    fn energise(&mut self) -> usize {
        // not sure how many iterations we need, but this gives the correct results
        for _ in 0..(self.width() * self.height()) {
            self.step();
        }

        self.energised.len()
    }
}

type Position = (i32, i32);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

fn part1() {
    let input = include_str!("../../puzzle_input/d16").trim();
    let mut contraption: Contraption = input.into();
    let energised = contraption.energise();
    println!("{}", energised);
}

fn part2() {
    let input = include_str!("../../puzzle_input/d16").trim();
    let mut contraption: Contraption = input.into();

    let mut energised_vals = vec![];
    for row in 0..contraption.height() {
        let start_beam = Beam {
            pos: (row as i32, 0),
            dir: Direction::East,
        };
        contraption.reset(start_beam);
        energised_vals.push(contraption.energise());

        let start_beam = Beam {
            pos: (row as i32, contraption.width() as i32 - 1),
            dir: Direction::West,
        };
        contraption.reset(start_beam);
        energised_vals.push(contraption.energise());
    }

    for col in 0..contraption.width() {
        let start_beam = Beam {
            pos: (0, col as i32),
            dir: Direction::South,
        };
        contraption.reset(start_beam);
        energised_vals.push(contraption.energise());

        let start_beam = Beam {
            pos: (contraption.height() as i32 - 1, col as i32),
            dir: Direction::North,
        };
        contraption.reset(start_beam);
        energised_vals.push(contraption.energise());
    }

    println!("{}", energised_vals.iter().max().unwrap());
}
