use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use itertools::Itertools;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_of_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
// TODO: Get big boy input
// const INPUT_FILE_BIG_BOY: &str = concatcp!("input/", "bigboy", DAY, ".txt");

const TEST: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9

"#;

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    // TODO: Solve Part 1 of the puzzle
    let mut result = 0;

    for line in reader.lines() {
        let mut report_is_safe = true;

        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let elements
            = line.split_whitespace()
            .map(
                |s| s.parse::<i32>()
                    .with_context(|| format!("Failed to parse number {}", s)))
            .collect::<Result<Vec<_>>>()?;

        let mut is_first_element = true;
        let mut is_decreasing = true;

        for window in elements.windows(2) {
            if let [first, second] = window {
                let diff = (first - second).abs();
                if diff >= 1 && diff <= 3 || diff == 3 {
                    if is_first_element {
                        is_decreasing = first > second;
                        is_first_element = false;
                    } else if (first < second && is_decreasing) || (first > second && !is_decreasing) {
                        report_is_safe = false;
                        break;
                    }
                } else {
                    report_is_safe = false;
                    break;
                }
            }
        }

result += if report_is_safe { 1 } else { 0 };
}

Ok(result)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    // TODO: Uncomment for big boy result
    // let result = time_snippet!(part1(
    //     BufReader::new(File::open(INPUT_FILE_BIG_BOY)?)
    // )?);
    // println!("Result (big boy) = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
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
    fn test_part1_1() {
        let input = r#"
            1 3 5 7 9
            1 3 5 7 9
              9 3 5 8
            23 25 27
            113 114 155
            100 101 102 103 105 106 104
        "#;

        let result = part1(input.as_bytes()).unwrap();

        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_2() {
        let input = r#"
            100 101 102 103 105 106 104
            8 9 8
        "#;

        let result = part1(input.as_bytes()).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_3() {
        let input = r#"
            100 101 102 103 105 106 103
        "#;

        let result = part1(input.as_bytes()).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_4() {
        let input = r#"
            7 6 4 2 1
        "#;

        let result = part1(input.as_bytes()).unwrap();

        assert_eq!(result, 1);
    }

    #[test]
    fn test_test_input() {
        let input = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "#;

        let result = part1(input.as_bytes()).unwrap();

        assert_eq!(result, 2);
    }
}
