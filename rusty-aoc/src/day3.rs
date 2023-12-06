use std::{collections::HashSet, usize};
use anyhow::Result;

/// Number struct
struct Number {
    adjacent: bool,
    literal: Vec<char>,
}

impl Default for Number {
    fn default() -> Number {
        Number::new(false, Vec::new())
    }
}

impl Number {
    fn new(adjacent: bool, literal: Vec<char>) -> Number {
        Number {
            adjacent,
            literal,
        }
    }
    /// Check if a number is adjacent to a symbol
    fn is_adjacent(&self) -> bool {
        self.adjacent
    }
    /// Get the value of a number
    fn get_value(&self) -> Result<usize> {
        Ok(self.literal
            .iter()
            .collect::<String>()
            .parse::<usize>()?)
    }
}

/// Get all symbols adjacent to a number
fn symbols_at(input: &[String]) -> HashSet<(usize, usize)> {
    let mut symbols = HashSet::new();
    for (idx, line) in input.iter().enumerate() {
        for (jdx, symbol) in line.chars().enumerate() {
            if symbol.is_ascii_punctuation() && symbol != '.' {
                // make a hashset with all the fields that are adjacent to a symbol
                symbols.insert((jdx, idx));
                symbols.insert((jdx + 1, idx));
                symbols.insert((jdx - 1, idx));
                symbols.insert((jdx, idx + 1));
                symbols.insert((jdx, idx - 1));
                symbols.insert((jdx + 1, idx - 1));
                symbols.insert((jdx - 1, idx + 1));
                symbols.insert((jdx + 1, idx + 1));
                symbols.insert((jdx - 1, idx - 1));
            }
        }
    }
    symbols
}

/// Parse numbers from a line of text
fn parse_numbers(idx: usize, line: &str, symbols: &HashSet<(usize, usize)>) -> Vec<Number> {
    let mut numbers = Vec::new();
    let mut i = 0;
    while i < line.len() {
        // parse numbers to nubmer struct
        if line.chars().nth(i).unwrap().is_ascii_digit() && i < line.len() {
            let mut number = Number::default();
            while line.chars().nth(i).unwrap().is_ascii_digit() {
                number.literal.push(line.chars().nth(i).unwrap());
                if symbols.contains(&(i, idx)) {
                    number.adjacent = true;
                }
                i += 1;
            }
            numbers.push(number);
        }
    }
    numbers
}

/// Sum all adjacent numbers
pub fn sum_adjacent_numbers(input: &[String]) -> Result<usize> {
    let mut sum = 0;
    let symbols = symbols_at(input);
    for (idx, line) in input.iter().enumerate() {
        // parse numbers to nubmer struct
        let numbers_in_line = parse_numbers(idx, line, &symbols);
        for number in numbers_in_line {
            if number.is_adjacent() {
                sum += number.get_value()?;
            }
        }
    }
    Ok(sum)
}
