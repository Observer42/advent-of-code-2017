use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/18.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    let instructions = parse_input(&raw_input);

    println!("day 18 first: {}", solve_first(&instructions));
    println!("day 18 second: {}", solve_second(&instructions));

    Ok(())
}

fn parse_input(raw: &str) -> Vec<Instruction> {
    raw.lines()
        .map(|line| Instruction::parse_from(line.trim()))
        .collect()
}

struct Process {
    instructions: Vec<Instruction>,
    current: isize,
    regs: HashMap<char, i64>,
}

fn solve_first(instructions: &[Instruction]) -> i64 {
    let mut last_sound = None;
    let mut process = Process {
        instructions: instructions.to_owned(),
        current: 0,
        regs: HashMap::new(),
    };
    loop {
        if process.current < process.instructions.len() as isize {
            match process.instructions[process.current as usize] {
                Instruction::Snd(arg) => {
                    let snd = match arg {
                        Arg::Val(val) => val,
                        Arg::Reg(reg) => *process.regs.entry(reg).or_insert(0),
                    };
                    last_sound = Some(snd);
                }
                Instruction::Set(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) = arg.value(&process)
                }
                Instruction::Add(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) += arg.value(&process)
                }
                Instruction::Mul(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) *= arg.value(&process)
                }
                Instruction::Mod(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) %= arg.value(&process)
                }
                Instruction::Rcv(reg) => {
                    if *process.regs.entry(reg).or_insert(0) != 0 {
                        return last_sound.unwrap();
                    }
                }
                Instruction::Jgz(arg0, arg1) => {
                    let condition = match arg0 {
                        Arg::Val(val) => val,
                        Arg::Reg(reg) => *process.regs.entry(reg).or_insert(0),
                    };
                    if condition > 0 {
                        process.current += arg1.value(&process) as isize;
                        continue;
                    }
                }
            };

            process.current += 1;
        } else {
            return last_sound.unwrap();
        }
    }
}

fn solve_second(instructions: &[Instruction]) -> i32 {
    let process0 = Process {
        instructions: instructions.to_owned(),
        current: 0,
        regs: HashMap::new(),
    };

    let process1 = Process {
        instructions: instructions.to_owned(),
        current: 0,
        regs: HashMap::new(),
    };
    let (tx0, rx1) = channel();
    let (tx1, rx0) = channel();

    let handle0 = thread::spawn(move || execute(process0, 0, &tx0, &rx0));

    let handle1 = thread::spawn(move || execute(process1, 1, &tx1, &rx1));

    let _res0 = handle0.join().unwrap();

    handle1.join().unwrap()
}

fn execute(mut process: Process, id: i64, sender: &Sender<i64>, receiver: &Receiver<i64>) -> i32 {
    process.regs.insert('p', id);
    let mut sent = 0;
    loop {
        if process.current < process.instructions.len() as isize {
            match process.instructions[process.current as usize] {
                Instruction::Snd(arg) => {
                    let snd = match arg {
                        Arg::Val(val) => val,
                        Arg::Reg(reg) => *process.regs.entry(reg).or_insert(0),
                    };
                    sender.send(snd).unwrap();
                    sent += 1;
                }
                Instruction::Set(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) = arg.value(&process)
                }
                Instruction::Add(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) += arg.value(&process)
                }
                Instruction::Mul(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) *= arg.value(&process)
                }
                Instruction::Mod(reg, arg) => {
                    *process.regs.entry(reg).or_insert(0) %= arg.value(&process)
                }
                Instruction::Rcv(reg) => {
                    if let Ok(val) = receiver.recv_timeout(Duration::from_secs(1)) {
                        *process.regs.entry(reg).or_insert(0) = val;
                    } else {
                        return sent;
                    }
                }
                Instruction::Jgz(arg0, arg1) => {
                    let condition = match arg0 {
                        Arg::Val(val) => val,
                        Arg::Reg(reg) => *process.regs.entry(reg).or_insert(0),
                    };
                    if condition > 0 {
                        process.current += arg1.value(&process) as isize;
                        continue;
                    }
                }
            };

            process.current += 1;
        } else {
            return sent;
        }
    }
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
    Snd(Arg),
    Set(char, Arg),
    Add(char, Arg),
    Mul(char, Arg),
    Mod(char, Arg),
    Rcv(char),
    Jgz(Arg, Arg),
}

impl Instruction {
    fn parse_from(input: &str) -> Instruction {
        let cmd = &input[0..3];
        let mut it = input[3..].split_whitespace();
        let arg0_str = it.next().unwrap();
        let reg = arg0_str.chars().nth(0).unwrap();
        let arg0 = Arg::parse_from(arg0_str);

        match cmd {
            cmd @ "snd" | cmd @ "rcv" => match cmd {
                "snd" => Instruction::Snd(arg0),
                "rcv" => Instruction::Rcv(reg),
                _ => unreachable!(),
            },
            cmd => {
                let arg1 = Arg::parse_from(it.next().unwrap());
                match cmd {
                    "set" => Instruction::Set(reg, arg1),
                    "add" => Instruction::Add(reg, arg1),
                    "mul" => Instruction::Mul(reg, arg1),
                    "mod" => Instruction::Mod(reg, arg1),
                    "jgz" => Instruction::Jgz(arg0, arg1),
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_first, solve_second};

    #[test]
    fn test_solve_first() {
        let raw = "set a 1
                   add a 2
                   mul a a
                   mod a 5
                   snd a
                   set a 0
                   rcv a
                   jgz a -1
                   set a 1
                   jgz a -2";

        let input = parse_input(raw);
        assert_eq!(solve_first(&input), 4)
    }

    #[test]
    fn test_solve_second() {
        let raw = "snd 1
                   snd 2
                   snd p
                   rcv a
                   rcv b
                   rcv c
                   rcv d";

        let input = parse_input(raw);
        assert_eq!(solve_second(&input), 3)
    }
}
