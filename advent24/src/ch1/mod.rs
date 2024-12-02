use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_columns(path: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut first_column = vec![];
    let mut second_column = vec![];

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        first_column.push(parts[0].parse::<i32>()?);
        second_column.push(parts[1].parse::<i32>()?);
    }

    Ok((first_column, second_column))
}

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let (mut first_column, mut second_column) = get_columns("src/ch1/input.txt")?;
    let mut out: i32 = 0;

    first_column.sort();
    second_column.sort();

    for (first, second) in first_column.iter().zip(second_column.iter()) {
        out += (first - second).abs();
    }

    Ok(out)
}

pub fn part2() -> Result<i32, Box<dyn Error>> {
    let (first_column, second_column) = get_columns("src/ch1/input.txt")?;
    let mut out: i32 = 0;

    for first in first_column.iter() {
        out += first * second_column.iter().filter(|&&x| x == *first).count() as i32;
    }

    Ok(out)
}
