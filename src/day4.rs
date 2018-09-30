use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("src/input/04.txt")?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    let matrix = generate_matrix(&lines);
    println!("day 4 first: {}", solve_first(&matrix));
    println!("day 4 second: {}", solve_second(&matrix));

    Ok(())
}

fn generate_matrix(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect()
}

fn solve_first(lines: &[Vec<&str>]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let mut set = HashSet::new();
            for word in line {
                if !set.insert(word) {
                    return 0;
                }
            }
            1
        }).sum()
}

fn generate_sorted_string(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars.iter().collect()
}

fn solve_second(lines: &[Vec<&str>]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let mut set = HashSet::new();
            for word in line {
                let sorted = generate_sorted_string(word);
                if set.contains(&sorted) {
                    return 0;
                }
                set.insert(sorted);
            }
            1
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::{generate_matrix, solve_first, solve_second};

    #[test]
    fn test_solve_first() {
        let input = "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa";
        let matrix = generate_matrix(&input);
        assert_eq!(solve_first(&matrix), 2);
    }

    #[test]
    fn test_solve_second() {
        let input = "abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio";
        let matrix = generate_matrix(&input);
        assert_eq!(solve_second(&matrix), 3);
    }
}
