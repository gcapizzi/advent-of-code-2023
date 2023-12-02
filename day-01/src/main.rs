use anyhow::{anyhow, Result};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let input = read_input("input.txt")?;

    let calibrations = input
        .iter()
        .map(|l| calibration(l))
        .collect::<Result<Vec<u32>>>()?;
    println!("{:?}", calibrations.iter().sum::<u32>());

    let calibrations2 = input
        .iter()
        .map(|l| calibration2(l))
        .collect::<Result<Vec<u32>>>()?;
    println!("{:?}", calibrations2.iter().sum::<u32>());

    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

fn calibration(s: &str) -> Result<u32> {
    let re = Regex::new("[0-9]").unwrap();
    let first = re.find(s).ok_or(anyhow!("no digits found"))?.as_str();
    let rev_s = rev(s);
    let last = re.find(&rev_s).ok_or(anyhow!("no digits found"))?.as_str();

    Ok([to_digit(first), to_digit(&last)]
        .iter()
        .collect::<String>()
        .parse::<u32>()?)
}

fn calibration2(s: &str) -> Result<u32> {
    let first_re = Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let first = first_re.find(s).ok_or(anyhow!("no digits found"))?.as_str();

    let rev_s = rev(s);

    let rev_last_re = Regex::new("[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let rev_last = rev_last_re
        .find(&rev_s)
        .ok_or(anyhow!("no digits found"))?
        .as_str();
    let last = rev(rev_last);

    Ok([to_digit(first), to_digit(&last)]
        .iter()
        .collect::<String>()
        .parse::<u32>()?)
}

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

fn to_digit(s: &str) -> char {
    match s {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => s.chars().next().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::{calibration, calibration2};
    use anyhow::Result;

    #[test]
    fn test_calibration() -> Result<()> {
        assert_eq!(calibration("1abc2")?, 12);
        assert_eq!(calibration("pqr3stu8vwx")?, 38);
        assert_eq!(calibration("a1b2c3d4e5f")?, 15);
        assert_eq!(calibration("treb7uchet")?, 77);
        Ok(())
    }

    #[test]
    fn test_calibration2() -> Result<()> {
        assert_eq!(calibration2("two1nine")?, 29);
        assert_eq!(calibration2("eightwothree")?, 83);
        assert_eq!(calibration2("abcone2threexyz")?, 13);
        assert_eq!(calibration2("xtwone3four")?, 24);
        assert_eq!(calibration2("4nineeightseven2")?, 42);
        assert_eq!(calibration2("zoneight234")?, 14);
        assert_eq!(calibration2("7pqrstsixteen")?, 76);
        assert_eq!(calibration2("eighthree")?, 83);
        assert_eq!(calibration2("sevenineighthree")?, 73);
        Ok(())
    }
}
