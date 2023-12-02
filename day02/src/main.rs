use std::cmp::max;
use std::error::Error;

const INPUT: &str = include_str!("input.txt");
const ELF_MIN_CUBE_SET: CubeSet = CubeSet {
    red: 12,
    blue: 14,
    green: 13,
};
const SET_SEPARATOR: char = ';';
const CUBE_SEPARATOR: char = ',';
const GAME_SEPARATOR: &str = ": ";
const COLOR_SEPARATOR: char = ' ';

#[derive(Default, Debug)]
struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}

impl TryFrom<&str> for CubeSet {
    type Error = Box<dyn Error>;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let parts = string.split(CUBE_SEPARATOR);
        let mut set = Self::default();
        for part in parts {
            let (amount, color) = part
                .trim()
                .split_once(COLOR_SEPARATOR)
                .ok_or("CubeSet does not contain spaces")?;
            let amount: usize = amount.parse()?;
            match color {
                "red" => set.red = amount,
                "blue" => set.blue = amount,
                "green" => set.green = amount,
                _ => (),
            }
        }
        Ok(set)
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

impl TryFrom<&str> for Game {
    type Error = Box<dyn Error>;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (game, sets) = line
            .split_once(GAME_SEPARATOR)
            .ok_or("Line does not have game separator")?;
        let id: usize = game
            .strip_prefix("Game ")
            .ok_or("Game ID missing prefix")?
            .parse()?;
        let sets = sets
            .split(SET_SEPARATOR)
            .filter_map(|set| CubeSet::try_from(set).ok())
            .collect();
        Ok(Self { id, sets })
    }
}

impl Game {
    fn max_set(&self) -> CubeSet {
        let mut max_set = CubeSet::default();
        for set in &self.sets {
            max_set.red = max(set.red, max_set.red);
            max_set.blue = max(set.blue, max_set.blue);
            max_set.green = max(set.green, max_set.green);
        }
        max_set
    }

    fn is_possible(&self, min_set: &CubeSet) -> bool {
        self.sets.iter().all(|set| {
            set.red <= min_set.red && set.blue <= min_set.blue && set.green <= min_set.green
        })
    }
}

fn part1(games: &[Game], min_set: &CubeSet) -> usize {
    games
        .iter()
        .filter_map(|game| {
            if game.is_possible(min_set) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part2(games: &[Game]) -> usize {
    games
        .iter()
        .map(Game::max_set)
        .map(|set| set.red * set.blue * set.green)
        .sum()
}

fn main() {
    let games: Vec<Game> = INPUT
        .lines()
        .filter_map(|line| Game::try_from(line).ok())
        .collect();
    println!("Part 1: {}", part1(&games, &ELF_MIN_CUBE_SET));
    println!("Part 2: {}", part2(&games));
}
