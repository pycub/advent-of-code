use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "src/ch2/input.txt";

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let mut found = 0;

    'outer: for line in reader.lines() {
        let line = line?;
        let levels: Vec<&str> = line.split_whitespace().collect();
        let levels_len = levels.len();

        // Skip if less than 3 levels
        if levels_len < 3 {
            continue;
        }

        // Convert first two numbers to determine if ascending or descending
        let first = levels[0].parse::<i32>()?;
        let second = levels[1].parse::<i32>()?;
        let ascending = first < second;

        // Check each pair of adjacent numbers
        for index in 0..levels_len - 1 {
            let level = levels[index].parse::<i32>()?;
            let nextlevel = levels[index + 1].parse::<i32>()?;

            // Check if sequence maintains ascending/descending pattern
            if ascending && level >= nextlevel {
                continue 'outer;
            }
            if !ascending && level <= nextlevel {
                continue 'outer;
            }

            // Check difference between adjacent numbers
            let diff = (level - nextlevel).abs();
            if diff < 1 || diff > 3 {
                continue 'outer;
            }
        }

        // If we get here, the report is safe
        found += 1;
    }

    Ok(found)
}
