use std::cmp;
use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/11.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let (first, second) = solve_both(input.lines().next().unwrap());
    println!("day 11 first: {}", first);
    println!("day 11 second: {}", second);

    Ok(())
}

struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    fn distance(&self) -> i32 {
        (self.x.abs() + self.y.abs() + (self.x + self.y).abs()) / 2
    }
}

fn solve_both(input: &str) -> (i32, i32) {
    let (pos, max) = input.split(',').filter(|&dir| !dir.is_empty()).fold(
        (Pos::new(0, 0), 0),
        |(pos, max), direction| {
            let new_pos = match direction {
                "n" => Pos::new(pos.x, pos.y + 1),
                "s" => Pos::new(pos.x, pos.y - 1),
                "ne" => Pos::new(pos.x + 1, pos.y),
                "sw" => Pos::new(pos.x - 1, pos.y),
                "se" => Pos::new(pos.x + 1, pos.y - 1),
                "nw" => Pos::new(pos.x - 1, pos.y + 1),
                _ => unreachable!(),
            };
            let max = cmp::max(max, new_pos.distance());
            (new_pos, max)
        },
    );
    (pos.distance(), max)
}

#[cfg(test)]
mod tests {
    use super::solve_both;

    #[test]
    fn test_solve_both() {
        assert_eq!(solve_both("ne,ne,ne"), (3, 3));
        assert_eq!(solve_both("ne,ne,sw,sw"), (0, 2));
        assert_eq!(solve_both("ne,ne,s,s"), (2, 2));
        assert_eq!(solve_both("se,sw,se,sw,sw"), (3, 3));
    }
}
