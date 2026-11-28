use crate::*;

impl Interpreter {
    pub fn new(p: Pond) -> Self {
        let nf: Frog = Frog::new();
        Self {
            p: p,
            f: nf,
            out: vec![],
            steps: 0,
            state: crate::ExecutionState::Running,
        }
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
        if (self.steps > steps_limit) {
            self.state = crate::ExecutionState::StepLimitReached;
        }
    }
}
