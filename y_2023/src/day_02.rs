use crate::lib::parse_input;

#[derive(Default)]
struct Game {
    red: i32,
    green: i32,
    blue: i32,
}



impl Game {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    fn is_valid(&self) -> bool {
        self.red <= Self::MAX_RED && self.green <= Self::MAX_GREEN && self.blue <= Self::MAX_BLUE
    }
}

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_02.txt");
    let mut result = 0;
    for line in input {
        let (id, rounds) = parse_line(line);
        if rounds.iter().all(|game| game.is_valid()) {
            result += id;
        }
    }
    result
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_02.txt");
    let mut result = 0;
    for line in input {
        let (_, rounds) = parse_line(line);
        result += max_colors_per_game(rounds);
    }
    result
}

fn parse_line(line: String) -> (i32, Vec<Game>) {
    let mut rounds: Vec<Game> = vec![];
    let line = line
        .replace(':', "")
        .replace("Game ", "")
        .replace(';', " ;")
        .replace(',', "");
    let mut line = line.split(' ');
    let id = line.next().unwrap().parse::<i32>().unwrap();
    let mut game = Game::default();
    let mut val = 0;
    for word in line {
        if word.to_string().parse::<i32>().is_ok() {
            val = word.parse::<i32>().unwrap();
        } else {
            match word {
                "red" => game.red = val,
                "green" => game.green = val,
                "blue" => game.blue = val,
                _ => {
                    rounds.push(game);
                    game = Game::default();
                }
            }
        }
    }
    // append last game
    rounds.push(game);
    (id, rounds)
}

fn max_colors_per_game(games: Vec<Game>) -> i32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for game in games {
        max_red = max_red.max(game.red);
        max_green = max_green.max(game.green);
        max_blue = max_blue.max(game.blue);
    }
    max_red * max_green * max_blue
}
