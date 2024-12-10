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

    let constraints = parts.next()
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

    let sequences = parts.next()
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

    Ok((constraints, sequences))
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

fn find_max_in_all_sequences(sequences: &Vec<Vec<usize>>) -> usize {
    *(sequences.iter().map(|seq| seq.iter().max().unwrap_or(&0)).max().unwrap_or(&0))
}

/// Pass by reference an array which can hold as many elements as the largest element in the ordering
fn initialize_positional_array(ordering: &Vec<usize>, positional_array: &mut Vec<usize>) {
    positional_array.fill(usize::MAX);
    for (i, n) in ordering.iter().enumerate() {
        positional_array[*n] = i;
    }
}

fn verify_ordering(adj_list: &Vec<Vec<usize>>, ordering: &Vec<usize>, positional_array: &mut Vec<usize>) -> bool {
    for n in ordering.iter() {
        let n: usize = *n;
        // Check each value against its constraints. All values in the adjacency list for this element must have a
        // position further to the right in the ordering.
        for &m in &adj_list[n as usize] {
            // If any values are -1, they are not in the ordering and we can skip them
            if positional_array[m] < positional_array[n] {
                // If the position of the adjacent element is less than the current element, the ordering is invalid
                return false;
            }
        }
    }
    true
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut result: usize = 0;

    let (constraints, sequences) = parse_file(reader)?;

    // find the largest value among the sequences
    let max_value: usize = find_max_in_all_sequences(&sequences);
    let mut positional_array: Vec<usize> = vec![usize::MAX; max_value + 1];

    let adj_list = parse_adj_list(constraints)?;

    for seq in sequences {
        initialize_positional_array(&seq, &mut positional_array);
        if verify_ordering(&adj_list, &seq, &mut positional_array) {
            result += seq[seq.len()/2] as usize;
        }
    }
    Ok(result)
}

fn swap_elements(ordering: &mut Vec<usize>, pos1: usize, pos2: usize) {
    let temp = ordering[pos1];
    ordering[pos1] = ordering[pos2];
    ordering[pos2] = temp;
}

/// Ensure the sequence is invalid and reorder it to be valid
fn verify_invalid_and_reorder(adj_list: &Vec<Vec<usize>>, ordering: &mut Vec<usize>, positional_array: &mut Vec<usize>) -> bool {
    let mut is_invalid = false;

    // By iterating through in reverse order, we will encounter out-of-order elements sooner
    for i in (0..ordering.len()).rev() {
        let n = ordering[i];
        // Check each value against its constraints. All values in the adjacency list for this element must have a
        // position further to the right in the ordering.
        for &m in &adj_list[n] {
            // If any values are -1, they are not in the ordering and we can skip them
            if positional_array[m] < positional_array[n] {
                // If the position of the adjacent element is less than the current element, the ordering is invalid
                is_invalid = true;
                // Swap the elements
                swap_elements(ordering, positional_array[m], positional_array[n]);
                // Update the positions in `positional_array` after swapping
                let pos_m = positional_array[m];
                let pos_n = positional_array[n];
                positional_array[ordering[pos_m]] = pos_m;
                positional_array[ordering[pos_n]] = pos_n;
            }
        }
    }
    is_invalid
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut result: usize = 0;

    let (constraints, sequences) = parse_file(reader)?;

    // find the largest value among the sequences
    let max_value: usize = find_max_in_all_sequences(&sequences);
    let mut positional_array: Vec<usize> = vec![usize::MAX; max_value + 1];

    let adj_list = parse_adj_list(constraints)?;

    for mut seq in sequences {
        initialize_positional_array(&seq, &mut positional_array);
        if verify_invalid_and_reorder(&adj_list, &mut seq, &mut positional_array) {
            result += seq[seq.len()/2] as usize;
        }
    }
    Ok(result)
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
    println!("\n=== Part 2 ===");

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

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
