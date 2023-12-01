use regex::Regex;
/// Solves the first part of the puzzle by going through each line of the input and adding the
/// two digit numbers to the result.
pub fn solve(input: &Vec<String>) -> usize {
    let mut result = 0;
    for line in input {
        let mut nums = "".to_string();
        for char in line.chars() {
            match char {
                '0'..='9' => {
                    dbg!(char);
                    nums.push(char);
                }
                _ => continue,
            };
        }

        let num = nums
            .chars()
            .next()
            .unwrap_or_else(|| {
                println!("Error getting first char! Defaulting to '0'");
                '0'
            })
            .to_string()
            + &nums.chars().last().unwrap().to_string();
        match num.parse::<usize>() {
            Ok(n) => {
                result += n;
            }
            Err(err) => println!("Error parsing number: {}", err),
        };
    }
    result
}

fn match_first_last(re_match: &str) -> &str {
    match re_match {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "eno" => "1",
        "owt" => "2",
        "eerht" => "3",
        "ruof" => "4",
        "evif" => "5",
        "xis" => "6",
        "neves" => "7",
        "thgie" => "8",
        "enin" => "9",
        _ => re_match,
    }
}

pub fn solve2(input: &Vec<String>) -> usize {
    let mut result = 0;
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_rev = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    for line in input {
        let first = re.find(line).unwrap().as_str();
        let rev_line = &line.chars().rev().collect::<String>();
        let last = re_rev.find(rev_line).unwrap().as_str();
        dbg!(first, last, rev_line);
        let num = match_first_last(first).to_string() + match_first_last(last);
        match num.parse::<usize>() {
            Ok(n) => {
                result += n;
            }
            Err(err) => println!("Error parsing number: {}", err),
        };
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_re() {
        let test = "one";
        let re = Regex::new(r#"(\d|one|two|three|four|five|six|seven|eight|nine")"#).unwrap();
        let find = re.find(test);
        assert_eq!(find.unwrap().as_str(), "one");
    }

    #[test]
    fn test_solve() {
        let test_input = vec!["123".to_string(), "456".to_string(), "789".to_string()];
        let result = solve(&test_input);
        assert_eq!(result, 13 + 46 + 79);
    }

    #[test]
    fn test_solve2() {
        let test_input = vec![
            "onetwothree".to_string(),
            "fourfivesix".to_string(),
            "seveneightnine".to_string(),
        ];
        let result = solve2(&test_input);
        assert_eq!(result, 13 + 46 + 79);
    }
}
