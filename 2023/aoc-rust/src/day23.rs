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

struct Map {
    data: Vec<Vec<char>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self {
            data: value.trim().lines().map(|l| l.chars().collect()).collect(),
        }
    }
}

impl Map {
    fn height(&self) -> usize {
        self.data.len()
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn get_accessible_neighbours(&self, pos: Position, can_climb_slopes: bool) -> Vec<Position> {
        if pos == (0, 1) {
            vec![(1, 1)]
        } else if pos == (self.height() - 1, self.width() - 2) {
            vec![(self.height() - 2, self.width() - 2)]
        } else {
            let (row, col) = pos;
            let mut nbrs = vec![];
            for (new_row, new_col) in [
                (row - 1, col),
                (row + 1, col),
                (row, col - 1),
                (row, col + 1),
            ] {
                let c = self.data[new_row][new_col];
                if c == '#' {
                    continue;
                } else if c == '.'
                    || can_climb_slopes
                    || (c == 'v' && new_row < row)
                    || (c == '^' && new_row > row)
                    || (c == '>' && new_col < col)
                    || (c == '<' && new_col > col)
                {
                    nbrs.push((new_row, new_col));
                }
            }

            nbrs
        }
    }

    fn get_longest_path(
        &mut self,
        from: Position,
        to: Position,
        ignore_positions: Vec<Position>,
    ) -> Option<Vec<Position>> {
        if to == from {
            return Some(vec![from]);
        }

        let mut ignore_positions = ignore_positions.clone();
        ignore_positions.push(to);

        let nbrs = self
            .get_accessible_neighbours(to, false)
            .into_iter()
            .filter(|n| !ignore_positions.contains(n))
            .collect_vec();

        if let Some(mut best_nbr_path) = nbrs
            .into_iter()
            .filter_map(|n| self.get_longest_path(from, n, ignore_positions.clone()))
            .max_by_key(|p| p.len())
        {
            best_nbr_path.push(to);
            Some(best_nbr_path)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Corridor {
    start: Position,
    end: Position,
    junction_1: Position,
    junction_2: Position,
    length: usize,
}

struct MazeGraph {
    corridors: Vec<Corridor>,
}

impl From<&str> for MazeGraph {
    fn from(value: &str) -> Self {
        let map = Map::from(value);
        let start_pos = (0, 1);
        let mut corridor_start_q = VecDeque::from([(start_pos, (0, 0))]);
        let mut seen = HashSet::new();
        let mut corridors = vec![];

        while let Some((start_pos, parent_junction)) = corridor_start_q.pop_front() {
            if seen.contains(&start_pos) {
                continue;
            }
            seen.insert(start_pos);
            let mut pos = start_pos;

            let mut prev_p = parent_junction;
            let mut length = 1;
            while let Ok(next_p) = map
                .get_accessible_neighbours(pos, true)
                .into_iter()
                .filter(|&p| p != prev_p)
                .exactly_one()
            {
                length += 1;
                prev_p = pos;
                pos = next_p;
            }

            let end_junction = pos;
            let corridor = Corridor {
                start: start_pos,
                end: prev_p,
                junction_1: parent_junction,
                junction_2: end_junction,
                length,
            };
            corridors.push(corridor);

            for p in map.get_accessible_neighbours(end_junction, true) {
                corridor_start_q.push_back((p, end_junction));
            }
        }

        corridors.sort_by_key(|c| c.start.min(c.end));
        corridors.dedup_by_key(|c| c.start.min(c.end));
        corridors.retain(|c| c.start != (1, 1) && c.end != (1, 1)); // hack

        Self { corridors }
    }
}

fn part1() {
    let mut map: Map = include_str!("../../puzzle_input/d23").trim().into();

    let start_pos = (0, 1);
    let end_pos = (map.height() - 1, map.width() - 2);

    let longest_path = map.get_longest_path(start_pos, end_pos, vec![]).unwrap();
    println!("{}", longest_path.len() - 1);
}

fn part2() {
    let maze = MazeGraph::from(include_str!("../../puzzle_input/d23"));

    let path = get_longest_path(
        *maze.corridors.first().unwrap(),
        *maze.corridors.last().unwrap(),
        &maze,
        HashSet::new(),
    )
    .unwrap();

    let total_length = path.iter().map(|s| s.length).sum::<usize>();
    println!("{:?}", total_length - 1);
}

fn get_attaching_corridors(corridor: Corridor, maze: &MazeGraph) -> Vec<Corridor> {
    let j1 = corridor.junction_1;
    let j2 = corridor.junction_2;
    maze.corridors
        .iter()
        .copied()
        .filter(|c| {
            c.junction_2 == j1 || c.junction_1 == j1 || c.junction_2 == j2 || c.junction_1 == j2
        })
        .filter(|&c| c != corridor)
        .collect()
}

fn get_longest_path(
    from: Corridor,
    to: Corridor,
    maze: &MazeGraph,
    ignore_junctions: HashSet<Position>,
) -> Option<Vec<Corridor>> {
    if to == from {
        return Some(vec![from]);
    }
    let prev_corridors = get_attaching_corridors(to, maze)
        .into_iter()
        .filter(|s| {
            !ignore_junctions.contains(&s.junction_1) && !ignore_junctions.contains(&s.junction_2)
        })
        .collect_vec();

    let mut ignore_junctions = ignore_junctions.clone();
    ignore_junctions.insert(to.junction_1);
    ignore_junctions.insert(to.junction_2);
    if let Some(mut best_prev_corridor_path) = prev_corridors
        .into_iter()
        .filter_map(|c| get_longest_path(from, c, maze, ignore_junctions.clone()))
        .max_by_key(|p| p.iter().map(|c| c.length).sum::<usize>())
    {
        best_prev_corridor_path.push(to);

        Some(best_prev_corridor_path)
    } else {
        None
    }
}
