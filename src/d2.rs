#[derive(Debug)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl Cubes {
    fn define_total(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }

    fn parse(input: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let cube_collection: Vec<_> = input.split(", ").collect();
        for cube in cube_collection.iter() {
            let cube_data: Vec<_> = cube.split_whitespace().collect();
            let parsed_count = cube_data[0].parse::<usize>();
            match (parsed_count, cube_data[1]) {
                (Ok(c), "red") => red += c,
                (Ok(c), "green") => green += c,
                (Ok(c), "blue") => blue += c,
                _ => {}
            }
        }
        Self { red, green, blue }
    }

    fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn is_possible(&self, max: &Cubes) -> bool {
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue
    }

    fn update(&mut self, other: &Cubes) {
        if other.red > self.red {
            self.red = other.red;
        }
        if other.green > self.green {
            self.green = other.green;
        }
        if other.blue > self.blue {
            self.blue = other.blue;
        }
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

pub mod p1 {
    use crate::d2::Cubes;

    fn get_game_id(input: &str) -> u32 {
        match input.split_once(' ') {
            Some((_, id)) => id.parse::<u32>().unwrap(),
            None => 0,
        }
    }

    fn evaluate_rounds(total: &Cubes, rounds_input: &str) -> bool {
        let mut valid_rounds = 0;
        let rounds: Vec<_> = rounds_input.split("; ").collect();
        let round_count = rounds.len();
        for round in rounds {
            let cubes = Cubes::parse(round);
            if cubes.is_possible(&total) {
                valid_rounds += 1;
            }
        }
        valid_rounds == round_count
    }

    fn handle_game(sum_handle: &mut u32, total: &Cubes, input: &str) {
        match input.split_once(": ") {
            Some((game_info, rounds)) => {
                if evaluate_rounds(total, rounds) {
                    *sum_handle += get_game_id(game_info);
                }
            }
            None => {}
        }
    }
    pub fn run() {
        let game_data = include_str!("../in-data/d2.txt");
        let total = Cubes::define_total(12, 13, 14);
        let mut possible_games = 0;

        for game in game_data.lines() {
            handle_game(&mut possible_games, &total, game);
        }
        println!("possible games: {possible_games}");
    }
}

pub mod p2 {
    use super::Cubes;

    fn min_game_cubes(rounds_input: &str) -> Cubes {
        let rounds: Vec<_> = rounds_input.split("; ").collect();
        let mut min_cubes = Cubes::empty();

        for round in rounds {
            min_cubes.update(&Cubes::parse(round));
        }
        min_cubes
    }

    fn handle_game(power_sum: &mut usize, input: &str) {
        match input.split_once(": ") {
            Some((_, rounds)) => {
                let min_cubes = min_game_cubes(rounds);
                *power_sum += min_cubes.power();
            }
            None => {}
        }
    }

    pub fn run() {
        let game_data = include_str!("../in-data/d2.txt");

        let mut power_sum = 0;

        for game in game_data.lines() {
            handle_game(&mut power_sum, game);
        }
        println!("power sum: {power_sum}");
    }
}
