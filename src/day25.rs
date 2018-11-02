use std::collections::HashSet;

pub fn solve() {
    println!("day 25 first: {}", solve_first());
}

fn solve_first() -> usize {
    let mut machine = Machine {
        state: State::A,
        pos: 0,
        tape: HashSet::new(),
    };
    for _ in 0..12_317_297 {
        machine.run();
    }

    machine.tape.len()
}

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct Machine {
    state: State,
    pos: i32,
    tape: HashSet<i32>,
}

impl Machine {
    fn run(&mut self) {
        use self::State::*;
        match self.state {
            A => {
                if self.tape.contains(&self.pos) {
                    self.tape.remove(&self.pos);
                    self.pos -= 1;
                    self.state = D;
                } else {
                    self.tape.insert(self.pos);
                    self.pos += 1;
                    self.state = B;
                }
            }
            B => {
                if self.tape.contains(&self.pos) {
                    self.tape.remove(&self.pos);
                    self.state = F;
                } else {
                    self.tape.insert(self.pos);
                    self.state = C;
                }
                self.pos += 1;
            }
            C => {
                if self.tape.contains(&self.pos) {
                    self.state = A;
                } else {
                    self.tape.insert(self.pos);
                    self.state = C;
                }
                self.pos -= 1;
            }
            D => {
                if self.tape.contains(&self.pos) {
                    self.pos += 1;
                    self.state = A;
                } else {
                    self.pos -= 1;
                    self.state = E;
                }
            }
            E => {
                if self.tape.contains(&self.pos) {
                    self.tape.remove(&self.pos);
                    self.pos += 1;
                    self.state = B;
                } else {
                    self.tape.insert(self.pos);
                    self.pos -= 1;
                    self.state = A;
                }
            }
            F => {
                if self.tape.contains(&self.pos) {
                    self.tape.remove(&self.pos);
                    self.state = E;
                } else {
                    self.state = C;
                }
                self.pos += 1;
            }
        }
    }
}
