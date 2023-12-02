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
        id: 0,
        rounds: Vec::new(),
    };

    game.id = line
        .split(':')
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();

    game.rounds = line
        .split(':')
        .last()
        .unwrap()
        .split(';')
        .map(|r| {
            let mut round = Cubes {
                blue: 0,
                red: 0,
                green: 0,
            };
            let pairs = r.trim().split(',').map(|pair| {
                let mut iter = pair.trim().split(' ');
                let num = iter.next().unwrap().parse().unwrap();
                let colour: &str = iter.next().unwrap();
                (num, colour)
            });

            for (num, colour) in pairs {
                match colour {
                    "red" => round.red = num,
                    "green" => round.green = num,
                    "blue" => round.blue = num,
                    _ => panic!("unknown colour"),
                }
            }

            round
        })
        .collect();
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
