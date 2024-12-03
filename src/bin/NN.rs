use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_of_code_2024::*;

const DAY: &str = "NN"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
// TODO: Get big boy input
// const INPUT_FILE_BIG_BOY: &str = concatcp!("input/", "bigboy", DAY, ".txt");

const TEST: &str = r#"\

"#; // TODO: Enter test input

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    // TODO: Solve Part 1 of the puzzle
    let answer = reader.lines().flatten().count();
    Ok(answer)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    // TODO: Solve Part 2 of the puzzle
    Ok(0)
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
