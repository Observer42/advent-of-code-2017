use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/13.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    let input = parse_input(&raw_input);

    println!("day 13 first: {}", solve_first(&input));
    println!("day 13 second: {}", solve_second(&input));

    Ok(())
}

fn parse_input(input: &str) -> HashMap<u32, u32> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line
                .split(": ")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            (split[0], split[1])
        }).filter(|&(_, range)| range != 0)
        .collect()
}

fn solve_first(input: &HashMap<u32, u32>) -> u32 {
    input
        .iter()
        .filter_map(|(&depth, &range)| {
            if depth % (2 * (range - 1)) == 0 {
                Some(depth * range)
            } else {
                None
            }
        }).sum()
}

fn solve_second(input: &HashMap<u32, u32>) -> u32 {
    (0..)
        .find(|wait| {
            !input
                .iter()
                .any(|(&depth, &range)| (wait + depth) % (2 * (range - 1)) == 0)
        }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{solve_first, solve_second};
    use std::collections::HashMap;

    #[test]
    fn test_solve_first() {
        let input: HashMap<u32, u32> = [(0, 3), (1, 2), (4, 4), (6, 4)].iter().cloned().collect();
        assert_eq!(solve_first(&input), 24);
    }

    #[test]
    fn test_solve_second() {
        let input: HashMap<u32, u32> = [(0, 3), (1, 2), (4, 4), (6, 4)].iter().cloned().collect();
        assert_eq!(solve_second(&input), 10);
    }
}
