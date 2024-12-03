mod ch1;
mod ch2;
mod ch3;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(ch1::part1()?, 1830467);
    assert_eq!(ch1::part2()?, 26674158);
    println!("Challenge1 - solved");

    assert_eq!(334, ch2::part1()?);
    assert_eq!(400, ch2::part2()?);
    println!("Challenge2 - solved");

    println!("{:?}", ch3::part1()?);

    Ok(())
}
