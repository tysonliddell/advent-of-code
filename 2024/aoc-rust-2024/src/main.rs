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
    let solution = aoc_rust_2024::solution(cli.day, cli.part);
    println!("Day {}, part {} solution: {}", cli.day, cli.part, solution);
}
