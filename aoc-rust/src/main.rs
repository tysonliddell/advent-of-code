#[macro_use]
extern crate lazy_static;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, value_name = "DAY")]
    day: u8,

    #[arg(short, long, value_name = "PART")]
    part: u8,
}

fn main() {
    let cli = Cli::parse();

    match cli.day {
        1 => day1::run(cli.part),
        2 => day2::run(cli.part),
        3 => day3::run(cli.part),
        4 => day4::run(cli.part),
        5 => day5::run(cli.part),
        6 => day6::run(cli.part),
        7 => day7::run(cli.part),
        8 => day8::run(cli.part),
        9 => day9::run(cli.part),
        10 => day10::run(cli.part),
        11 => day11::run(cli.part),
        12 => day12::run(cli.part),
        13 => day13::run(cli.part),
        _ => println!("Day {} not implemented", cli.day),
    }
}
