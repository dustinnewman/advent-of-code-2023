use std::cmp::max;
use std::error::Error;

const INPUT: &str = include_str!("input.txt");
const SET_SEPARATOR: char = ';';
const CUBE_SEPARATOR: char = ',';
const GAME_SEPARATOR: &str = ": ";
const COLOR_SEPARATOR: char = ' ';
const ELF_MIN_CUBE_SET: CubeSet = CubeSet {
    red: 12,
    blue: 14,
    green: 13,
};

#[derive(Default, Debug)]
struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    max_set: CubeSet,
}

impl TryFrom<&str> for Game {
    type Error = Box<dyn Error>;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (id, sets) = line
            .strip_prefix("Game ")
            .ok_or("Game ID missing")?
            .split_once(GAME_SEPARATOR)
            .ok_or("Line does not have game separator")?;
        let mut max_set = CubeSet::default();
        for cubes in sets.split([SET_SEPARATOR, CUBE_SEPARATOR]) {
            let (amount, color) = cubes
                .trim()
                .split_once(COLOR_SEPARATOR)
                .ok_or("CubeSet does not contain spaces")?;
            let amount: usize = amount.parse()?;
            match color {
                "red" => max_set.red = max(max_set.red, amount),
                "blue" => max_set.blue = max(max_set.blue, amount),
                "green" => max_set.green = max(max_set.green, amount),
                _ => (),
            }
        }
        Ok(Self {
            id: id.parse()?,
            max_set,
        })
    }
}

impl Game {
    fn is_possible(&self, min_set: &CubeSet) -> bool {
        self.max_set.red <= min_set.red
            && self.max_set.blue <= min_set.blue
            && self.max_set.green <= min_set.green
    }
}

fn part1(games: &[Game], min_set: &CubeSet) -> usize {
    games
        .iter()
        .filter(|game| game.is_possible(min_set))
        .map(|game| game.id)
        .sum()
}

fn part2(games: &[Game]) -> usize {
    games
        .iter()
        .map(|game| game.max_set.red * game.max_set.blue * game.max_set.green)
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
