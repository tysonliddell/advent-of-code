// #[macro_use]
// extern crate lazy_static;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
        _ => println!("Day {} not implemented", cli.day),
    }
}
