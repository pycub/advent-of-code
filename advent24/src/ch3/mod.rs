use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "src/ch2/input.txt";

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let mut found = 0;

    Ok(1)
}
