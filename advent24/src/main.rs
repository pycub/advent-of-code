mod ch1;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(ch1::part1()?, 1830467);
    assert_eq!(ch1::part2()?, 26674158);
    println!("Challenge1 - solved");

    Ok(())
}
