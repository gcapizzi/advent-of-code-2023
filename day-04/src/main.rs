use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let cards = input
        .iter()
        .map(|l| l.parse::<Card>())
        .collect::<Result<Vec<Card>>>()?;

    let sol1 = cards.iter().map(|c| c.value()).sum::<u32>();
    dbg!(sol1);

    let mut copies: Vec<usize> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        for j in (i + 1)..(i + 1 + card.score()) {
            copies[j] += copies[i];
        }
    }

    let sol2 = copies.iter().sum::<usize>();
    dbg!(sol2);

    Ok(())
}

struct Card {
    numbers: HashSet<u32>,
    winning: HashSet<u32>,
}

impl std::str::FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, card_str) = s
            .split_once(": ")
            .ok_or(anyhow!("invalid card str: {}", s))?;

        let (numbers_str, winning_str) = card_str
            .split_once(" | ")
            .ok_or(anyhow!("invalid card str: {}", s))?;

        let numbers = numbers_str
            .split_ascii_whitespace()
            .map(|n| n.parse().map_err(|e: std::num::ParseIntError| anyhow!(e)))
            .collect::<Result<HashSet<u32>>>()?;
        let winning = winning_str
            .split_ascii_whitespace()
            .map(|n| n.parse().map_err(|e: std::num::ParseIntError| anyhow!(e)))
            .collect::<Result<HashSet<u32>>>()?;

        Ok(Card { numbers, winning })
    }
}

impl Card {
    fn score(&self) -> usize {
        self.numbers.intersection(&self.winning).count()
    }

    fn value(&self) -> u32 {
        let score = self.score();
        if score == 0 {
            return 0;
        }
        2_u32.pow(score as u32 - 1)
    }
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}
