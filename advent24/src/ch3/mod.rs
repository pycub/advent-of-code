use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "src/ch3/input.txt";

#[derive(Debug)]
enum EventType {
    Do,
    Dont,
    Mul(u32, u32),
}

fn seek_mul(window: &str, re: &Regex) -> Result<i32, Box<dyn Error>> {
    let mut total = 0;

    for captured in re.captures_iter(window) {
        if let (Some(x), Some(y)) = (captured.get(1), captured.get(2)) {
            total += x.as_str().parse::<i32>()? * y.as_str().parse::<i32>()?;
        }
    }

    Ok(total)
}

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"mul\s*\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)")?;
    let mut total: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        total += seek_mul(&line, &re)?;
    }

    Ok(total)
}

pub fn part2() -> Result<u32, Box<dyn Error>> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line = line?;

        let events = find_events(&line);
        let mut enabled = true;
        for event in events {
            match event.0 {
                EventType::Do => {
                    enabled = true;
                }
                EventType::Dont => {
                    enabled = false;
                }
                EventType::Mul(x, y) => {
                    if enabled {
                        sum += x * y;
                    }
                }
            }
        }
    }

    Ok(sum)
}

fn find_events(content: &str) -> Vec<(EventType, usize)> {
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let mul_re = Regex::new(r"\bmul\s*\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)").unwrap();

    let do_matches: Vec<_> = do_re
        .find_iter(content)
        .map(|mat| (EventType::Do, mat.start()))
        .collect();
    let dont_matches: Vec<_> = dont_re
        .find_iter(content)
        .map(|mat| (EventType::Dont, mat.start()))
        .collect();
    let mul_matches: Vec<_> = mul_re
        .captures_iter(content)
        .map(|caps| {
            let x: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let y: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
            (EventType::Mul(x, y), caps.get(0).unwrap().start())
        })
        .collect();

    let mut events = Vec::new();
    events.extend(do_matches);
    events.extend(dont_matches);
    events.extend(mul_matches);

    // Sort events by their start positions
    events.sort_by_key(|&(_, pos)| pos);

    events
}
