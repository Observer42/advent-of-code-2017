use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::mem::swap;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/12.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    let input = parse_input(&raw_input);

    let mut solver = Solver::default();

    let (first, second) = solver.solve_both(&input);
    println!("day 12 first: {}", first);
    println!("day 12 second: {}", second);

    Ok(())
}

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split(|c: char| !c.is_numeric())
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        }).collect()
}

#[derive(Default)]
struct Solver {
    root_count: HashMap<i32, i32>,
    parent: HashMap<i32, i32>,
}

impl Solver {
    fn solve_both(&mut self, input: &[Vec<i32>]) -> (i32, i32) {
        self.root_count.clear();
        self.parent.clear();

        for pid_list in input {
            for &pid in pid_list {
                if !self.parent.contains_key(&pid) {
                    self.root_count.insert(pid, 1);
                    self.parent.insert(pid, pid);
                }
            }
            let pid = pid_list[0];
            let connections = &pid_list[1..];

            for &conn in connections {
                self.quick_union(pid, conn);
            }
        }
        (self.root_count[&0], self.root_count.len() as i32)
    }

    fn quick_union(&mut self, id1: i32, id2: i32) {
        let mut r1 = self.find_root(id1);
        let mut r2 = self.find_root(id2);
        if r1 == r2 {
            return;
        }
        if r1 == 0 || (self.root_count[&r1] >= self.root_count[&r2] && r2 != 0) {
        } else {
            swap(&mut r1, &mut r2);
        }

        *self.root_count.get_mut(&r1).unwrap() += self.root_count[&r2];
        self.root_count.remove(&r2);
        *self.parent.get_mut(&r2).unwrap() = r1;
    }

    fn find_root(&mut self, mut id: i32) -> i32 {
        while self.parent[&id] != id {
            *self.parent.get_mut(&id).unwrap() = self.parent[&self.parent[&id]];
            id = self.parent[&id];
        }
        id
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_input, Solver};

    #[test]
    fn test_solver() {
        let input = "0 <-> 2
                     1 <-> 1
                     2 <-> 0, 3, 4
                     3 <-> 2, 4
                     4 <-> 2, 3, 6
                     5 <-> 6
                     6 <-> 4, 5";
        let mut solver = Solver::default();
        assert_eq!(solver.solve_both(&parse_input(input)), (6, 2));
    }
}
