// use std::error::Error;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_of_code_2024::*;

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
// TODO: Get big boy input
// const INPUT_FILE_BIG_BOY: &str = concatcp!("input/", "bigboy", DAY, ".txt");

const TEST: &str = r#"
    47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47
"#; // TODO: Enter test input

fn parse_file<R: BufRead>(mut reader: R) -> Result<(Vec<(usize, usize)>, Vec<Vec<usize>>)> {
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let mut parts = content.split("\n\n");

    let part1 = parts.next()
        .unwrap_or("")
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut split = l.split("|");
            let first_str = split.next().ok_or_else(|| anyhow!("Missing first number in part1 line"))?.trim();
            let second_str = split.next().ok_or_else(|| anyhow!("Missing first number in part1 line"))?.trim();
            if split.next().is_some() {
                Err(anyhow!("Extra data in part1 line"))
            } else {
                let first = first_str.parse::<usize>().map_err(|_| anyhow!("Invalid number '{}'", first_str.trim()))?;
                let second = second_str.parse::<usize>().map_err(|_| anyhow!("Invalid number '{}'", second_str.trim()))?;
                Ok((first, second))
            }
        })
        .collect::<Result<Vec<(usize, usize)>>>()?;

    let part2 = parts.next()
        .unwrap_or("")
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(",")
                .map(|s| s.trim().parse::<usize>().map_err(|_| anyhow!("Invalid number '{}'", s.trim())))
                .collect::<Result<Vec<usize>>>()
        })
        .collect::<Result<Vec<Vec<usize>>>>()?;

    Ok((part1, part2))
}

fn parse_adj_list(edge_tuples: Vec<(usize, usize)>) -> Result<Vec<Vec<usize>>> {
    // find max number in among all tuple-pairs
    let max = edge_tuples.iter().map(|(a, b)| *a.max(b)).max().unwrap_or(0);
    
    let mut adj_list = vec![Vec::new(); max + 1];

    for (a, b) in edge_tuples {
        adj_list[a].push(b);
    }
    Ok(adj_list)
}

fn verify

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    // TODO: Solve Part 1 of the puzzle
    let result: usize = 0;

    let (part1, part2) = parse_file(reader)?;

    let adj_list = parse_adj_list(part1);
    Ok(result)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    // TODO: Solve Part 2 of the puzzle
    Ok(0)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_empty_input() {
        let input = "";
        let cursor = Cursor::new(input);
        let (part1, part2) = parse_file(cursor).unwrap();
        assert!(part1.is_empty());
        assert!(part2.is_empty());
    }

    #[test]
    fn test_valid_input() {
        let input = "\
        47|53
        97|13

        75,47,61
        97,13
        ";
        let cursor = Cursor::new(input);
        let (part1, part2) = parse_file(cursor).unwrap();
        assert_eq!(part1, vec![(47,53), (97,13)]);
        assert_eq!(part2, vec![vec![75,47,61], vec![97,13]]);
    }

    #[test]
    fn test_invalid_number_part1() {
        let input = "abc|53\n\n75,47";
        let cursor = Cursor::new(input);
        let result = parse_file(cursor);
        assert!(result.is_err());
        let err = format!("{}", result.unwrap_err());
        assert!(err.contains("Invalid number 'abc'"));
    }

    #[test]
    fn test_invalid_number_part2() {
        let input = "47|53\n\n75,xyz";
        let cursor = Cursor::new(input);
        let result = parse_file(cursor);
        assert!(result.is_err());
        let err = format!("{}", result.unwrap_err());
        assert!(err.contains("Invalid number 'xyz'"));
    }

    #[test]
    fn test_extra_data_part1_line() {
        let input = "47|53|99\n\n75,47";
        let cursor = Cursor::new(input);
        let result = parse_file(cursor);
        assert!(result.is_err());
        let err = format!("{}", result.unwrap_err());
        assert!(err.contains("Extra data in part1 line"));
    }
}
