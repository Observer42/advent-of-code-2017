use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/19.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    let diagram = parse_input(&raw_input);

    let (first, second) = solve_both(&diagram);
    println!("day 19 first: {}", first);
    println!("day 19 second: {}", second);

    Ok(())
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
        self.left().left().left()
    }
}

struct Diagram {
    maze: Vec<Vec<char>>,
    height: isize,
    width: isize,
}

impl Diagram {
    fn step(&self, direction: Direction, row: usize, col: usize) -> Option<(usize, usize)> {
        use self::Direction::*;
        let (r, c) = match direction {
            Up => (row as isize - 1, col as isize),
            Down => (row as isize + 1, col as isize),
            Left => (row as isize, col as isize - 1),
            Right => (row as isize, col as isize + 1),
        };
        if r >= 0
            && r < self.height
            && c >= 0
            && c < self.width
            && !self.maze[r as usize][c as usize].is_whitespace()
        {
            Some((r as usize, c as usize))
        } else {
            None
        }
    }

    fn next_dir(&self, direction: Direction, row: usize, col: usize) -> Option<Direction> {
        match self.maze[row][col] {
            ' ' => None,
            '+' => {
                if self.step(direction.left(), row, col).is_some() {
                    Some(direction.left())
                } else if self.step(direction.right(), row, col).is_some() {
                    Some(direction.right())
                } else {
                    None
                }
            }
            _ => Some(direction),
        }
    }
}

fn parse_input(input: &str) -> Diagram {
    let maze: Vec<Vec<char>> = input
        .lines()
        .filter_map(|line| {
            if !line.is_empty() {
                Some(line.chars().collect())
            } else {
                None
            }
        }).collect();

    let height = maze.len() as isize;
    let width = maze[0].len() as isize;

    Diagram {
        maze,
        height,
        width,
    }
}

fn solve_both(diagram: &Diagram) -> (String, i32) {
    let mut res = String::new();
    let mut count = 0;

    let (mut cur_r, mut cur_c) = (0, diagram.maze[0].iter().position(|&ch| ch != ' ').unwrap());
    let mut direction = Direction::Down;

    loop {
        count += 1;
        if let Some((r, c)) = diagram.step(direction, cur_r, cur_c) {
            cur_r = r;
            cur_c = c;
            let ch = diagram.maze[cur_r][cur_c];
            if ch.is_ascii_uppercase() {
                res.push(ch)
            }
            if let Some(d) = diagram.next_dir(direction, cur_r, cur_c) {
                direction = d;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    (res, count)
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_both};

    #[test]
    fn test_solve_both() {
        let input = "     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ \n";
        let diagram = parse_input(input);
        assert_eq!(solve_both(&diagram), ("ABCDEF".to_string(), 38));
    }
}
