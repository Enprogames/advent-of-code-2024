use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_of_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
// TODO: Get big boy input
const INPUT_FILE_BIG_BOY: &str = concatcp!("input/", "bigboy", DAY, ".txt");

const TEST_PT1: &str = r#"\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#;

const TEST_PT2: &str = r#"\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#;

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
    let str_len = str.len();
    let sub_len = sub_str.len();
    if str_len < sub_len
    {
        return None;
    }

    let lower_bound = start.unwrap_or(0);
    let upper_bound = end.unwrap_or(str_len);

    if lower_bound >= upper_bound
        || (upper_bound - lower_bound) <= sub_len-1
        || upper_bound > str_len {
        return None;
    }

    let search_range = &str[lower_bound..upper_bound].as_bytes();
    for i in 0..search_range.len() {
        if lower_bound + i + sub_len > upper_bound {
            return None;
        }

        for (j, c_sub) in sub_str.char_indices() {
            let c_str = search_range[i+j] as char;
            if c_sub != c_str {  // Nonmatching. End immediately.
                break;
            } else if c_sub == c_str && j == sub_len-1 {  // The final character matches, so return the location.
                return Some((lower_bound + i, lower_bound + i + sub_len));
            }
        }
    }

    None
}

fn sum_all_mul_pairs_in_range(str: &str, start: usize, end: usize) -> usize {
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

/// All ranges of values where mul will be enabled.
/// So all ranges from the start to the first don't, and then repeatedly from the next do()
/// to the next don't().
fn do_dont_bounds(str: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut finding_do = false;
    let mut string_to_find = "don't()";
    let mut last_upper_bound = 0;

    while let Some((lower_bound, upper_bound))
        = find_next_substring_in_range(&str, string_to_find, Some(last_upper_bound), None)
    {
        if finding_do {
            string_to_find = "don't()";
        } else {
            result.push((last_upper_bound, lower_bound));
            string_to_find = "do()";
        }
        finding_do = !finding_do;

        // do some processing between the bounds of the string, between the "don't()" and the "do()"
        last_upper_bound = upper_bound;
    }

    if !finding_do {
        result.push((last_upper_bound, str.len()));
    }

    result.into_iter()
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut result: usize = 0;

    // filter out any error lines and collect all lines into one long string
    let flattened_input: String = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    result += do_dont_bounds(flattened_input.as_str())
        .map(
            |(start, end)| sum_all_mul_pairs_in_range(flattened_input.as_str(), start, end)
        )
        .sum::<usize>();

    Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    assert_eq!(161, part1(BufReader::new(TEST_PT1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    // let result = time_snippet!(part1(
    //     BufReader::new(File::open(INPUT_FILE_BIG_BOY)?)
    // )?);
    // println!("Result (big boy) = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    assert_eq!(48, part2(BufReader::new(TEST_PT2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

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
    #[ignore]  // TODO: Figure out why this is failing
    fn test_part1_simple() {
        let input = r#"
            mul(5, 6)
        "#;

        let result = part1(input.as_bytes()).unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    #[ignore]  // TODO: Figure out why this is failing
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
                assert_eq!(end, 4);
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
                assert_eq!(end, 10);
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

    #[test]
    fn test_find_do_dont_bounds_no_match()
    {
        let str: &str = "aaaadon'taaaadoaaadodon't(aaadon't)";

        let mut bounds = do_dont_bounds(str);

        assert_eq!(bounds.next(), Some((0, str.len())));
    }

    #[test]
    fn test_find_do_dont_bounds_single_dont()
    {
        let str: &str = "aaaadon'taaaadoaaadodon't()aaadon't)";

        let mut bounds = do_dont_bounds(str);

        assert_eq!(bounds.next(), Some((0, 20)));
    }

    #[test]
    fn test_find_do_dont_bounds_simple_1()
    {
        let str: &str = "aaaadon't()aaaado()aaado()don't()aaadon't()";

        let mut bounds = do_dont_bounds(str);

        assert_eq!(bounds.next(), Some((0, 4)));
        assert_eq!(bounds.next(), Some((19, 26)));
    }

    #[test]
    fn test_find_do_dont_bounds_simple_2()
    {
        let str: &str = "aaaadon't()aaaado()a";

        let mut bounds = do_dont_bounds(str);

        assert_eq!(bounds.next(), Some((0, 4)));
        assert_eq!(bounds.next(), Some((19, str.len())));
    }

    #[test]
    fn test_part2_empty_input() {
        let input = "";
        assert_eq!(part2(input.as_bytes()).unwrap(), 0);
    }

    #[test]
    fn test_part2_no_do_or_dont() {
        let input = "mul(2,3) mul(4,5) mul(6,7)";
        assert_eq!(part2(input.as_bytes()).unwrap(), 6 + 20 + 42);
    }

    #[test]
    fn test_part2_multiple_consecutive_dont() {
        let input = "mul(2,3) don't() don't() mul(4,5) don't() mul(6,7)";
        assert_eq!(part2(input.as_bytes()).unwrap(), 6);
    }

    #[test]
    fn test_part2_multiple_consecutive_do() {
        let input = "mul(2,3) don't() mul(4,5) do() do() mul(6,7)";
        assert_eq!(part2(input.as_bytes()).unwrap(), 48);
    }

    #[test]
    fn test_part2_nested_do_dont() {
        let input = "mul(2,3) don't() do() don't() mul(4,5) do() mul(6,7)";
        assert_eq!(part2(input.as_bytes()).unwrap(), 48);
    }

    #[test]
    fn test_part2_multiline() {
        let input = r#"mul(2,3)
        don't() do() don't()
        mul(4,5) do()
        mul(6,7)"#;
        assert_eq!(part2(input.as_bytes()).unwrap(), 48);
    }

    #[test]
    fn test_part2_invalid_mul_syntax() {
        let input = "mul(2,3) mul[4,5] don't() mul(6,7) mul(x,y)";
        assert_eq!(part2(input.as_bytes()).unwrap(), 6);
    }

    #[test]
    fn test_long_input() {
        let input = "mul(2,3) ".repeat(1000) + "don't() mul(4,5) do() mul(6,7)";
        assert_eq!(part2(input.as_bytes()).unwrap(), 6 * 1000 + 42);
    }

    #[test]
    fn test_part2_simple_1()
    {
        assert_eq!(part2("aaa<'-:adon't".as_bytes()).unwrap(), 0);
        assert_eq!(part2("mul(5,6)".as_bytes()).unwrap(), 30);
        assert_eq!(part2("f<'-ffxmul(5,6)??fads<'-fj?mul(13,24)<'-".as_bytes()).unwrap(), 342);
        assert_eq!(part2("how()aaaamul(5,6)mul(5,6]mul[5,6]don't()".as_bytes()).unwrap(), 30);
        assert_eq!(part2("aaaamul(5,6)don't()mul(100,200)".as_bytes()).unwrap(), 30);
    }

    #[test]
    fn test_part2_complex_1()
    {
        assert_eq!(part2("aaaamul(5,6)don't()asdfadfmul(6,3)aadfasdfdo()mul(100,200)".as_bytes()).unwrap(), 20030);
    }

    #[test]
    fn test_part2_complex_2()
    {
        assert_eq!(part2("aaaamul(5,6)don't()asdfadfmul(6,3)aadfasdfdo()too_long_mul(1000,1)mul(100,200)".as_bytes()).unwrap(), 20030);
    }

    #[test]
    fn test_part2_complex_3()
    {
        assert_eq!(part2("don't()don't()don't()don't()mul(5,6)do()mul(5,6)".as_bytes()).unwrap(), 30);
        assert_eq!(part2("don't()don't()don't()mul(5,6)do()mul(5,6)".as_bytes()).unwrap(), 30);
    }

    #[test]
    fn test_part2_complex_4()
    {
        assert_eq!(part2("aaaamul(5,6)don't()asdfadfmul(6,3)aadfasdfdo()too_long_mul(1000,1)mul(100,200)don't()mul(100,200)do()mul(100,200)do()mul(100,200)do()mul(100,200)do()don't()".as_bytes()).unwrap(), 80030);
    }
}
