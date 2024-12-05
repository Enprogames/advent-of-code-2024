use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_of_code_2024::*;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
// TODO: Get big boy input
const INPUT_FILE_BIG_BOY: &str = concatcp!("input/", "bigboy", DAY, ".txt");

const TEST: &str = r#"\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#; // TODO: Enter test input

const MULT_RE: &str = r"mul\((\d{1,3}),(\d{1,3})\)";

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut result: usize = 0;

    let re = Regex::new(MULT_RE)?;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        result += re.captures_iter(line)
            .map(|cap| {
                // lammma remove
                let left: usize = cap[1].parse().with_context(|| format!("Error parsing {}", line))?;
                let right: usize  = cap[2].parse().with_context(|| format!("Error parsing {}", line))?;

                Ok(left * right)
            }
            ).collect::<Result<Vec<_>>>()?
            .into_iter()
            .sum::<usize>();
    }

    Ok(result)
}

/// Finds the first occurrence of a string within a specified range.
/// Either range can be unprovided, in which case, it is treated as if the entire string will be searched.
///
/// Returns (usize, usize): The start and end of the string
fn find_next_substring_in_range(
    str: &str,
    sub_str: &str,
    start: Option<usize>,
    end: Option<usize>) -> Option<(usize, usize)>
{
    let mut found_index = 0;

    let str_len = str.len();
    let sub_len = sub_str.len();
    if str_len < sub_len
    {
        return None;
    }

    let lower_bound = start.unwrap_or(0);
    let upper_bound = end.unwrap_or(str_len-1);

    if lower_bound >= upper_bound
        || (upper_bound - lower_bound) <= sub_len-1
        || upper_bound >= str_len {
        return None;
    }

    let search_range = &str[lower_bound..upper_bound];
    for (i, c_str) in str.char_indices() {
        if lower_bound + i + sub_len > upper_bound {
            return None;
        }

        if &search_range[i..i + sub_len] == sub_str {
            return Some((lower_bound + i, lower_bound + i + sub_len));
        }
    }

    None
}

fn sum_all_mult_pairs_in_range(str: &str, start: usize, end: usize) -> usize {
    let slice = &str[start..end];

    let re = Regex::new(MULT_RE).unwrap();

    re.captures_iter(slice)
        .map(|cap| {
            // lammma remove
            let left: usize = cap[1].parse().with_context(|| format!("Error parsing {}", slice))?;
            let right: usize  = cap[2].parse().with_context(|| format!("Error parsing {}", slice))?;

            Ok(left * right)
        }
        ).collect::<Result<Vec<_>>>().unwrap()
        .into_iter()
        .sum::<usize>()
}

// fn do_dont_bounds(str: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
//     let (lower_bound, upper_bound) = find_next_substring_in_range(str, "don't()", None, None).unwrap();
//
//     while <values are found> {
//         // Find next occurrence of "do()" that is above upper_bound
//
//         // do some processing between the bounds of the string, between the "don't()" and the "do()"
//
//     }
// }

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut result: usize = 0;

    // for line in reader.lines() {
    //     let line = line?;
    //     let line = line.trim();
    //     if line.is_empty() {
    //         continue;
    //     }
    //
    //     result += do_dont_bounds(line)
    //         .map(
    //             |(start, end)| find_all_mul_pairs_in_range(line, start, end)
    //         )
    //         .flatten()
    //         .map(|(first, second)| first * second)
    //         .sum::<usize>();
    // }

    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    let result = time_snippet!(part1(
        BufReader::new(File::open(INPUT_FILE_BIG_BOY)?)
    )?);
    println!("Result (big boy) = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);

    // TODO: Uncomment for big boy result
    // let result = time_snippet!(part2(
    //     BufReader::new(File::open(INPUT_FILE_BIG_BOY)?)
    // )?);
    // println!("Result (big boy) = {}", result);
    //endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_simple() {
        let input = r#"
            mul(5, 6)
        "#;

        let result = part1(input.as_bytes()).unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    fn test_part1_invalid_character() {
        let input = r#"
            #*mul(5, 6)%
        "#;
        let result = part1(input.as_bytes()).unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    fn test_regex() {
        let input = r#" mul(1,333)&& mul(222,3) |@()%)&(^! mut(mut(555, 3)) mul(2222,3) *mul(1) ^&  mul(33) mul(553,333)&&"#;
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let result = re.captures_iter(input).count();

        // Extract and print the first match
        for cap in re.captures_iter(input) {
            let first_value: usize = cap[1].parse().unwrap(); // Parse the first captured group
            let second_value: usize = cap[2].parse().unwrap(); // Parse the second captured group
            println!("First value: {}, Second value: {}", first_value, second_value);
        }

        assert_eq!(result, 3);
    }

    #[test]
    fn test_find_next_substring_in_range_simple_1() {
        let str: &str = "do()";

        let bounds: Option<(usize, usize)>
            = find_next_substring_in_range(str, "do()", None, None);

        match bounds {
            Some((start, end)) => {
                assert_eq!(start, 0);
                assert_eq!(end, 3);
            },
            _ => {
                assert!(false, "The match was not found");
            }
        }
    }

    #[test]
    fn test_find_next_substring_in_range_simple_2() {
        let str: &str = "aaaaa do() bbbbb";

        let bounds: Option<(usize, usize)>
            = find_next_substring_in_range(str, "do()", None, None);

        match bounds {
            Some((start, end)) => {
                assert_eq!(start, 6);
                assert_eq!(end, 9);
            },
            _ => {
                assert!(false, "The match was not found");
            }
        }
    }

    #[test]
    fn test_find_next_substring_in_range_exact_range() {
        let str: &str = "fffffdo()ffff";

        let bounds: Option<(usize, usize)>
            = find_next_substring_in_range(str, "do()", Some(5), Some(9));

        match bounds {
            Some((start, end)) => {
                assert_eq!(start, 5);
                assert_eq!(end, 9);
            },
            _ => {
                assert!(false, "The match was not found");
            }
        }
    }

    #[test]
    fn test_find_next_substring_in_range_too_small_range() {
        let str: &str = "fffffdo()ffff";

        let bounds: Option<(usize, usize)>
            = find_next_substring_in_range(str, "do()", Some(5), Some(8));

        match bounds {
            Some((start, end)) => {
                assert!(false, "The match was found, which is not what we want");
            },
            _ => {
                assert!(true, "The match was not found, which is what we want");
            }
        }
    }
}
