use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/06.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let bank: Vec<i32> = input
        .split_whitespace()
        .filter_map(|digits| digits.parse::<i32>().ok())
        .collect();
    println!("day 6 first: {}", solve_both(bank.clone()).0);
    println!("day 6 second: {}", solve_both(bank.clone()).1);

    Ok(())
}

fn find_max(bank: &[i32]) -> (usize, i32) {
    let mut pos = 0;
    let mut val = 0;
    for (i, v) in bank.iter().enumerate() {
        if val < *v {
            pos = i;
            val = *v;
        }
    }
    (pos, val)
}

fn solve_both(mut bank: Vec<i32>) -> (u32, u32) {
    let mut hash = HashMap::new();
    let mut step = 0;
    hash.insert(bank.clone(), step);

    loop {
        step += 1;
        let (pos, val) = find_max(&bank);
        bank[pos] = 0;
        let len = bank.len();
        let avg = val / (len as i32);
        if avg > 0 {
            bank.iter_mut().for_each(|elem| *elem += avg);
        }
        for i in (pos + 1)..(pos + 1 + (val as usize % len)) {
            bank[i % len] += 1;
        }
        if hash.contains_key(&bank) {
            break;
        } else {
            hash.insert(bank.clone(), step);
        }
    }
    (step, step - hash[&bank])
}

#[cfg(test)]
mod tests {
    use super::solve_both;

    #[test]
    fn test_solve_both() {
        assert_eq!(solve_both(vec![0, 2, 7, 0]), (5, 4));
    }
}
