pub fn solve() {
    println!("day 17 first: {}", solve_first(377, 2017));
    println!("day 17 second: {}", solve_second(377, 50_000_000));
}

fn solve_first(step: usize, insertions: u32) -> u32 {
    let mut buffer = vec![0];
    let mut pos = 0;

    for i in 1..=insertions {
        pos = (pos + step) % buffer.len() + 1;
        buffer.insert(pos, i);
    }

    buffer[pos + 1]
}

fn solve_second(step: usize, insertions: u32) -> u32 {
    let mut len = 1;
    let mut pos = 0;
    let mut res = 0;

    for i in 1..=insertions {
        pos = (pos + step) % len + 1;
        len += 1;
        if pos == 1 {
            res = i;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::{solve_first, solve_second};

    #[test]
    fn test_solve_first() {
        assert_eq!(solve_first(3, 9), 5);
    }

    #[test]
    fn test_solve_second() {
        assert_eq!(solve_second(3, 9), 9);
    }
}
