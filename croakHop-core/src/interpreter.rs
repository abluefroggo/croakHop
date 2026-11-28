use crate::*;

impl Interpreter {
    pub fn new(p: Pond) -> Self {
        let nf: Frog = Frog {
            x: p.sx as isize,
            y: p.sy as isize,
            dx: 1,
            dy: 0,
            lp_stack: vec![],
        };
        let mut interpreter = Self {
            p: p,
            f: nf,
            out: vec![],
            steps: 0,
            state: crate::ExecutionState::Running,
            history: vec![],
        };
        interpreter.save_snapshot(Instruction::Empty);
        interpreter
    }

    pub fn save_snapshot(&mut self, ins: Instruction) {
        let snapshot = ExecutionSnapshot {
            step: self.steps,
            x: self.f.x,
            y: self.f.y,
            dx: self.f.dx,
            dy: self.f.dy,
            stack: self.f.lp_stack.clone(),
            output: self.out.clone(),
            instruction: ins,
        };
        self.history.push(snapshot);
    }
    pub fn step(&mut self) -> bool {
        hop(&mut self.f, &self.p);
        let ctile: char = self.p.p_grid[to_idx(&self.f, &self.p)];
        let ins: Instruction = Instruction::from_char(ctile);
        match ins {
            Instruction::Empty => {}
            Instruction::Turn => {
                let old_dx = self.f.dx;
                self.f.dx = -self.f.dy;
                self.f.dy = old_dx;
            }
            Instruction::Up => {
                self.f.dx = 0;
                self.f.dy = -self.f.dy.abs();
                if self.f.dy == 0 {
                    self.f.dy = -1;
                }
            }
            Instruction::Down => {
                self.f.dx = 0;
                self.f.dy = self.f.dy.abs();
                if self.f.dy == 0 {
                    self.f.dy = 1;
                }
            }
            Instruction::Left => {
                self.f.dx = -self.f.dx.abs();
                self.f.dy = 0;
                if self.f.dx == 0 {
                    self.f.dx = -1
                }
            }
            Instruction::Right => {
                self.f.dx = self.f.dx.abs();
                self.f.dy = 0;
                if self.f.dx == 0 {
                    self.f.dx = 1;
                }
            }
            Instruction::Accelerate => {
                self.f.dx *= 2;
                self.f.dy *= 2;
            }
            Instruction::Duplicate => {
                let t = self.f.lp_stack.last().copied().unwrap_or(0);
                self.f.lp_stack.push(t);
            }
            Instruction::Push(n) => {
                self.f.lp_stack.push(n);
            }
            Instruction::Swap => {
                if self.f.lp_stack.len() > 1 {
                    let idxa = self.f.lp_stack.len() - 1;
                    let idxb = idxa - 1;
                    let temp = self.f.lp_stack[idxb];
                    self.f.lp_stack[idxb] = self.f.lp_stack[idxa];
                    self.f.lp_stack[idxa] = temp;
                }
            }
            Instruction::Pop => {
                self.f.lp_stack.pop();
            }
            Instruction::Add => {
                let a = self.f.lp_stack.pop().unwrap_or(0);
                let b = self.f.lp_stack.pop().unwrap_or(0);
                self.f.lp_stack.push(a + b);
            }
            Instruction::Multiply => {
                let a = self.f.lp_stack.pop().unwrap_or(0);
                let b = self.f.lp_stack.pop().unwrap_or(0);
                self.f.lp_stack.push(a * b);
            }
            Instruction::Subtract => {
                let a = self.f.lp_stack.pop().unwrap_or(0);
                let b = self.f.lp_stack.pop().unwrap_or(0);
                self.f.lp_stack.push(b - a);
            }
            Instruction::Divide => {
                let a = self.f.lp_stack.pop().unwrap_or(0);
                let b = self.f.lp_stack.pop().unwrap_or(0);
                if a == 0 {
                    self.f.lp_stack.push(b);
                } else {
                    self.f.lp_stack.push(b / a);
                }
            }
            Instruction::Modulo => {
                let a = self.f.lp_stack.pop().unwrap_or(0);
                let b = self.f.lp_stack.pop().unwrap_or(0);
                if a == 0 {
                    self.f.lp_stack.push(b);
                } else {
                    self.f.lp_stack.push(b % a);
                }
            }
            Instruction::OutputNum => {
                self.out
                    .push(self.f.lp_stack.pop().unwrap_or(0).to_string());
            }
            Instruction::OutputASCII => {
                self.out
                    .push((self.f.lp_stack.pop().unwrap_or(0) as u8 as char).to_string());
            }
            Instruction::Halt => {
                self.save_snapshot(ins);
                return false;
            }
        }
        self.save_snapshot(ins);
        true
    }
    pub fn run(&mut self, steps_limit: usize, verbose: bool) {
        let mut running = true;
        while self.steps < steps_limit {
            running = self.step();
            self.steps += 1;
            if !running {
                self.state = crate::ExecutionState::Halted;
                break;
            }
            if verbose {
                println!(
                    "x: {} y: {} dx: {} dy: {}",
                    self.f.x, self.f.y, self.f.dx, self.f.dy
                );
                to_outpt(&self.f, &self.p);
            }
        }
        if (self.steps >= steps_limit && running) {
            self.state = crate::ExecutionState::StepLimitReached;
        }
        for i in &self.out {
            print!("{}", i);
        }
    }
}
