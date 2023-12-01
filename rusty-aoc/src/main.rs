use std::fs;
mod day1;

const DAY: usize = 1;

pub fn read_input(day: usize, test: Option<bool>) -> Vec<String> {
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

fn main() {
    let test = Some(false);
    let input = read_input(DAY, test);
    let result = day1::solve2(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1sol1() {
        let test = Some(true);
        match test {
            Some(true) => {
                let input = read_input(DAY, test);
                let result = day1::solve(&input);
                assert_eq!(result, 142);
            }
            Some(false) | None => {
                let test_input = read_input(DAY, Some(true));
                let test_result = day1::solve(&test_input);
                let input = read_input(DAY, test);
                let result = day1::solve(&input);
                // check the test and the real result
                assert_eq!(test_result, 142);
                assert_eq!(result, 54927)
            }
        }
    }

    #[test]
    fn test_day1sol2() {
        let test = Some(true);
        match test {
            Some(true) => {
                let input: Vec<String> = (r"two1nine\n\
                                            eightwothree\n\
                                            abcone2threexyz\n\
                                            xtwone3four\n\
                                            4nineeightseven2\n\
                                            zoneight234\n\
                                            7pqrstsixteen")
                    .to_string()
                    .split('\n')
                    .map(|x| x.to_string())
                    .collect();
                let result = day1::solve2(&input);
                assert_eq!(result, 281);
            }
            Some(false) | None => {
                let test_input = read_input(DAY, Some(true));
                let test_result = day1::solve2(&test_input);
                let input = read_input(DAY, test);
                let result = day1::solve2(&input);
                // check the test and the real result
                assert_eq!(test_result, 0);
                assert_eq!(result, 0)
            }
        }
    }
}
