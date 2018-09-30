use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("src/input/02.txt")?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;

    let matrix = generate_matrix(&lines);
    println!("day 2 first: {}", solve_checksum_first(&matrix));
    println!("day 2 second: {}", solve_checksum_second(&matrix));
    Ok(())
}

fn generate_matrix(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|digits| digits.parse::<u32>().ok())
                .collect()
        }).collect()
}

fn solve_checksum_first(matrix: &[Vec<u32>]) -> u32 {
    matrix
        .iter()
        .map(|line| line.iter().max().unwrap() - line.iter().min().unwrap())
        .sum()
}

fn solve_checksum_second(matrix: &[Vec<u32>]) -> u32 {
    matrix
        .iter()
        .filter_map(|line| {
            for i in 0..(line.len() - 1) {
                for j in (i + 1)..line.len() {
                    if line[i] % line[j] == 0 {
                        return Some(line[i] / line[j]);
                    } else if line[j] % line[i] == 0 {
                        return Some(line[j] / line[i]);
                    }
                }
            }
            None
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::{generate_matrix, solve_checksum_first, solve_checksum_second};

    #[test]
    fn test_solve_checksum_first() {
        let input = "5 1 9 5\n7 5 3\n2 4 6 8";
        let matrix = generate_matrix(input);
        assert_eq!(solve_checksum_first(&matrix), 18);
    }

    #[test]
    fn test_solve_checksum_second() {
        let input = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        let matrix = generate_matrix(input);
        assert_eq!(solve_checksum_second(&matrix), 9);
    }
}
