use crate::*;

impl Interpreter {
    pub fn new(p: Pond) -> Self {
        let nf: Frog = Frog::new();
        let mut interpreter = Self {
            p: p,
            f: nf,
            out: vec![],
            steps: 0,
            state: crate::ExecutionState::Running,
            history: vec![],
        };
        interpreter.save_snapshot('.');
        interpreter
    }

    pub fn save_snapshot(&mut self, ins: char) {
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
        crate::step(&mut self.f, &self.p)
    }
    pub fn run(&mut self, steps_limit: usize, verbose: bool) {
        while self.steps < steps_limit {
            if !self.step() {
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
            self.steps += 1;
        }
        if (self.steps >= steps_limit) {
            self.state = crate::ExecutionState::StepLimitReached;
        }
    }
}
