//!lib.rs

pub mod days;

use anyhow::Result;

pub fn run() -> Result<()> {
    let input_01 = include_str!("../assets/day_01.txt");
    days::day_01::day_01(input_01)?;
    //days::day_02::day_02(input_01)?;
    
    Ok(())
}