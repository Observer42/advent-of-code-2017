use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/23.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    let instructions = parse_input(&raw_input);

    println!("day 23 first: {}", solve_first(&instructions));

    Ok(())
}

fn parse_input(raw: &str) -> Vec<Instruction> {
    raw.lines()
        .map(|line| Instruction::parse_from(line.trim()))
        .collect()
}

fn solve_first(instructions: &[Instruction]) -> i64 {
    let mut process = Process {
        instructions: instructions.to_owned(),
        current: 0,
        regs: HashMap::new(),
    };
    let mut count = 0;

    loop {
        if process.current < process.instructions.len() as isize {
            match process.instructions[process.current as usize] {
                Instruction::Set(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) = arg.value(&process)
                }
                Instruction::Mul(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) *= arg.value(&process);
                    count += 1;
                }
                Instruction::Sub(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) -= arg.value(&process)
                }
                Instruction::Jnz(arg0, arg1) => {
                    let condition = match arg0 {
                        Arg::Val(val) => val,
                        Arg::Reg(reg) => *process.regs.entry(reg).or_insert(0),
                    };
                    if condition != 0 {
                        process.current += arg1.value(&process) as isize;
                        continue;
                    }
                }
            };
            process.current += 1;
        } else {
            break;
        }
    }

    count
}

struct Process {
    instructions: Vec<Instruction>,
    current: isize,
    regs: HashMap<char, i64>,
}

#[derive(Clone, Copy, Debug)]
enum Arg {
    Reg(char),
    Val(i64),
}

impl Arg {
    fn parse_from(input: &str) -> Arg {
        match input.chars().next().unwrap() {
            ch if ch.is_numeric() || ch == '-' => Arg::Val(input.parse::<i64>().unwrap()),
            ch => Arg::Reg(ch),
        }
    }

    fn value(&self, process: &Process) -> i64 {
        match self {
            Arg::Reg(ch) => process.regs[&ch],
            Arg::Val(val) => *val,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Set(char, Arg),
    Sub(char, Arg),
    Mul(char, Arg),
    Jnz(Arg, Arg),
}

impl Instruction {
    fn parse_from(input: &str) -> Instruction {
        let cmd = &input[0..3];
        let mut it = input[3..].split_whitespace();
        let arg0_str = it.next().unwrap();
        let reg = arg0_str.chars().nth(0).unwrap();
        let arg0 = Arg::parse_from(arg0_str);
        let arg1 = Arg::parse_from(it.next().unwrap());

        match cmd {
            "set" => Instruction::Set(reg, arg1),
            "sub" => Instruction::Sub(reg, arg1),
            "mul" => Instruction::Mul(reg, arg1),
            "jnz" => Instruction::Jnz(arg0, arg1),
            _ => unreachable!(),
        }
    }
}
