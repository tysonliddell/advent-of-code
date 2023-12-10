use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

type Position = (usize, usize);

struct Loop {
    data: Vec<Vec<char>>,
    start: Position,
    map_height: usize,
    map_width: usize,
}

impl Loop {
    fn find_farthest_pos_distance(&self) -> usize {
        let (_, prev_pos) = self.neighbour_positions(self.start);

        let mut curr_pos = self.next_pos(self.start, prev_pos);
        let mut prev_pos = self.start;
        let mut count = 1;

        while curr_pos != self.start {
            (curr_pos, prev_pos) = (self.next_pos(curr_pos, prev_pos), curr_pos);
            count += 1;
        }

        count / 2
    }

    fn all_loop_positions(&self) -> HashSet<Position> {
        let (_, prev_pos) = self.neighbour_positions(self.start);
        let mut loop_positions = HashSet::new();
        loop_positions.insert(self.start);

        let mut curr_pos = self.next_pos(self.start, prev_pos);
        let mut prev_pos = self.start;
        while curr_pos != self.start {
            loop_positions.insert(curr_pos);
            (curr_pos, prev_pos) = (self.next_pos(curr_pos, prev_pos), curr_pos);
        }

        loop_positions
    }

    fn cleanup_pipes(&mut self) {
        let loop_positions = self.all_loop_positions();
        for row in 0..self.map_height {
            for col in 0..self.map_width {
                if !loop_positions.contains(&(row, col)) {
                    self.data[row][col] = '.';
                }
            }
        }
    }

    fn possible_neighbour_positions(&self, pos: Position) -> Vec<Position> {
        let (r, c) = pos;
        let mut nbrs = vec![];

        if r > 0 {
            let above = self.data[r - 1][c];
            if ['7', 'F', '|'].contains(&above) {
                nbrs.push((r - 1, c));
            }
        }
        if r < self.map_height - 1 {
            let below = self.data[r + 1][c];
            if ['J', 'L', '|'].contains(&below) {
                nbrs.push((r + 1, c));
            }
        }
        if c > 0 {
            let left = self.data[r][c - 1];
            if ['F', 'L', '-'].contains(&left) {
                nbrs.push((r, c - 1));
            }
        }
        if c < self.map_width - 1 {
            let right = self.data[r][c + 1];
            if ['7', 'J', '-'].contains(&right) {
                nbrs.push((r, c + 1));
            }
        }

        nbrs
    }

    fn all_neighbour_positions(&self, pos: Position) -> Vec<Position> {
        let (r, c) = pos;
        let mut nbrs = vec![];

        if r > 0 {
            nbrs.push((r - 1, c));
        }
        if r < self.map_height - 1 {
            nbrs.push((r + 1, c));
        }
        if c > 0 {
            nbrs.push((r, c - 1));
        }
        if c < self.map_width - 1 {
            nbrs.push((r, c + 1));
        }

        nbrs
    }

    fn neighbour_positions(&self, pos: Position) -> (Position, Position) {
        let (r, c) = pos;
        let mut nbrs = vec![];
        let pipe = self.data[r][c];

        if r > 0 && ['J', 'L', '|'].contains(&pipe) {
            let above = self.data[r - 1][c];
            if ['7', 'F', '|'].contains(&above) {
                nbrs.push((r - 1, c));
            }
        }
        if r < self.map_height - 1 && ['7', 'F', '|'].contains(&pipe) {
            let below = self.data[r + 1][c];
            if ['J', 'L', '|'].contains(&below) {
                nbrs.push((r + 1, c));
            }
        }
        if c > 0 && ['J', '7', '-'].contains(&pipe) {
            let left = self.data[r][c - 1];
            if ['F', 'L', '-'].contains(&left) {
                nbrs.push((r, c - 1));
            }
        }
        if c < self.map_width - 1 && ['L', 'F', '-'].contains(&pipe) {
            let right = self.data[r][c + 1];
            if ['7', 'J', '-'].contains(&right) {
                nbrs.push((r, c + 1));
            }
        }

        if nbrs.len() != 2 {
            panic!("Wrong number of neighbours");
        }

        nbrs.into_iter().collect_tuple().unwrap()
    }

    fn next_pos(&self, pos: Position, prev_pos: Position) -> Position {
        let nbrs = self.neighbour_positions(pos);
        if nbrs.0 != prev_pos {
            nbrs.0
        } else {
            nbrs.1
        }
    }

    fn replace_start_char(&mut self) {
        let (p1, p2) = self
            .possible_neighbour_positions(self.start)
            .into_iter()
            .collect_tuple()
            .unwrap();
        let (r, c) = self.start;

        let has_above = r > 0 && [p1.0, p2.0].contains(&(r - 1));
        let has_below = r < self.map_height - 1 && [p1.0, p2.0].contains(&(r + 1));
        let has_left = c > 0 && [p1.1, p2.1].contains(&(c - 1));
        let has_right = c < self.map_width - 1 && [p1.1, p2.1].contains(&(c + 1));

        if has_above && has_below {
            self.data[r][c] = '|';
        } else if has_above && has_left {
            self.data[r][c] = 'J';
        } else if has_above && has_right {
            self.data[r][c] = 'L';
        } else if has_below && has_left {
            self.data[r][c] = '7';
        } else if has_below && has_right {
            self.data[r][c] = 'F';
        } else if has_left && has_right {
            self.data[r][c] = '-';
        } else {
            panic!("Position cannot be in loop!")
        }
    }

    // Double to size of the board. This makes separating the interior from the
    // exterior easier.
    //
    // Use the '*' character to denote squares we don't want to copy, to ensure
    // that the double board size has exactly 4x the number of '.' characters.
    fn stretch_board(&mut self) {
        let mut res = vec![];

        for row in self.data.iter() {
            let mut s = Vec::new();
            for &c in row {
                if ".-".contains(c) {
                    s.push(c);
                    s.push(c);
                } else if c == '|' {
                    s.push(c);
                    s.push('*');
                } else if "FL".contains(c) {
                    s.push(c);
                    s.push('-');
                } else if "7J".contains(c) {
                    s.push(c);
                    s.push('*');
                } else {
                    panic!("Invalid character '{}'", c);
                }
            }
            res.push(s.clone());
            res.push(s.clone());
        }

        for row in (0..res.len()).step_by(2) {
            for col in 0..res[0].len() {
                let c = res[row][col];

                if ".|*".contains(c) {
                    res[row + 1][col] = c;
                } else if c == '-' {
                    res[row + 1][col] = '*';
                } else if "F7".contains(c) {
                    res[row + 1][col] = '|';
                } else if "LJ".contains(c) {
                    res[row + 1][col] = '*';
                } else {
                    panic!("Invalid character '{}'", c);
                }
            }
        }

        self.data = res;
        self.map_height *= 2;
        self.map_width *= 2;
        self.start = (self.start.0 * 2, self.start.1 * 2);
    }

    fn get_interior_positions(&self) -> HashSet<Position> {
        let mut exterior_positions = HashSet::new();
        let mut q: VecDeque<_> = (0..self.map_height)
            .cartesian_product(0..self.map_width)
            .filter(|&(r, c)| {
                r == 0 || r == self.map_height - 1 || c == 0 || c == self.map_width - 1
            })
            .filter(|&(r, c)| "*.".contains(self.data[r][c]))
            .inspect(|&pos| {
                exterior_positions.insert(pos);
            })
            .collect();

        while !q.is_empty() {
            let pos = q.pop_front().unwrap();
            exterior_positions.insert(pos);

            let nbrs = self.all_neighbour_positions(pos);
            for n in nbrs {
                let c = self.data[n.0][n.1];
                if !exterior_positions.contains(&n) && "*.".contains(c) {
                    q.push_back(n);
                    exterior_positions.insert(n);
                }
            }
        }
        exterior_positions
    }
}

fn parse_loop() -> Loop {
    let input = include_str!("../../puzzle_input/d10").trim();
    let data = input
        .trim()
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let mut start = (0, 0);
    for (r, row) in data.iter().enumerate() {
        for (c, &char) in row.iter().enumerate() {
            if char == 'S' {
                start = (r, c)
            }
        }
    }

    let (map_height, map_width) = (data.len(), data[0].len());

    let mut l = Loop {
        data,
        start,
        map_height,
        map_width,
    };
    l.replace_start_char();
    l.cleanup_pipes();
    l
}

fn part1() {
    let l = parse_loop();
    println!("{:?}", l.find_farthest_pos_distance());
}

fn part2() {
    let mut l = parse_loop();
    l.stretch_board();
    let exterior_positions = l.get_interior_positions();
    let interior_position_count = (0..l.map_height)
        .cartesian_product(0..l.map_width)
        .filter(|pos| !exterior_positions.contains(pos))
        .filter(|&(r, c)| l.data[r][c] == '.') // make sure to ignore '*' chars
        .count();

    // stretched board has 4x as many '.' characters
    println!("{}", interior_position_count / 4);
}
