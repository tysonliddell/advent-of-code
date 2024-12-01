use std::ops::RangeInclusive;

use itertools::Itertools;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

type Position = (usize, usize, usize);
type Area = (RangeInclusive<usize>, RangeInclusive<usize>);

#[derive(Clone, Copy, PartialEq)]
struct Brick {
    start_pos: Position,
    end_pos: Position,
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (p1, p2) = value.split_once('~').unwrap();
        let p1 = p1
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let p2 = p2
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Self {
            start_pos: p1,
            end_pos: p2,
        }
    }
}

impl Brick {
    fn get_base_level(&self) -> usize {
        self.start_pos.2.min(self.end_pos.2)
    }

    fn get_top_level(&self) -> usize {
        self.start_pos.2.max(self.end_pos.2)
    }

    fn get_brick_canonical_range(
        &self,
    ) -> (
        RangeInclusive<usize>,
        RangeInclusive<usize>,
        RangeInclusive<usize>,
    ) {
        let (mut x1, mut x2) = (self.start_pos.0, self.end_pos.0);
        let (mut y1, mut y2) = (self.start_pos.1, self.end_pos.1);
        let (mut z1, mut z2) = (self.start_pos.2, self.end_pos.2);
        if x2 < x1 {
            (x1, x2) = (x2, x1);
        }
        if y2 < y1 {
            (y1, y2) = (y2, y1);
        }
        if z2 < z1 {
            (z1, z2) = (z2, z1);
        }
        (x1..=x2, y1..=y2, z1..=z2)
    }

    fn get_brick_xy_area(&self) -> Area {
        let (x_range, y_range, _) = self.get_brick_canonical_range();
        (x_range, y_range)
    }
}

#[derive(Clone)]
struct Bricks {
    bricks: Vec<Brick>,
}

impl Bricks {
    fn settle_bricks(&mut self, sort: bool) {
        if sort {
            self.bricks.sort_by_key(|brick| brick.get_base_level());
        }

        let drop_brick = |this: &Self, brick: Brick| -> Brick {
            let mut new_brick = brick;

            while new_brick.get_base_level() > 1 && this.get_bricks_underneath(new_brick).is_empty()
            {
                new_brick.start_pos.2 -= 1;
                new_brick.end_pos.2 -= 1;
            }
            new_brick
        };

        for i in 0..self.bricks.len() {
            self.bricks[i] = drop_brick(self, self.bricks[i]);
        }
    }

    // return the bricks directly underneath and touching another brick
    fn get_bricks_underneath(&self, brick: Brick) -> Vec<Brick> {
        self.bricks
            .iter()
            .copied()
            .filter(|&b| {
                b.get_top_level() == brick.get_base_level() - 1
                    && areas_overlap(b.get_brick_xy_area(), brick.get_brick_xy_area())
            })
            .collect()
    }

    // return the bricks directly above and touching another brick
    fn get_bricks_above(&self, brick: Brick) -> Vec<Brick> {
        self.bricks
            .iter()
            .copied()
            .filter(|&b| {
                b.get_base_level() == brick.get_top_level() + 1
                    && areas_overlap(b.get_brick_xy_area(), brick.get_brick_xy_area())
            })
            .collect()
    }

    fn can_disintegrate_brick(&self, brick: Brick) -> bool {
        let bricks_above = self.get_bricks_above(brick);
        if bricks_above.is_empty() {
            return true;
        }

        bricks_above
            .into_iter()
            .all(|b| self.get_bricks_underneath(b).len() > 1)
    }

    fn removable_bricks(&self) -> Vec<Brick> {
        self.bricks
            .iter()
            .copied()
            .filter(|&brick| self.can_disintegrate_brick(brick))
            .collect()
    }

    fn num_bricks_supported_by_brick(&self, brick_id: usize) -> usize {
        let mut new_bricks = self.clone();

        new_bricks.bricks[brick_id].start_pos.2 = 0;
        new_bricks.bricks[brick_id].end_pos.2 = 0;
        new_bricks.settle_bricks(false);

        let mut count = 0;
        for (b1, b2) in new_bricks.bricks[brick_id + 1..]
            .iter()
            .zip(self.bricks[brick_id + 1..].iter())
        {
            if b1 != b2 {
                count += 1;
            }
        }
        count
    }
}

impl From<&str> for Bricks {
    fn from(value: &str) -> Self {
        Self {
            bricks: value.trim().lines().map(Brick::from).collect(),
        }
    }
}

fn areas_overlap(a1: Area, a2: Area) -> bool {
    let (a1_x_range, a1_y_range) = a1;
    let (a2_x_range, a2_y_range) = a2;
    a1_x_range.start() <= a2_x_range.end()
        && a1_x_range.end() >= a2_x_range.start()
        && a1_y_range.start() <= a2_y_range.end()
        && a1_y_range.end() >= a2_y_range.start()
}

fn part1() {
    let mut bricks: Bricks = include_str!("../../puzzle_input/d22").trim().into();
    bricks.settle_bricks(true);
    println!("{}", bricks.removable_bricks().len())
}

fn part2() {
    let mut bricks: Bricks = include_str!("../../puzzle_input/d22").trim().into();
    bricks.settle_bricks(true);

    let total_count: usize = (0..bricks.bricks.len())
        .map(|brick_id| bricks.num_bricks_supported_by_brick(brick_id))
        .sum();

    println!("{}", total_count);
}
