use std::result::Result::Ok;
use anyhow::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use std::collections::{BTreeSet, HashMap};

use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_of_code_2024::*;
use log::error;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_number_pairs_file<R: BufRead>(reader: R) -> io::Result<(Vec<usize>, Vec<usize>)> {
        let mut left_values = Vec::new();
        let mut right_values = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split_whitespace();
            if let (Some(left_str), Some(right_str)) = (parts.next(), parts.next()) {
                match (left_str.parse::<usize>(), right_str.parse::<usize>()) {
                    (Ok(left_val), Ok(right_val)) => {
                        left_values.push(left_val);
                        right_values.push(right_val);
                    }
                    (Err(_), _) | (_, Err(_)) => {
                        eprintln!("Invalid input: failed to parse '{}' or '{}' as integers.",
                                  left_str, right_str);
                    }
                }
            } else {
                eprintln!("Could not parse input as two separate space-separated strings")
            }
        }

        Ok((left_values, right_values))
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;

        let (mut left_values, mut right_values) = parse_number_pairs_file(reader)?;

        left_values.sort_unstable();
        right_values.sort_unstable();

        // Create iterators for both sets
        let mut left_iter = left_values.iter();
        let mut right_iter = right_values.iter();

        // Initialize first elements
        let mut left_next = left_iter.next();
        let mut right_next = right_iter.next();

        while let (Some(&left_val), Some(&right_val)) = (left_next, right_next) {
            let mut distance: usize = 0;
            if (left_val < right_val) {
                distance = (right_val - left_val) as usize;
            } else {
                distance = (left_val - right_val) as usize;
            }
            // println!("left: '{}', right: '{}', distance: '{}'.", left_val, right_val, distance);
            result += distance;
            left_next = left_iter.next();
            right_next = right_iter.next();
        }
        Ok(result)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    assert_eq!(2031679, result);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut result: usize = 0;

        let (mut left_values, mut right_values) = parse_number_pairs_file(reader)?;

        let mut occurrence_counts = HashMap::new();

        for right_val in right_values {
            *occurrence_counts.entry(right_val).or_insert(0) += 1;
        }

        for left_val in left_values {
            // println!("Finding occurrence count for '{}'.", left_val);
            if let Some(&occurrence_count) = occurrence_counts.get(&left_val) {
                result += left_val * occurrence_count;
            }
        }

        Ok(result)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    assert_eq!(19678534, result);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
