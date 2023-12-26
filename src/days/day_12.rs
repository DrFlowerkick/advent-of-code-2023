//!day_12.rs

use anyhow::Result;

trait Spring {
    fn is_spring(&self) -> bool;
    fn is_operational(&self) -> bool;
    fn is_or_may_be_undamaged(&self) -> bool;
    fn is_or_may_be_damaged(&self) -> bool;
}

impl Spring for char {
    fn is_spring(&self) -> bool {
        match self {
            '.' | '#' | '?' => true,
            _ => false,
        }
    }
    fn is_operational(&self) -> bool {
        match self {
            '.' => true,
            _ => false,
        }
    }
    fn is_or_may_be_undamaged(&self) -> bool {
        match self {
            '.' | '?' => true,
            _ => false,
        }
    }
    fn is_or_may_be_damaged(&self) -> bool {
        match self {
            '#' | '?' => true,
            _ => false,
        }
    }
}

fn springs_and_damaged_clusters(input: &str) -> (&str, Vec<usize>) {
    input
        .trim()
        .split_once(' ')
        .map(|(springs, d)| {
            if springs
                .trim()
                .chars()
                .filter(|c| !c.is_spring())
                .next()
                .is_some()
            {
                panic!("bad spring char");
            }
            let damaged_clusters: Vec<usize> = d
                .split(',')
                .map(|s| s.parse::<usize>().expect("bad input"))
                .collect();
            (springs.trim(), damaged_clusters)
        })
        .unwrap()
}

fn get_max_range_end(springs: &str, damaged_clusters: &[usize]) -> usize {
    springs.len() - if damaged_clusters.len() == 1 {
        0
    } else {
        damaged_clusters[1..].len() + damaged_clusters[1..].iter().sum::<usize>()
    }
}

fn different_arrangements(springs: &str, damaged_clusters: &[usize]) -> usize {
    let max_range_end = get_max_range_end(springs, damaged_clusters);
    let mut index = 0;
    let mut num_different_arrangements = 0;
    loop {
        match springs
            .chars()
            .enumerate()
            .position(|(i, s)| i >= index && s.is_or_may_be_damaged())
        {
            Some(range_start) => {
                let range_end = range_start + damaged_clusters[0];
                if range_end > max_range_end {
                    // cluster(s) cannot fit
                    return num_different_arrangements;
                }
                if springs[range_start..range_end]
                    .chars()
                    .filter(|s| s.is_operational())
                    .next()
                    .is_none()
                    && (range_start == 0
                        || springs
                            .chars()
                            .nth(range_start - 1)
                            .unwrap()
                            .is_or_may_be_undamaged())
                    && (range_end == springs.len()
                        || springs
                            .chars()
                            .nth(range_end)
                            .unwrap()
                            .is_or_may_be_undamaged())
                {
                    // cluster fit's
                    if damaged_clusters.len() == 1 {
                        num_different_arrangements += 1;
                    } else if range_end + 1 >= springs.len() {
                        // no more springs left after current cluster
                        return num_different_arrangements;
                    } else {
                        // search for next cluster
                        num_different_arrangements += different_arrangements(
                            &springs[(range_end + 1)..],
                            &damaged_clusters[1..],
                        );
                    }
                }
                index = range_start + 1;
            }
            None => return num_different_arrangements,
        }
    }
}

pub fn day_12() -> Result<()> {
    let input = include_str!("../../assets/day_12.txt");
    let mut sum_different_arrangements = 0;
    for line in input.lines() {
        let (springs, damaged_clusters) = springs_and_damaged_clusters(line);
        let line_arrangements = different_arrangements(&springs[..], &damaged_clusters[..]);
        if line_arrangements == 0 {
            eprintln!("line_arrangements == 0 at {} {:?}", springs, damaged_clusters);
        }
        sum_different_arrangements += line_arrangements;
    }

    println!("result day 12 part 1: {}", sum_different_arrangements);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_arrangements_01() {
        let input = "???.### 1,1,3";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(&springs[..], &damaged_clusters[..]);
        println!(
            "test 01 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 1);
    }

    #[test]
    fn test_arrangements_02() {
        let input = ".??..??...?##. 1,1,3";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(&springs[..], &damaged_clusters[..]);
        println!(
            "test 02 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 4);
    }

    #[test]
    fn test_arrangements_03() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(&springs[..], &damaged_clusters[..]);
        println!(
            "test 03 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 1);
    }

    #[test]
    fn test_arrangements_04() {
        let input = "????.#...#... 4,1,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(&springs[..], &damaged_clusters[..]);
        println!(
            "test 04 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 1);
    }

    #[test]
    fn test_arrangements_05() {
        let input = "????.######..#####. 1,6,5";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(&springs[..], &damaged_clusters[..]);
        println!(
            "test 05 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 4);
    }

    #[test]
    fn test_arrangements_06() {
        let input = "?###???????? 3,2,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(&springs[..], &damaged_clusters[..]);
        println!(
            "test 06 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 10);
    }

    #[test]
    fn test_arrangements_07() {
        let input = ".#??#..???#..?? 2,2,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(&springs[..], &damaged_clusters[..]);
        println!(
            "test 07 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 10);
    }

    #[test]
    fn test_arrangements_08() {
        let input = ".#??#..???#..?? 2,1,2";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(&springs[..], &damaged_clusters[..]);
        println!(
            "test 08 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 12);
    }

    #[test]
    fn test_arrangements_09() {
        let input = "????#?.??? 2,1,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(&springs[..], &damaged_clusters[..]);
        println!(
            "test 09 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 10);
    }

    #[test]
    fn test_arrangements_10() {
        let input = "??.??????#???#?????# 1,1,7,3,1";
        let (springs, damaged_clusters) = springs_and_damaged_clusters(input);
        eprintln!("{}", springs);
        eprintln!("{:?}", damaged_clusters);
        let test_different_arrangements =
            different_arrangements(&springs[..], &damaged_clusters[..]);
        println!(
            "test 10 result day 12 part 1: {}",
            test_different_arrangements
        );
        assert_eq!(test_different_arrangements, 13);
    }
}
