use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/16.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    let input = parse_input(&raw_input);

    println!("day 16 first: {}", solve_first(&input));
    println!("day 16 second: {}", solve_second(&input));

    Ok(())
}

enum Operation {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn dance(op: &Operation, input: &mut [char]) {
    match *op {
        Operation::Spin(i) => input.rotate_right(i),
        Operation::Exchange(i, j) => input.swap(i, j),
        Operation::Partner(a, b) => {
            let ia = input.iter().position(|ch| *ch == a).unwrap();
            let ib = input.iter().position(|ch| *ch == b).unwrap();
            input.swap(ia, ib);
        }
    }
}

fn parse_input(input: &str) -> Vec<Operation> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|op| {
            let (cmd, data) = op.split_at(1);
            match cmd {
                "s" => Operation::Spin(data.parse::<usize>().unwrap()),
                "x" => {
                    let nums: Vec<usize> = data
                        .split('/')
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect();
                    Operation::Exchange(nums[0], nums[1])
                }
                "p" => {
                    let partners: Vec<char> = data
                        .split('/')
                        .map(|p| p.parse::<char>().unwrap())
                        .collect();
                    Operation::Partner(partners[0], partners[1])
                }
                _ => unreachable!(),
            }
        }).collect()
}

fn solve_once(operations: &[Operation], programs: &mut [char]) -> String {
    for op in operations {
        dance(op, programs);
    }
    programs.iter().collect()
}

fn solve_first(operations: &[Operation]) -> String {
    let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
    solve_once(operations, &mut programs)
}

fn solve_second(operations: &[Operation]) -> String {
    let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
    let mut records = vec![programs.iter().collect()];
    records.push(solve_once(operations, &mut programs));
    loop {
        let cur = solve_once(operations, &mut programs);
        if cur == records[0] {
            break;
        } else {
            records.push(cur);
        }
    }
    records[1_000_000_000 % records.len()].clone()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_first, solve_second};

    #[test]
    fn test_solve_first() {
        let operations = parse_input("s2,x3/4,pe/b");
        assert_eq!(solve_first(&operations), "opacedbfghijklmn");
    }

    #[test]
    fn second() {
        let operations = parse_input("s2,x3/4,pe/b");
        assert_eq!(solve_second(&operations), "abcdefghijklmnop");
    }
}
