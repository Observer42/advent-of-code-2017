use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/22.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    let input = parse_input(&raw_input);
    println!("day 22 first: {}", solve_first(&input));
    println!("day 22 second: {}", solve_second(&input));

    Ok(())
}

fn parse_input(input: &str) -> HashMap<(i32, i32), Status> {
    let mut res = HashMap::new();
    let lines: Vec<_> = input.lines().collect();
    let size = lines.len() as i32;
    let offset = size / 2;

    for (line, i) in lines.iter().zip(0..size) {
        for (ch, j) in line.as_bytes().iter().zip(0..size) {
            if *ch == b'#' {
                res.insert((j - offset, i - offset), Status::Infected);
            }
        }
    }

    res
}

#[derive(Copy, Clone)]
enum Status {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(self) -> Self {
        use self::Direction::*;
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    fn right(self) -> Self {
        use self::Direction::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

struct Cluster {
    computers: HashMap<(i32, i32), Status>,
    x: i32,
    y: i32,
    direction: Direction,
}

impl Cluster {
    fn new(computers: HashMap<(i32, i32), Status>) -> Self {
        Cluster {
            computers,
            x: 0,
            y: 0,
            direction: Direction::Up,
        }
    }

    fn run_rule_1(&mut self, steps: i32) -> i32 {
        let mut count = 0;
        use self::Direction::*;
        for _ in 0..steps {
            if self.computers.contains_key(&(self.x, self.y)) {
                self.computers.remove(&(self.x, self.y));
                self.direction = self.direction.right();
            } else {
                self.computers.insert((self.x, self.y), Status::Infected);
                self.direction = self.direction.left();
                count += 1;
            }
            match self.direction {
                Up => self.y -= 1,
                Down => self.y += 1,
                Left => self.x -= 1,
                Right => self.x += 1,
            }
        }
        count
    }

    fn run_rule_2(&mut self, steps: i32) -> i32 {
        let mut count = 0;
        use self::Direction::*;
        use self::Status::*;
        for _ in 0..steps {
            let status = self.computers.entry((self.x, self.y)).or_insert(Clean);

            match *status {
                Clean => {
                    self.direction = self.direction.left();
                    *status = Weakened;
                }
                Weakened => {
                    *status = Infected;
                    count += 1;
                }
                Infected => {
                    self.direction = self.direction.right();
                    *status = Flagged;
                }
                Flagged => {
                    self.direction = self.direction.right().right();
                    *status = Clean;
                }
            }
            match self.direction {
                Up => self.y -= 1,
                Down => self.y += 1,
                Left => self.x -= 1,
                Right => self.x += 1,
            }
        }
        count
    }
}

fn solve_first(computers: &HashMap<(i32, i32), Status>) -> i32 {
    let mut cluster = Cluster::new(computers.clone());

    cluster.run_rule_1(10_000)
}

fn solve_second(computers: &HashMap<(i32, i32), Status>) -> i32 {
    let mut cluster = Cluster::new(computers.clone());

    cluster.run_rule_2(10_000_000)
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_first, solve_second};

    const INPUT: &str = "..#\n#..\n...";

    #[test]
    fn test_solve_first() {
        let computers = parse_input(INPUT);
        assert_eq!(solve_first(&computers), 5587);
    }

    #[test]
    fn test_solve_second() {
        let computers = parse_input(INPUT);
        assert_eq!(solve_second(&computers), 2511944);
    }

}
