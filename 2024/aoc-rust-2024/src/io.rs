use std::{fs, path::Path};

pub fn get_puzzle_input(day: u8) -> String {
    const PUZZLE_DIR: &str = "../puzzle_input";

    let filename = format!("d{}", day);
    let puzzle_input_path = Path::new(PUZZLE_DIR).join(filename);

    fs::read_to_string(puzzle_input_path).unwrap()
}
