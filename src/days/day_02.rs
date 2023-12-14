//!day_02.rs

use anyhow::Result;

const digits: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn day_02(input: &str) -> Result<()> {
    let sum: u32 = input
        .lines()
        .map(|l| {
            match l.chars().filter(|c| c.is_ascii_digit()).next() {
                Some(left) => {
                    let right = l.chars().rev().filter(|c| c.is_ascii_digit()).next().unwrap();
                    let mut number = String::new();
                    number.push(left);
                    number.push(right);
                    number.parse::<u32>().unwrap()
                },
                None => 0,
            }
        })
        .sum();
    println!("result day 01: {}", sum);
    Ok(())
}