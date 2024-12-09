mod ch1;
mod ch2;
mod ch3;
mod ch4;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(ch1::part1()?, 1830467);
    assert_eq!(ch1::part2()?, 26674158);
    println!("Day3 - solved");

    assert_eq!(334, ch2::part1()?);
    assert_eq!(400, ch2::part2()?);
    println!("Day2 - solved");

    assert_eq!(188192787, ch3::part1()?);
    assert_eq!(113965544, ch3::part2()?);
    println!("Day3 - solved");

    ch4::part1("src/ch4/test-input.txt")?;

    Ok(())
}
