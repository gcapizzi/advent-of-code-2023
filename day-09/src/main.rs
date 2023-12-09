use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let seqs = input
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();

    let sol1 = seqs.iter().map(|s| predict_last(s.to_vec())).sum::<i64>();
    dbg!(sol1);

    let sol2 = seqs.iter().map(|s| predict_first(s.to_vec())).sum::<i64>();
    dbg!(sol2);

    Ok(())
}

fn predict_last(values: Vec<i64>) -> i64 {
    let mut diff_seqs = itertools::iterate(values, diffs)
        .take_while(|d| d.iter().any(|x| *x != 0))
        .collect::<Vec<Vec<i64>>>();
    diff_seqs.reverse();
    diff_seqs
        .iter()
        .fold(0, |prev, xs| prev + xs.last().unwrap())
}

fn predict_first(values: Vec<i64>) -> i64 {
    let mut diff_seqs = itertools::iterate(values, diffs)
        .take_while(|d| d.iter().any(|x| *x != 0))
        .collect::<Vec<Vec<i64>>>();
    diff_seqs.reverse();
    diff_seqs
        .iter()
        .fold(0, |prev, xs| xs.first().unwrap() - prev)
}

fn diffs(values: &Vec<i64>) -> Vec<i64> {
    values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(x, y)| y - x)
        .collect()
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use super::{predict_first, predict_last};

    #[test]
    fn test_predict_last() {
        assert_eq!(predict_last(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(predict_last(vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(predict_last(vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_predict_first() {
        assert_eq!(predict_first(vec![0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(predict_first(vec![1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(predict_first(vec![10, 13, 16, 21, 30, 45]), 5);
    }
}
