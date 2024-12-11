use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn remove_repetitive_chars(s: &str) -> String {
    s.chars().fold(String::new(), |mut result, c| {
        if result.chars().last() != Some(c) {
            result.push(c);
        }
        result
    })
}

fn horizontal(matrix: &Vec<Vec<char>>, q: &str) -> usize {
    let mut found: usize = 0;

    for row in 0..matrix.len() {
        let s: String = matrix[row].iter().collect();
        let s = remove_repetitive_chars(&s);
        found += s.matches(q).count();
    }

    found
}

fn vertical(matrix: &Vec<Vec<char>>, q: &str) -> usize {
    let mut found: usize = 0;

    for col in 0..matrix[0].len() {
        let mut s: String = String::new();

        for row in 0..matrix.len() {
            s.push(matrix[col][row]);
        }

        let s = remove_repetitive_chars(&s);

        println!("Found {} {q} in {s}", s.matches(q).count());
        found += s.matches(q).count();
    }

    found
}

fn diognal(matrix: &Vec<Vec<char>>, q: &str) -> usize {
    let mut found = 0;

    // TODO: Implement diognal traversal

    found
}

pub fn part1(filepath: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let q: String = String::from("XMAS");
    let reveresed_q: String = q.chars().rev().collect();
    let mut found = 0;

    let mut matrix: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    found += horizontal(&matrix, &q);
    found += horizontal(&matrix, &reveresed_q);

    found += vertical(&matrix, &q);
    found += vertical(&matrix, &reveresed_q);

    println!("{found}");

    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_with_test_input() {
        assert_eq!(18, part1("src/ch4/test-input.txt").unwrap())
    }
}
