use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "src/ch2/input.txt";

fn is_safe_sequence(levels: &[i32]) -> bool {
    if levels.len() < 3 {
        return false;
    }

    let ascending = levels[0] < levels[1];

    for i in 0..levels.len() - 1 {
        let level = levels[i];
        let nextlevel = levels[i + 1];

        // Check if sequence maintains ascending/descending pattern
        if ascending && level >= nextlevel {
            return false;
        }
        if !ascending && level <= nextlevel {
            return false;
        }

        // Check difference between adjacent numbers
        let diff = (level - nextlevel).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let mut found = 0;

    for line in reader.lines() {
        let line = line?;
        let str_levels: Vec<&str> = line.split_whitespace().collect();
        let levels: Vec<i32> = str_levels
            .iter()
            .map(|s| s.parse::<i32>())
            .collect::<Result<_, _>>()?;

        // First check if the sequence is safe without removing any number
        if is_safe_sequence(&levels) {
            found += 1;
        }
    }

    Ok(found)
}

pub fn part2() -> Result<i32, Box<dyn Error>> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let mut found = 0;

    for line in reader.lines() {
        let line = line?;
        let str_levels: Vec<&str> = line.split_whitespace().collect();
        let mut levels: Vec<i32> = str_levels
            .iter()
            .map(|s| s.parse::<i32>())
            .collect::<Result<_, _>>()?;

        // First check if the sequence is safe without removing any number
        if is_safe_sequence(&levels) {
            found += 1;
            continue;
        }

        // Try removing each number one at a time
        for i in 0..levels.len() {
            let removed = levels.remove(i);
            if is_safe_sequence(&levels) {
                found += 1;
                break;
            }
            levels.insert(i, removed);
        }
    }

    Ok(found)
}
