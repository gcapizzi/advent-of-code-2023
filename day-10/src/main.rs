use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Start,
    Ground,
    Pipe(Direction, Direction),
}

impl Tile {
    fn exit_dir(&self, enter_dir: Direction) -> Direction {
        match self {
            Tile::Start => todo!(),
            Tile::Ground => todo!(),
            Tile::Pipe(d1, d2) => {
                if &enter_dir.opposite() == d1 {
                    d2.clone()
                } else if &enter_dir.opposite() == d2 {
                    d1.clone()
                } else {
                    todo!()
                }
            }
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(Tile::Start),
            '.' => Ok(Tile::Ground),
            '|' => Ok(Tile::Pipe(Direction::North, Direction::South)),
            '-' => Ok(Tile::Pipe(Direction::East, Direction::West)),
            'L' => Ok(Tile::Pipe(Direction::North, Direction::East)),
            'J' => Ok(Tile::Pipe(Direction::North, Direction::West)),
            '7' => Ok(Tile::Pipe(Direction::South, Direction::West)),
            'F' => Ok(Tile::Pipe(Direction::South, Direction::East)),
            _ => Err(anyhow!("invalid tile char: {}", c)),
        }
    }
}

fn main() -> Result<()> {
    let input = read_input("input.txt")?;

    let start_row = input
        .iter()
        .enumerate()
        .find(|(_, r)| r.contains(&Tile::Start))
        .unwrap()
        .0;
    let start_col = input[start_row]
        .iter()
        .position(|t| t == &Tile::Start)
        .unwrap();

    let mut row = start_row;
    let mut col = start_col;
    let mut direction = Direction::South; // manual
    let mut len = 0;
    let mut path: HashSet<(usize, usize)> = HashSet::from([(row, col)]);
    loop {
        match direction {
            Direction::North => row -= 1,
            Direction::South => row += 1,
            Direction::East => col += 1,
            Direction::West => col -= 1,
        }
        len += 1;
        if row == start_row && col == start_col {
            dbg!(len);
            break;
        }
        direction = input[row][col].exit_dir(direction);
        path.insert((row, col));
    }

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if path.contains(&(row, col)) {
                match input[row][col] {
                    Tile::Start => print!("S"),
                    Tile::Ground => print!("."),
                    Tile::Pipe(Direction::North, Direction::South) => print!("┃"),
                    Tile::Pipe(Direction::East, Direction::West) => print!("━"),
                    Tile::Pipe(Direction::North, Direction::East) => print!("┗"),
                    Tile::Pipe(Direction::North, Direction::West) => print!("┛"),
                    Tile::Pipe(Direction::South, Direction::West) => print!("┓"),
                    Tile::Pipe(Direction::South, Direction::East) => print!("┏"),
                    Tile::Pipe(_, _) => todo!(),
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }

    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Vec<Tile>>> {
    let file = File::open(path)?;
    BufReader::new(file)
        .lines()
        .map(|l| {
            Ok(l?
                .chars()
                .map(|c| c.try_into())
                .collect::<Result<Vec<Tile>>>()?)
        })
        .collect()
}
