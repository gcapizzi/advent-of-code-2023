use anyhow::{anyhow, Result};
use num_integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow!("invalid direction: {}", c)),
        }
    }
}

fn main() -> Result<()> {
    let mut input = read_input("input.txt")?;

    let directions = input
        .pop_front()
        .unwrap()
        .chars()
        .map(|c| c.try_into())
        .collect::<Result<Vec<Direction>>>()?;
    input.pop_front();
    let graph = input
        .iter()
        .map(|s| {
            let (from, to) = s
                .split_once(" = ")
                .ok_or(anyhow!("invalid mapping: {}", s))?;
            let parens: &[_] = &['(', ')'];
            let (to_left, to_right) = to
                .trim_matches(parens)
                .split_once(", ")
                .ok_or(anyhow!("invalid mapping: {}", s))?;
            Ok((from, (to_left, to_right)))
        })
        .collect::<Result<HashMap<&str, (&str, &str)>>>()?;

    let sol1 = directions
        .iter()
        .cycle()
        .scan("AAA", |state, d| {
            *state = if d == &Direction::Left {
                graph.get(state).unwrap().0
            } else {
                graph.get(state).unwrap().1
            };
            if *state == "ZZZ" {
                None
            } else {
                Some(d)
            }
        })
        .count()
        + 1;
    dbg!(sol1);

    let start_nodes = graph
        .clone()
        .into_keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<&str>>();

    let sol2 = start_nodes
        .into_iter()
        .map(|n| {
            directions
                .iter()
                .cycle()
                .scan(n, |state, d| {
                    *state = if d == &Direction::Left {
                        graph.get(state).unwrap().0
                    } else {
                        graph.get(state).unwrap().1
                    };
                    if state.ends_with("Z") {
                        None
                    } else {
                        Some(d)
                    }
                })
                .count()
                + 1
        })
        .reduce(|acc, e| lcm(acc, e));
    dbg!(sol2);

    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<VecDeque<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn test_foo() -> Result<()> {
        Ok(())
    }
}
