use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("src/input/05.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let jumps: Vec<i32> = input
        .lines()
        .filter_map(|digits| digits.parse::<i32>().ok())
        .collect();
    println!("day 5 first: {}", solve_first(&mut jumps.clone()));
    println!("day 5 second: {}", solve_second(&mut jumps.clone()));

    Ok(())
}

fn solve_first(jumps: &mut Vec<i32>) -> u32 {
    solve_general(jumps, &|_| 1)
}

fn solve_second(jumps: &mut Vec<i32>) -> u32 {
    solve_general(jumps, &|i| if i >= 3 { -1 } else { 1 })
}

fn solve_general(jumps: &mut Vec<i32>, func: &Fn(i32) -> i32) -> u32 {
    let mut res = 0;
    let len = jumps.len() as i32;
    let mut cur: i32 = 0;
    while cur >= 0 && cur < len {
        let next = cur + jumps[cur as usize];
        jumps[cur as usize] += func(jumps[cur as usize]);
        cur = next;
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{solve_first, solve_second};

    #[test]
    fn test_solve_first() {
        let mut input = vec![0, 3, 0, 1, -3];
        assert_eq!(solve_first(&mut input), 5);
    }

    #[test]
    fn test_solve_second() {
        let mut input = vec![0, 3, 0, 1, -3];
        assert_eq!(solve_second(&mut input), 10);
    }
}
