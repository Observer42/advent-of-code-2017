use std::cmp;
use std::collections::HashMap;

pub fn solve() {
    println!("day 3 first: {}", solve_distance_first(277_678));
    println!("day 3 second: {}", solve_second(277_678));
}

fn solve_distance_first(dest: u32) -> u32 {
    if dest == 1 {
        return 0;
    }

    let sqrt = f64::from(dest).sqrt() as u32;
    if sqrt % 2 != 0 && dest == sqrt * sqrt {
        return sqrt - 1;
    }

    let length = if sqrt % 2 == 0 { sqrt + 1 } else { sqrt + 2 };

    let square = length * length;
    let tmp = (square + length / 2 - dest) % (length - 1);
    length / 2 + cmp::min(tmp, length - 1 - tmp)
}

fn sum_neighbors(hash: &HashMap<(i32, i32), u32>, x: i32, y: i32) -> u32 {
    let mut sum = 0;
    for i in (-1)..2 {
        for j in (-1)..2 {
            sum += hash.get(&(x + i, y + j)).unwrap_or(&0);
        }
    }
    sum
}

fn solve_second(input: u32) -> u32 {
    let mut hash = HashMap::new();
    hash.insert((0, 0), 1u32);
    let mut rad = 1;

    loop {
        for i in 0..(2 * rad) {
            let sum = sum_neighbors(&hash, rad, 1 - rad + i);
            if sum > input {
                return sum;
            }
            hash.insert((rad, 1 - rad + i), sum);
        }
        for i in 0..(2 * rad) {
            let sum = sum_neighbors(&hash, rad - 1 - i, rad);
            if sum > input {
                return sum;
            }
            hash.insert((rad - 1 - i, rad), sum);
        }
        for i in 0..(2 * rad) {
            let sum = sum_neighbors(&hash, 0 - rad, rad - 1 - i);
            if sum > input {
                return sum;
            }
            hash.insert((0 - rad, rad - 1 - i), sum);
        }
        for i in 0..(2 * rad) {
            let sum = sum_neighbors(&hash, i + 1 - rad, 0 - rad);
            if sum > input {
                return sum;
            }
            hash.insert((i + 1 - rad, 0 - rad), sum);
        }

        rad += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_distance_first, solve_second};

    #[test]
    fn test_solve_distance_first() {
        assert_eq!(solve_distance_first(1), 0);
        assert_eq!(solve_distance_first(12), 3);
        assert_eq!(solve_distance_first(23), 2);
        assert_eq!(solve_distance_first(1024), 31);
    }

    #[test]
    fn test_solve_second() {
        assert_eq!(solve_second(1), 2);
        assert_eq!(solve_second(4), 5);
        assert_eq!(solve_second(100), 122);
        assert_eq!(solve_second(400), 747);
    }
}
