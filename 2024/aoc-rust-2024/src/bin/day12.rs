use std::collections::HashSet;

use aoc_rust_2024::io;

type Map = Vec<Vec<u8>>;

type Position = (usize, usize);

#[derive(Debug)]
struct Region {
    plots: HashSet<Position>,
    _plant: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FenceDirection {
    Horizontal,
    Veritcal,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FenceOrientation {
    NorthOrWestOfPlot,
    SouthOrEastOfPlot,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct FencePiece {
    position: Position,
    dir: FenceDirection,
    orientation: FenceOrientation,
}

impl Region {
    fn get_area(&self) -> usize {
        self.plots.len()
    }

    fn get_perimeter(&self, map: &Map) -> Vec<FencePiece> {
        let height = map.len();
        let width = map[0].len();

        let boundary_type_horiz = |(row, col): (usize, usize)| -> Option<FenceOrientation> {
            if row == 0 {
                if self.plots.contains(&(row, col)) {
                    Some(FenceOrientation::NorthOrWestOfPlot)
                } else {
                    None
                }
            } else if row == height {
                if self.plots.contains(&(row - 1, col)) {
                    Some(FenceOrientation::SouthOrEastOfPlot)
                } else {
                    None
                }
            } else {
                let has_above = self.plots.contains(&(row - 1, col));
                let has_below = self.plots.contains(&(row, col));
                if has_above && !has_below || has_below && !has_above {
                    Some(if has_below {
                        FenceOrientation::NorthOrWestOfPlot
                    } else {
                        FenceOrientation::SouthOrEastOfPlot
                    })
                } else {
                    None
                }
            }
        };

        let boundary_type_vert = |(row, col): (usize, usize)| -> Option<FenceOrientation> {
            if col == 0 {
                if self.plots.contains(&(row, col)) {
                    Some(FenceOrientation::NorthOrWestOfPlot)
                } else {
                    None
                }
            } else if col == width {
                if self.plots.contains(&(row, col - 1)) {
                    Some(FenceOrientation::SouthOrEastOfPlot)
                } else {
                    None
                }
            } else {
                let has_left = self.plots.contains(&(row, col - 1));
                let has_right = self.plots.contains(&(row, col));
                if has_left && !has_right || has_right && !has_left {
                    Some(if has_right {
                        FenceOrientation::NorthOrWestOfPlot
                    } else {
                        FenceOrientation::SouthOrEastOfPlot
                    })
                } else {
                    None
                }
            }
        };

        let mut perimeter = Vec::new();
        for fence_row in 0..height + 1 {
            for fence_col in 0..width {
                if let Some(boundary_orientation) = boundary_type_horiz((fence_row, fence_col)) {
                    // perimeter.push((fence_row, fence_col, true, boundary_orientation));
                    perimeter.push(FencePiece {
                        position: (fence_row, fence_col),
                        dir: FenceDirection::Horizontal,
                        orientation: boundary_orientation,
                    });
                }
            }
        }

        for fence_col in 0..width + 1 {
            for fence_row in 0..height {
                if let Some(boundary_orientation) = boundary_type_vert((fence_row, fence_col)) {
                    // println!("{}: {:?}", self.plant, (fence_row, fence_col));
                    // perimeter.push((fence_row, fence_col, false, boundary_type));

                    perimeter.push(FencePiece {
                        position: (fence_row, fence_col),
                        dir: FenceDirection::Veritcal,
                        orientation: boundary_orientation,
                    });
                }
            }
        }

        perimeter
    }
}

fn perimeter_to_num_sides(perimeter: Vec<FencePiece>) -> usize {
    let (mut horiz, mut vert): (Vec<FencePiece>, Vec<_>) = perimeter
        .iter()
        .partition(|fence_piece| matches!(fence_piece.dir, FenceDirection::Horizontal));

    horiz.sort();
    let mut horiz_sides_count = 1;
    let (mut curr_row, mut curr_col) = horiz[0].position;
    let mut curr_orientation = horiz[0].orientation;
    for &fence_piece in horiz.iter().skip(1) {
        let (row, col) = fence_piece.position;
        let orientation = fence_piece.orientation;
        if row != curr_row || col != curr_col + 1 || orientation != curr_orientation {
            horiz_sides_count += 1;
        }
        curr_col = col;
        curr_row = row;
        curr_orientation = orientation;
    }

    // bit of a hack to make sorting by column easier (just swap row/col positions in tuple)
    for fence_piece in &mut vert {
        *fence_piece = FencePiece {
            position: (fence_piece.position.1, fence_piece.position.0),
            dir: fence_piece.dir,
            orientation: fence_piece.orientation,
        };
    }
    vert.sort();
    let mut vert_sides_count = 1;
    let (mut curr_col, mut curr_row) = horiz[0].position;
    let mut curr_orientation = horiz[0].orientation;
    for &fence_piece in horiz.iter().skip(1) {
        let (col, row) = fence_piece.position;
        let orientation = fence_piece.orientation;
        if col != curr_col || row != curr_row + 1 || orientation != curr_orientation {
            vert_sides_count += 1;
        }
        curr_row = row;
        curr_col = col;
        curr_orientation = orientation;
    }

    horiz_sides_count + vert_sides_count
}

fn parse_input() -> Map {
    let input = io::get_puzzle_input(12);
    let input = input.trim();

    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn get_alike_neighbours(pos: (usize, usize), map: &Map) -> Vec<(usize, usize)> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let (row, col) = pos;
    let plant = map[row][col];

    let nbhrs = [
        (row as i32 - 1, col as i32),
        (row as i32 + 1, col as i32),
        (row as i32, col as i32 - 1),
        (row as i32, col as i32 + 1),
    ]
    .into_iter()
    .filter(|(row, col)| (0..height).contains(row) && (0..width).contains(col))
    .map(|(row, col)| (row as usize, col as usize))
    .filter(|&(row, col)| map[row][col] == plant);

    nbhrs.collect()
}

fn get_regions(map: &Map) -> Vec<Region> {
    let mut seen = HashSet::new();

    let explore = |(row, col): (usize, usize), seen: &mut HashSet<(usize, usize)>| -> Region {
        let plant = map[row][col];

        let mut region = HashSet::new();
        let mut stack = vec![(row, col)];
        while let Some((row, col)) = stack.pop() {
            if seen.contains(&(row, col)) {
                continue;
            }
            region.insert((row, col));
            seen.insert((row, col));

            let nbrs = get_alike_neighbours((row, col), map);
            for n in nbrs {
                if !seen.contains(&n) {
                    stack.push(n);
                }
            }
        }

        Region {
            plots: region,
            _plant: plant,
        }
    };

    let mut regions = Vec::new();
    for (row, line) in map.iter().enumerate() {
        for (col, _) in line.iter().enumerate() {
            if !seen.contains(&(row, col)) {
                let region = explore((row, col), &mut seen);
                regions.push(region);
            }
        }
    }

    regions
}

fn part1_solution() -> usize {
    let map = parse_input();
    let regions = get_regions(&map);
    regions
        .into_iter()
        .map(|region| region.get_area() * region.get_perimeter(&map).len())
        .sum()
}

fn part2_solution() -> usize {
    let map = parse_input();
    let regions = get_regions(&map);
    regions
        .into_iter()
        .map(|region| (region.get_area(), region.get_perimeter(&map)))
        .map(|(area, perim)| (area, perimeter_to_num_sides(perim)))
        .map(|(area, sides)| area * sides)
        .sum()
}

fn main() {
    println!("{}", part1_solution());
    println!("{}", part2_solution());
}
