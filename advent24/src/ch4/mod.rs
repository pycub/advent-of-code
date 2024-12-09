use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn horizontal(matrix: &Vec<Vec<String>>) -> u32 {
    let found = 0;

    found
}

pub fn part1(filepath: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<String>> = vec![];

    for line in reader.lines() {
        let line = line?;
        let row: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        matrix.push(row);
    }

    println!("{:?}", matrix);

    Ok(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_with_test_input() {
        assert_eq!(18, part1("src/ch4/test-input.txt").unwrap())
    }
}
