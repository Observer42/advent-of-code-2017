struct Generator {
    current: u64,
    factor: u64,
    criteria: u64,
}

impl Generator {
    fn new(current: u64, factor: u64, criteria: u64) -> Generator {
        Generator {
            current,
            factor,
            criteria,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        loop {
            self.current = self.current * self.factor % 2_147_483_647;
            if self.current % self.criteria == 0 {
                return Some(self.current);
            }
        }
    }
}

pub fn solve() {
    println!("day 15 first: {}", solve_first());
    println!("day 15 second: {}", solve_second());
}

fn solve_first() -> usize {
    let gen_a = Generator::new(634, 16807, 1);
    let gen_b = Generator::new(301, 48271, 1);

    gen_a
        .zip(gen_b)
        .take(40_000_000)
        .filter(|&(a, b)| (a & 65535) == (b & 65535))
        .count()
}

fn solve_second() -> usize {
    let gen_a = Generator::new(634, 16807, 4);
    let gen_b = Generator::new(301, 48271, 8);

    gen_a
        .zip(gen_b)
        .take(5_000_000)
        .filter(|&(a, b)| (a & 65535) == (b & 65535))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{solve_first, solve_second};

    #[test]
    fn test_solve_first() {
        assert_eq!(solve_first(), 573);
    }

    #[test]
    fn test_solve_second() {
        assert_eq!(solve_second(), 294);
    }
}
