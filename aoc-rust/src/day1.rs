pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

// PART 1
fn get_calibration_number_p1(line: &str) -> u32 {
    let nums: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    let first = *nums.as_slice().first().unwrap();
    let last = *nums.as_slice().last().unwrap();
    (10 * first) + last
}

fn get_cal_numbers_p1() -> Vec<u32> {
    let input = include_str!("../../puzzle_input/d1").trim();
    input
        .lines()
        .map(|l: &str| get_calibration_number_p1(l))
        .collect()
}

fn part1() {
    println!("{}", get_cal_numbers_p1().into_iter().sum::<u32>())
}

// PART 2
fn parse_num(line: &[u8]) -> Option<u32> {
    let first_char = line[0] as char;

    if let Some(digit) = first_char.to_digit(10) {
        Some(digit)
    } else if line.starts_with(b"one") {
        Some(1)
    } else if line.starts_with(b"two") {
        Some(2)
    } else if line.starts_with(b"three") {
        Some(3)
    } else if line.starts_with(b"four") {
        Some(4)
    } else if line.starts_with(b"five") {
        Some(5)
    } else if line.starts_with(b"six") {
        Some(6)
    } else if line.starts_with(b"seven") {
        Some(7)
    } else if line.starts_with(b"eight") {
        Some(8)
    } else if line.starts_with(b"nine") {
        Some(9)
    } else if line.starts_with(b"zero") {
        Some(0)
    } else {
        None
    }
}

fn get_line_nums(line: &str) -> Vec<u32> {
    let bytes = line.as_bytes();
    let mut result = Vec::new();

    for i in 0..bytes.len() {
        if let Some(num) = parse_num(&bytes[i..]) {
            result.push(num)
        }
    }
    result
}

fn get_calibration_number_p2(line: &str) -> u32 {
    let nums = get_line_nums(line);
    let first = nums.as_slice().first().unwrap();
    let last = nums.as_slice().last().unwrap();
    (10 * first) + last
}

fn get_cal_numbers_p2() -> Vec<u32> {
    let input = include_str!("../../puzzle_input/d1").trim();
    input
        .lines()
        .map(|l: &str| get_calibration_number_p2(l))
        .collect()
}

fn part2() {
    println!("{}", get_cal_numbers_p2().into_iter().sum::<u32>());
}
