pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

struct Cubes {
    blue: u32,
    red: u32,
    green: u32,
}

struct Game {
    id: u32,
    rounds: Vec<Cubes>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.rounds.iter().map(|r| r.red).max().unwrap() <= 12
            && self.rounds.iter().map(|r| r.green).max().unwrap() <= 13
            && self.rounds.iter().map(|r| r.blue).max().unwrap() <= 14
    }

    fn min_cubes(&self) -> Cubes {
        Cubes {
            red: self.rounds.iter().map(|r| r.red).max().unwrap(),
            green: self.rounds.iter().map(|r| r.green).max().unwrap(),
            blue: self.rounds.iter().map(|r| r.blue).max().unwrap(),
        }
    }
}

fn parse_game(line: &str) -> Game {
    let mut game = Game {
        id: line
            .split(':')
            .next()
            .and_then(|s| s.split(' ').last())
            .and_then(|s| s.parse().ok())
            .unwrap(),
        rounds: Vec::new(),
    };

    for round in line.split(':').last().unwrap().split(';') {
        let mut cubes = Cubes {
            blue: 0,
            red: 0,
            green: 0,
        };

        for cube in round.trim().split(',') {
            let (num, colour) = cube.trim().split_once(' ').unwrap();
            let num = num.parse().unwrap();

            match colour {
                "red" => cubes.red = num,
                "green" => cubes.green = num,
                "blue" => cubes.blue = num,
                _ => panic!("unknown colour"),
            }
        }
        game.rounds.push(cubes);
    }

    game
}

fn get_games() -> Vec<Game> {
    let input = include_str!("../../puzzle_input/d2").trim();
    input.lines().map(|l: &str| parse_game(l)).collect()
}

fn part1() {
    let games = get_games();
    let valid_game_ids =
        games
            .into_iter()
            .filter_map(|g| if g.is_possible() { Some(g.id) } else { None });

    println!("{}", valid_game_ids.sum::<u32>());
}

fn part2() {
    let games = get_games();
    let powers = games
        .into_iter()
        .map(|g| g.min_cubes())
        .map(|cubes| cubes.red * cubes.green * cubes.blue);

    println!("{}", powers.sum::<u32>());
}
