use crate::*;

impl Interpreter {
    pub fn new(p: Pond) -> Self {
        let nf: Frog = Frog {
            x: 0,
            y: 0,
            dx: 1,
            dy: 1,
            lp_stack: vec![],
        };
        Self {
            p: p,
            f: nf,
            out: vec![],
            steps: 0,
            state: String::from("Running"),
        }
    }
    pub fn step(&mut self) -> bool {
        crate::step(&mut self.f, &self.p)
    }
    pub fn run(&mut self, steps_limit: usize) {
        while self.steps < steps_limit {
            if !self.step() {
                self.state = String::from("Finished");
                break;
            }

            println!(
                "x: {} y: {} dx: {} dy: {}",
                self.f.x, self.f.y, self.f.dx, self.f.dy
            );
            to_outpt(&self.f, &self.p);
            self.steps += 1;
        }
    }
}
