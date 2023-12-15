use anyhow::Result;
use std::collections::HashMap;

fn parse_line(line: &str) -> Result<(String, String, String)> {
    let (key, lr) = line.split_once('=').unwrap();
    let lr: Vec<String> = lr
        .split(',')
        .map(|x| x.trim().trim_matches('(').trim_matches(')').to_string())
        .collect::<Vec<String>>();
    Ok((key.to_string(), lr[0].clone(), lr[1].clone()))
}

type Directions = (String, String);

fn parse_input(input: &[String]) -> Result<(String, HashMap<String, Directions>)> {
    let mut map = HashMap::new();
    let directions = input[0].clone();
    for line in input.iter().skip(2) {
        let (key, left, right) = parse_line(line)?;
        let dir: Directions = (left, right);
        map.insert(key, dir);
    }
    Ok((directions, map))
}

pub fn walk_like_a_ghost(input: &[String]) -> Result<usize> {
    let map = parse_input(input)?;
    dbg!(map);
    Ok(0)
}
