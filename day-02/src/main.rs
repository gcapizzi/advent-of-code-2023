use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl std::str::FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split(": ");
        let id = chunks
            .next()
            .ok_or(anyhow!("invalid game str: {}", s))?
            .split(" ")
            .nth(1)
            .ok_or(anyhow!("invalid game str: {}", s))?
            .parse::<u32>()?;
        let sets = chunks
            .next()
            .ok_or(anyhow!("invalid game str: {}", s))?
            .split("; ")
            .map(|s| s.parse())
            .collect::<Result<Vec<Set>>>()?;

        Ok(Game { id, sets })
    }
}

impl Game {
    fn is_possible(&self, bag: &Set) -> bool {
        self.sets.iter().all(|s| s.is_possible(bag))
    }

    fn min_bag(&self) -> Set {
        let red = self.sets.iter().map(|s| s.red).max().unwrap_or(0);
        let green = self.sets.iter().map(|s| s.green).max().unwrap_or(0);
        let blue = self.sets.iter().map(|s| s.blue).max().unwrap_or(0);

        Set { red, green, blue }
    }
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl std::str::FromStr for Set {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };
        for chunk in s.split(", ") {
            if chunk.ends_with("red") {
                set.red = chunk
                    .split(" ")
                    .next()
                    .ok_or(anyhow!("invalid set str: {}", s))?
                    .parse::<u32>()?;
            }
            if chunk.ends_with("green") {
                set.green = chunk
                    .split(" ")
                    .next()
                    .ok_or(anyhow!("invalid set str: {}", s))?
                    .parse::<u32>()?;
            }
            if chunk.ends_with("blue") {
                set.blue = chunk
                    .split(" ")
                    .next()
                    .ok_or(anyhow!("invalid set str: {}", s))?
                    .parse::<u32>()?;
            }
        }

        Ok(set)
    }
}

impl Set {
    fn is_possible(&self, bag: &Set) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let games = input
        .iter()
        .map(|l| l.parse::<Game>())
        .collect::<Result<Vec<Game>>>()?;

    let bag = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let sol1: u32 = games
        .iter()
        .filter(|g| g.is_possible(&bag))
        .map(|g| g.id)
        .sum();
    dbg!(sol1);

    let sol2: u32 = games.iter().map(|g| g.min_bag().power()).sum();
    dbg!(sol2);

    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}
