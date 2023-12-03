use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;

    let symbols = input
        .iter()
        .enumerate()
        .flat_map(|(i, l)| extract_labels(l, r"[^0-9\.]+", i))
        .collect::<Vec<Label>>();
    let numbers = input
        .iter()
        .enumerate()
        .flat_map(|(i, l)| extract_labels(l, r"\d+", i))
        .collect::<Vec<Label>>();

    let sol1 = numbers
        .iter()
        .filter(|n| symbols.iter().any(|s| s.is_adiacent(n)))
        .map(|n| n.value.parse::<u32>().unwrap())
        .sum::<u32>();
    dbg!(sol1);

    let sol2 = symbols
        .iter()
        .filter_map(|s| {
            let ns: Vec<&Label> = numbers.iter().filter(|n| n.is_adiacent(s)).collect();
            if ns.len() == 2 {
                Some(ns[0].value.parse::<u32>().unwrap() * ns[1].value.parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .sum::<u32>();
    dbg!(sol2);

    Ok(())
}

#[derive(Debug)]
struct Label {
    value: String,
    row: usize,
    col: usize,
}

impl Label {
    fn is_adiacent(&self, other: &Label) -> bool {
        let prev_row = if self.row > 0 { self.row - 1 } else { 0 };
        let prev_col = if self.col > 0 { self.col - 1 } else { 0 };
        other.row >= prev_row
            && other.row <= self.row + 1
            && (other.col + other.value.len() - 1) >= prev_col
            && other.col <= self.col + self.value.len()
    }
}

fn extract_labels(s: &str, re: &str, row: usize) -> Vec<Label> {
    let re = Regex::new(re).unwrap();
    re.find_iter(s)
        .map(|m| Label {
            value: m.as_str().to_string(),
            row,
            col: m.start(),
        })
        .collect()
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}
