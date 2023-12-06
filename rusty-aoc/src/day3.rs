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


const DIRECTIONS : [(isize, isize); 8] = [
    (0, 1), (0, -1), (1, 0), (-1, 0),
    (1, 1), (-1, -1), (1, -1), (-1, 1)
];

/// Check if a symbol is adjacent to a number
fn check_for_symbol(x: usize, y: usize, input: &[String]) -> bool {
    let mut symbol = false;
    for direction in DIRECTIONS {
        let x = x as isize + direction.0;
        let y = y as isize + direction.1;
        if x >= 0 && y >= 0 && x < input[0].len() as isize && y < input.len() as isize {
            let char = input[y as usize].chars().nth(x as usize).unwrap();
            if char.is_ascii_punctuation() && char != '.' {
                symbol = true;
            }
        }
    }
    symbol
}

/// Sum all adjacent numbers
pub fn sum_adjacent_numbers(input: &[String]) -> Result<usize> {
    let mut sum = 0;
    for (y, line) in input.iter().enumerate() {
        // parse numbers to nubmer struct
        let mut number = Number::default();
        for (x, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                number.literal.push(char);
                if check_for_symbol(x, y, input) {
                    number.adjacent = true;
                }
            } else if !number.literal.is_empty() {
                // if it is not a ascii digit and the literal is not empty then we have to add the
                // number because the number is finished
                if number.is_adjacent() {
                    sum += number.get_value()?;
                }
                number = Number::default();
            } 
        }
        if !number.literal.is_empty() && number.is_adjacent() {
            sum += number.get_value()?;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_adjacent_numbers() {
        let input = [
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        assert_eq!(sum_adjacent_numbers(input.to_vec().as_ref()).unwrap(), 4361);
    }
}
