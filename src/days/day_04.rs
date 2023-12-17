//!day_04.rs

use anyhow::Result;
use my_lib::my_array::MyArray;

const NUM_CARDS: usize = 201;

pub fn day_04() -> Result<()> {
    let input = include_str!("../../assets/day_04.txt");
    let mut result_part1 = 0;
    let mut num_per_card: MyArray<u32, NUM_CARDS> = MyArray::init(1, NUM_CARDS);
    for (current_card, line) in input
        .lines()
        .map(|l| l.split_once(':').unwrap().1.trim())
        .enumerate()
    {
        let (win_str, my_str) = line.split_once('|').unwrap();
        let winners: Vec<u8> = win_str
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect();
        let my_numbers: Vec<u8> = my_str
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse::<u8>().unwrap())
            .collect();
        let my_winners = winners
            .iter()
            .filter(|w| my_numbers.iter().find(|m| m == w).is_some())
            .count();
        if my_winners > 0 {
            result_part1 += 2_u32.pow((my_winners - 1) as u32);
            let num_curent_card = num_per_card[current_card];
            for index in current_card + 1..=current_card + my_winners {
                if index < NUM_CARDS {
                    *num_per_card.get_mut(index).unwrap() += num_curent_card;
                }
            }
        }
    }

    println!("result day 04 part 1: {}", result_part1);
    println!("result day 04 part 2: {}", num_per_card.iter().sum::<u32>());
    Ok(())
}
