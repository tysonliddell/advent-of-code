use regex::Regex;
use std::ops::Range;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

// PART 1
type Position = (usize, usize);

trait Neighbours {
    fn has_neighbour(&self, pos: Position) -> bool;
}

impl Neighbours for HashSet<Position> {
    fn has_neighbour(&self, pos: Position) -> bool {
        let (row, col) = pos;
        let row = row as i32;
        let col = col as i32;
        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                if self.contains(&(r as usize, c as usize)) {
                    return true;
                }
            }
        }
        false
    }
}

fn get_part_numbers(symbol_positions: HashSet<(usize, usize)>) -> Vec<u32> {
    let mut result = Vec::new();
    let input = include_str!("../../puzzle_input/d3").trim();

    let re = Regex::new(r"\d+").unwrap();
    for (row, line) in input.lines().enumerate() {
        for m in re.find_iter(line) {
            let num = m.as_str().parse().unwrap();
            for i in m.range() {
                if symbol_positions.has_neighbour((row, i)) {
                    result.push(num);
                    break;
                }
            }
        }
    }
    result
}

fn get_symbols() -> HashSet<Position> {
    let mut result = HashSet::new();

    let input = include_str!("../../puzzle_input/d3").trim();

    let re = Regex::new(r"[^.\d]").unwrap();
    for (row, line) in input.lines().enumerate() {
        for m in re.find_iter(line) {
            for i in m.range() {
                result.insert((row, i));
            }
        }
    }
    result
}

fn part1() {
    let symbol_positions = get_symbols();
    let part_number_sum: u32 = get_part_numbers(symbol_positions).into_iter().sum();

    println!("{}", part_number_sum);
}

// PART 2
#[derive(Clone, Copy)]
enum NodeData {
    Number(u32),
    Symbol(char),
}

struct Vertex {
    data: NodeData,
    positions: (usize, Range<usize>),
}

struct Graph {
    vertices: Vec<Vertex>,
    vertex_lookup: HashMap<Position, usize>,
}

impl From<&str> for Graph {
    fn from(value: &str) -> Self {
        let mut result = Self {
            vertices: Vec::default(),
            vertex_lookup: HashMap::default(),
        };

        let re = Regex::new(r"\d+|[^.]").unwrap();

        for (row, line) in value.lines().enumerate() {
            for m in re.find_iter(line) {
                let s = m.as_str();
                let data = if s.starts_with(|c: char| c.is_ascii_digit()) {
                    NodeData::Number(s.parse().unwrap())
                } else {
                    NodeData::Symbol(s.chars().next().unwrap())
                };

                for col in m.range() {
                    result
                        .vertex_lookup
                        .insert((row, col), result.vertices.len());
                }

                result.vertices.push(Vertex {
                    data,
                    positions: (row, m.range()),
                });
            }
        }
        result
    }
}

fn part2() {
    let input = include_str!("../../puzzle_input/d3").trim();
    let graph = Graph::from(input);

    // find gears
    let mut total = 0u32;
    for vertex in graph.vertices.iter() {
        if matches!(vertex.data, NodeData::Symbol('*')) {
            // search around gear symbol '*' for neighbours that are numbers
            let mut numeric_neighbour_vertices = HashSet::new();
            let (row, col) = (vertex.positions.0, vertex.positions.1.start);
            for r in row - 1..=row + 1 {
                for c in col - 1..=col + 1 {
                    if let Some(&node) = graph.vertex_lookup.get(&(r, c)) {
                        let node_data = graph.vertices[node].data;
                        if let NodeData::Number(num) = node_data {
                            numeric_neighbour_vertices.insert((node, num));
                        }
                    }
                }
            }

            if numeric_neighbour_vertices.len() == 2 {
                // exactly 2 numeric neighbours, so it's a gear
                total += numeric_neighbour_vertices
                    .into_iter()
                    .map(|(_, num)| num)
                    .product::<u32>();
            }
        }
    }
    println!("{}", total);
}
