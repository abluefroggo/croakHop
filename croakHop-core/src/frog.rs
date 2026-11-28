use crate::*;

impl Frog {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
            lp_stack: vec![],
        }
    }
}
