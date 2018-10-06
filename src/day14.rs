use super::day10::knot_hash;
use std::collections::HashSet;

pub fn solve() {
    println!("day 14 first: {}", solve_first("xlqgujun"));
    println!("day 14 second: {}", solve_second("xlqgujun"));
}

fn solve_first(input: &str) -> u32 {
    (0..128)
        .map(|i| knot_hash(format!("{}-{}", input, i).into_bytes(), 256, 64).count_ones())
        .sum()
}

fn solve_second(input: &str) -> u32 {
    let disk: Vec<u128> = (0..128)
        .map(|i| knot_hash(format!("{}-{}", input, i).into_bytes(), 256, 64))
        .collect();
    let mut hash: HashSet<(u8, u8)> = HashSet::new();
    let mut count = 0;

    for i in 0..128 {
        for j in 0..128 {
            if dfs(&disk, i, j, &mut hash) {
                count += 1;
            }
        }
    }

    count
}

fn dfs(disk: &[u128], i: u8, j: u8, hash: &mut HashSet<(u8, u8)>) -> bool {
    if !get_bit_at(disk[i as usize], j) || hash.contains(&(i, j)) {
        return false;
    }
    hash.insert((i, j));
    if i > 0 {
        dfs(disk, i - 1, j, hash);
    }
    if i < 127 {
        dfs(disk, i + 1, j, hash);
    }
    if j > 0 {
        dfs(disk, i, j - 1, hash);
    }
    if j < 127 {
        dfs(disk, i, j + 1, hash);
    }
    true
}

#[inline]
fn get_bit_at(input: u128, n: u8) -> bool {
    input & (1 << (127 - n)) != 0
}

#[cfg(test)]
mod tests {
    use super::{solve_first, solve_second};

    #[test]
    fn test_solve_first() {
        assert_eq!(solve_first("xlqgujun"), 8204);
    }

    #[test]
    fn test_solve_second() {
        assert_eq!(solve_second("xlqgujun"), 1089);
    }
}
