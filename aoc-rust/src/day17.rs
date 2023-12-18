use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

struct Map {
    data: Vec<Vec<u64>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let data = value
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
            .collect();
        Self { data }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .data
            .iter()
            .map(|row| row.iter().join(""))
            .collect_vec()
            .join("\n");
        write!(f, "{}", s)
    }
}

type Position = (i32, i32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn step_from(&self, pos: Position) -> Position {
        match self {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::West => (pos.0, pos.1 - 1),
        }
    }

    fn from_delta(from: Position, to: Position) -> Self {
        if to.1 > from.1 {
            Self::East
        } else if to.1 < from.1 {
            Self::West
        } else if to.0 > from.0 {
            Self::South
        } else if to.0 < from.0 {
            Self::North
        } else {
            panic!("No direction when position doesn't change!")
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct CityBlockMove {
    pos: Position,
    direction_entered: Option<Direction>,
    straight_line_length: u8,
}

#[derive(PartialEq, Eq)]
struct CityBlockDistance {
    distance: u64,
    mov: CityBlockMove,
}

impl Ord for CityBlockDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap used is a max-heap, we need a min-heap, so we flip the
        // comparison around
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for CityBlockDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // BinaryHeap used is a max-heap, we need a min-heap, so we flip the
        // comparison around
        Some(self.cmp(other))
    }
}

impl Map {
    fn height(&self) -> usize {
        self.data.len()
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn block_heat_loss(&self, pos: Position) -> u64 {
        self.data[pos.0 as usize][pos.1 as usize]
    }

    fn get_neighbours(&self, pos: Position) -> Vec<Position> {
        vec![
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ]
        .into_iter()
        .filter(|(r, c)| {
            (0..self.height() as i32).contains(r) && (0..self.width() as i32).contains(c)
        })
        .collect()
    }

    fn min_heat_loss(&self) -> u64 {
        let mut processed = HashSet::new();
        let mut q = BinaryHeap::new();

        let start = (0, 0);
        let end = (self.height() as i32 - 1, self.width() as i32 - 1);

        q.push(CityBlockDistance {
            distance: 0,
            mov: CityBlockMove {
                pos: start,
                direction_entered: None,
                straight_line_length: 0,
            },
        });

        let min_heat_loss = loop {
            let CityBlockDistance { distance, mov } = q.pop().unwrap();
            if processed.contains(&mov) {
                continue;
            }

            if mov.pos == end {
                break distance;
            } else {
                // best distance for this CityBlockMove
                processed.insert(mov);

                // add neighbours to priority queue
                for nbr in self.get_neighbours(mov.pos) {
                    let distance = distance + self.block_heat_loss(nbr);
                    let same_dir_nbr = mov.direction_entered.map(|d| d.step_from(mov.pos));
                    let opposite_dir_nbr = mov
                        .direction_entered
                        .map(|d| d.opposite().step_from(mov.pos));

                    if same_dir_nbr.is_some_and(|x| x == nbr) && mov.straight_line_length < 3 {
                        let nbr_mov = CityBlockMove {
                            pos: nbr,
                            direction_entered: mov.direction_entered,
                            straight_line_length: mov.straight_line_length + 1,
                        };
                        q.push(CityBlockDistance {
                            distance,
                            mov: nbr_mov,
                        });
                    } else if same_dir_nbr.is_none()
                        || (same_dir_nbr.is_some_and(|x| x != nbr)
                            && opposite_dir_nbr.is_some_and(|x| x != nbr))
                    {
                        let nbr_mov = CityBlockMove {
                            pos: nbr,
                            direction_entered: Some(Direction::from_delta(mov.pos, nbr)),
                            straight_line_length: 1,
                        };
                        q.push(CityBlockDistance {
                            distance,
                            mov: nbr_mov,
                        });
                    }
                }
            }
        };

        #[allow(clippy::let_and_return)]
        min_heat_loss
    }

    fn min_heat_loss_ultra(&self) -> u64 {
        let mut processed = HashSet::new();
        let mut q = BinaryHeap::new();

        let start = (0, 0);
        let end = (self.height() as i32 - 1, self.width() as i32 - 1);

        q.push(CityBlockDistance {
            distance: 0,
            mov: CityBlockMove {
                pos: start,
                direction_entered: None,
                straight_line_length: 0,
            },
        });

        let min_heat_loss = loop {
            let CityBlockDistance { distance, mov } = q.pop().unwrap();
            if processed.contains(&mov) || (mov.pos == end && mov.straight_line_length < 4) {
                continue;
            }

            if mov.pos == end {
                break distance;
            } else {
                // best distance for this CityBlockMove
                processed.insert(mov);

                // add neighbours to priority queue
                for nbr in self.get_neighbours(mov.pos) {
                    let distance = distance + self.block_heat_loss(nbr);
                    let same_dir_nbr = mov.direction_entered.map(|d| d.step_from(mov.pos));
                    let opposite_dir_nbr = mov
                        .direction_entered
                        .map(|d| d.opposite().step_from(mov.pos));

                    if same_dir_nbr.is_some_and(|x| x == nbr) && mov.straight_line_length < 10 {
                        let nbr_mov = CityBlockMove {
                            pos: nbr,
                            direction_entered: mov.direction_entered,
                            straight_line_length: mov.straight_line_length + 1,
                        };
                        q.push(CityBlockDistance {
                            distance,
                            mov: nbr_mov,
                        });
                    } else if same_dir_nbr.is_none()
                        || (same_dir_nbr.is_some_and(|x| x != nbr)
                            && opposite_dir_nbr.is_some_and(|x| x != nbr)
                            && mov.straight_line_length >= 4)
                    {
                        let nbr_mov = CityBlockMove {
                            pos: nbr,
                            direction_entered: Some(Direction::from_delta(mov.pos, nbr)),
                            straight_line_length: 1,
                        };
                        q.push(CityBlockDistance {
                            distance,
                            mov: nbr_mov,
                        });
                    }
                }
            }
        };

        #[allow(clippy::let_and_return)]
        min_heat_loss
    }
}

fn part1() {
    let input = include_str!("../../puzzle_input/d17").trim();
    let map: Map = input.into();
    println!("{}", map.min_heat_loss());
}

fn part2() {
    let input = include_str!("../../puzzle_input/d17").trim();
    let map: Map = input.into();
    println!("{}", map.min_heat_loss_ultra());
}
