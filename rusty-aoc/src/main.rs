use anyhow::Result;
use clap::Parser;
use std::fs;
mod day1;
mod day2;
mod day3;

/// Advent of Code 2023
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Day to run
    day: u8,
    /// Solution to run
    solution: u8,
}

pub fn read_input(day: u8, test: Option<bool>) -> Vec<String> {
    // read from input file
    let filename = match test {
        Some(true) => format!("./inputs/test_input{}.txt", day),
        _ => format!("./inputs/input{}.txt", day),
    };
    let mut contents: Vec<String> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split('\n')
        .map(|x| x.to_string())
        .collect();
    if contents.last().unwrap().as_str() == "" {
        contents.pop();
    };
    contents
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match (args.day, args.solution) {
        (1, 1) => {
            let input = read_input(1, Some(false));
            let result = day1::solve(&input)?;
            println!("Day 1 part 1: {}", result);
        }
        (1, 2) => {
            let input = read_input(1, Some(false));
            let result = day1::solve2(&input);
            println!("Day 1 part 2: {}", result);
        }
        (2, 1) => {
            let input = read_input(2, Some(false));
            let result = day2::sum_valid_games(&input)?;
            println!("Day 2 part 1: {}", result);
        }
        (2, 2) => {
            let input = read_input(2, Some(false));
            let result = day2::sum_powers_sets(&input)?;
            println!("Day 2 part 2: {}", result);
        }
        (3, 1) => {
            let input = read_input(3, Some(false));
            let result = day3::sum_adjacent_numbers(&input)?;
            println!("Day 3 part 1: {}", result);
        }
        (a, b) => println!("No solution for day {} part {}", a, b),
    }
    Ok(())
}
