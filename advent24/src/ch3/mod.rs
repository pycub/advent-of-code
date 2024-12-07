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

pub fn part1() -> Result<u32, Box<dyn Error>> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let mut total: u32 = 0;

    for line in reader.lines() {
        let line = line?;

        let events = find_events(&line);
        for event in events {
            match event.0 {
                EventType::Do => {}
                EventType::Dont => {}
                EventType::Mul(x, y) => {
                    total += x * y;
                }
            }
        }
    }

    Ok(total)
}

pub fn part2() -> Result<u32, Box<dyn Error>> {
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"mul\((\d*),(\d*)\)|don't\(\)|do\(\)")?;
    let mut enabled = true;
    let mut total: u32 = 0;
    let mut instructions = vec![];

    for line in reader.lines() {
        for capture in re.captures_iter(&line?) {
            let get_int = |idx: usize| capture.get(idx).map_or("i", |m| m.as_str()).parse();
            let instr_raw = capture.get(0).map_or("", |m| m.as_str());

            if instr_raw.contains("mul") {
                let left = get_int(1)?;
                let right = get_int(2)?;
                instructions.push(EventType::Mul(left, right));
            } else if instr_raw.contains("don't") {
                instructions.push(EventType::Dont);
            } else if instr_raw.contains("do") {
                instructions.push(EventType::Do);
            }
        }
    }

    for instr in &instructions {
        match instr {
            EventType::Do => enabled = true,
            EventType::Dont => enabled = false,
            EventType::Mul(x, y) => {
                if enabled {
                    total += x * y
                }
            }
        }
    }

    Ok(total)
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
