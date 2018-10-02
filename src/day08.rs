use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/08.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let instructions = parse_input(&input);

    let (first, second) = solve_both(&instructions);
    println!("day 8 first: {}", first);
    println!("day 8 second: {}", second);
    Ok(())
}

enum Operation {
    Inc(i32),
    Dec(i32),
}

enum Comparator {
    LT,
    LE,
    GT,
    GE,
    NE,
    EQ,
}

struct Condition {
    reg: String,
    cmp: Comparator,
    val: i32,
}

struct Instruction {
    reg: String,
    op: Operation,
    cond: Condition,
}

impl Instruction {
    fn parse(input: &str) -> Option<Self> {
        let words: Vec<_> = input.split_whitespace().collect();
        if words.len() != 7 {
            return None;
        }
        let cond_reg = words[4].to_string();
        let cmp = match words[5] {
            "<" => Comparator::LT,
            ">" => Comparator::GT,
            "<=" => Comparator::LE,
            ">=" => Comparator::GE,
            "!=" => Comparator::NE,
            "==" => Comparator::EQ,
            _ => return None,
        };
        let val = words[6].parse::<i32>().ok()?;
        let reg = words[0].to_string();
        let op = match words[1] {
            "inc" => Operation::Inc(words[2].parse().ok()?),
            "dec" => Operation::Dec(words[2].parse().ok()?),
            _ => return None,
        };
        Some(Instruction {
            reg,
            op,
            cond: Condition {
                reg: cond_reg,
                cmp,
                val,
            },
        })
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|line| Instruction::parse(line))
        .collect()
}

fn solve_both(instructions: &[Instruction]) -> (i32, i32) {
    let mut hash = HashMap::new();
    let mut record_max = 0;
    for instruction in instructions {
        let cond_reg = *(hash.entry(&instruction.cond.reg).or_insert(0));
        let cond_val = instruction.cond.val;
        let condition = match instruction.cond.cmp {
            Comparator::LT => cond_reg < cond_val,
            Comparator::GT => cond_reg > cond_val,
            Comparator::LE => cond_reg <= cond_val,
            Comparator::GE => cond_reg >= cond_val,
            Comparator::NE => cond_reg != cond_val,
            Comparator::EQ => cond_reg == cond_val,
        };
        if condition {
            let reg = hash.entry(&instruction.reg).or_insert(0);
            match instruction.op {
                Operation::Inc(ref val) => *reg += val,
                Operation::Dec(ref val) => *reg -= val,
            };
            record_max = cmp::max(record_max, *reg);
        }
    }
    (*hash.values().max().unwrap(), record_max)
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_both};

    #[test]
    fn test_solve_both() {
        let input = "b inc 5 if a > 1
                     a inc 1 if b < 5
                     c dec -10 if a >= 1
                     c inc -20 if c == 10";
        let instructions = parse_input(input);
        assert_eq!(solve_both(&instructions), (1, 10));
    }
}
