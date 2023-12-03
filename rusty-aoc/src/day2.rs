use anyhow::{Error, Result};
use std::str::FromStr;

// only 12 red cubes, 13 green cubes, and 14 blue cubes
const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

#[derive(Debug, PartialEq, Eq)]
struct ParseSetError;

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

#[derive(Debug, PartialEq, Eq)]
struct Set {
    red: u8,
    blue: u8,
    green: u8,
}

impl FromStr for Set {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();
        for entry in s.trim().split(',') {
            let (number, kind) = entry
                .trim()
                .split_once(' ')
                .and_then(|(number, kind)| Some((number.parse::<u8>().ok()?, kind.trim())))
                .ok_or(Error::msg("Failed to parse cube list"))?;
            match kind {
                "red" => set.red = number,
                "green" => set.green = number,
                "blue" => set.blue = number,
                _ => return Err(Error::msg("Invalid color specified")),
            }
        }
        Ok(set)
    }
}

impl Default for Set {
    fn default() -> Self {
        Set::new(0, 0, 0)
    }
}

impl Set {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Set { red, green, blue }
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let mut sets = Vec::new();
        if line.starts_with("Game") {
            id = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .trim_end_matches(':')
                .parse::<u8>()
                .unwrap();
            let sets_str: Vec<&str> = line
                .split(':')
                .nth(1)
                .expect("this should not happen because the game format is always the same")
                .split(';')
                .map(|x| x.trim())
                .collect();
            for set in sets_str {
                sets.push(set.parse::<Set>()?);
            }
        }
        Ok(Game { id, sets })
    }
}

pub fn sum_valid_games(input: &Vec<String>) -> Result<u64> {
    let mut sum: u64 = 0;
    for line in input {
        let game = line.parse::<Game>()?;
        let mut possible = true;
        for set in game.sets {
            if set.red > RED || set.green > GREEN || set.blue > BLUE {
                possible = false;
                break;
            }
        }
        sum += game.id as u64 * possible as u64;
    }
    Ok(sum)
}

pub fn sum_powers_sets(input: &Vec<String>) -> Result<u64> {
    let mut sum: u64 = 0;
    for line in input {
        let game = line.parse::<Game>()?;
        let mut maxr = 0;
        let mut maxg = 0;
        let mut maxb = 0;
        for set in game.sets {
            if set.red > maxr {
                maxr = set.red;
            }
            if set.green > maxg {
                maxg = set.green;
            }
            if set.blue > maxb {
                maxb = set.blue;
            }
        }
        sum += maxr as u64 * maxg as u64 * maxb as u64;
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            id: 1,
            sets: vec![
                Set {
                    red: 4,
                    blue: 3,
                    green: 0,
                },
                Set {
                    red: 1,
                    blue: 6,
                    green: 2,
                },
                Set {
                    red: 0,
                    blue: 0,
                    green: 2,
                },
            ],
        };
        assert_eq!(expected, input.parse::<Game>().unwrap());
    }
    #[test]
    fn test_parse_set() {
        let input = "3 blue, 4 red";
        let expected = Set {
            red: 4,
            blue: 3,
            green: 0,
        };
        assert_eq!(expected, Set::from_str(input).unwrap());
    }

    #[test]
    fn test_sum_valid_games() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        assert_eq!(8, sum_valid_games(&input.to_vec()).unwrap());
    }

    #[test]
    fn test_sum_powers_sets() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        assert_eq!(2286, sum_powers_sets(&input.to_vec()).unwrap());
    }
}
