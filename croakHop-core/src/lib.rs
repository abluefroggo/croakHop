pub mod frog;
pub mod interpreter;

enum ExecutionState {
    Running,
    Halted,
    Error(String),
    StepLimitReached,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Instruction {
    Empty,
    Turn,
    Up,
    Down,
    Left,
    Right,
    Accelerate,
    Duplicate,
    Push(i64),
    Swap,
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    OutputNum,
    OutputASCII,
    Halt,
}

impl Instruction {
    pub fn from_char(ctile: char) -> Instruction {
        if ctile.is_ascii_digit() {
            Instruction::Push(ctile as i64 - 48)
        } else {
            match ctile {
                '.' => Instruction::Empty,
                '@' => Instruction::Turn,
                '^' => Instruction::Up,
                'v' => Instruction::Down,
                '<' => Instruction::Left,
                '>' => Instruction::Right,
                'c' => Instruction::Accelerate,
                ':' => Instruction::Duplicate,
                's' => Instruction::Swap,
                'x' => Instruction::Pop,
                '+' => Instruction::Add,
                '-' => Instruction::Subtract,
                '*' => Instruction::Multiply,
                '/' => Instruction::Divide,
                '%' => Instruction::Modulo,
                'p' => Instruction::OutputNum,
                'a' => Instruction::OutputASCII,
                '#' => Instruction::Halt,
                _ => Instruction::Empty,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExecutionSnapshot {
    pub step: usize,
    pub x: isize,
    pub y: isize,

    pub dx: isize,
    pub dy: isize,

    pub stack: Vec<i64>,
    pub output: Vec<String>,
    pub instruction: Instruction,
}
pub struct Interpreter {
    pub p: Pond,
    pub f: Frog,
    pub out: Vec<String>,
    pub steps: usize,
    pub state: ExecutionState,
    pub history: Vec<ExecutionSnapshot>,
}

pub struct Pond {
    p_grid: Vec<char>,
    row: usize,
    col: usize,
}
pub struct Frog {
    pub x: isize,
    pub y: isize,
    pub dx: isize,
    pub dy: isize,
    pub lp_stack: Vec<i64>,
}

pub fn parse_str_vec(inp: &str) -> Vec<char> {
    let mut out: Vec<char> = vec![];
    for i in inp.chars() {
        out.push(i);
    }
    out
}

pub fn parse_input(inp: &str) -> Pond {
    let lines: Vec<&str> = inp.lines().filter(|l| !l.trim().is_empty()).collect();
    let r: usize = lines.len();
    let c: usize = lines.first().map_or(0, |l| l.chars().count());

    let mut out: Vec<char> = vec![];
    for l in lines {
        for ch in l.chars() {
            out.push(ch);
        }
    }
    Pond {
        p_grid: out,
        row: r,
        col: c,
    }
}

pub fn to_idx(f: &Frog, p: &Pond) -> usize {
    f.y as usize * p.col + f.x as usize
}

pub fn hop(f: &mut Frog, p: &Pond) -> () {
    f.x += f.dx;
    if (f.x >= p.col as isize) {
        f.x = p.col as isize - 1;
        f.dx = -f.dx;
    } else if (f.x < 0) {
        f.x = 0;
        f.dx = -f.dx;
    }
    f.y += f.dy;
    if (f.y >= p.row as isize) {
        f.y = p.row as isize - 1;
        f.dy = -f.dy;
    } else if (f.y < 0) {
        f.y = 0;
        f.dy = -f.dy;
    }
}

pub fn step(f: &mut Frog, p: &Pond) -> bool {
    hop(f, p);
    let ctile: char = p.p_grid[to_idx(f, p)];
    if (ctile == '.') {
    } else if (ctile == '@') {
        let old_dx = f.dx;
        f.dx = -f.dy;
        f.dy = old_dx;
    } else if (ctile == '^') {
        f.dx = 0;
        f.dy = -f.dy.abs();
        if f.dy == 0 {
            f.dy = -1;
        }
    } else if (ctile == 'v') {
        f.dx = 0;
        f.dy = f.dy.abs();
        if f.dy == 0 {
            f.dy = 1;
        }
    } else if (ctile == '<') {
        f.dx = -f.dx.abs();
        f.dy = 0;
        if f.dx == 0 {
            f.dx = -1;
        }
    } else if (ctile == '>') {
        f.dx = f.dx.abs();
        f.dy = 0;
        if f.dx == 0 {
            f.dx = 1;
        }
    } else if (ctile == 'c') {
        f.dx *= 2;
        f.dy *= 2;
    } else if (ctile == ':') {
        let t = f.lp_stack.last().copied().unwrap_or(0);
        f.lp_stack.push(t);
    } else if (ctile.is_ascii_digit()) {
        f.lp_stack.push(ctile as i64 - 48);
    } else if (ctile == 's') {
        if f.lp_stack.len() > 1 {
            let idxa = f.lp_stack.len() - 1;
            let idxb = idxa - 1;
            let temp = f.lp_stack[idxb];
            f.lp_stack[idxb] = f.lp_stack[idxa];
            f.lp_stack[idxa] = temp;
        }
    } else if (ctile == 'x') {
        f.lp_stack.pop();
    } else if (ctile == '+') {
        let a = f.lp_stack.pop().unwrap_or(0);
        let b = f.lp_stack.pop().unwrap_or(0);
        f.lp_stack.push(a + b);
    } else if (ctile == '*') {
        let a = f.lp_stack.pop().unwrap_or(0);
        let b = f.lp_stack.pop().unwrap_or(0);
        f.lp_stack.push(a * b);
    } else if (ctile == '-') {
        let a = f.lp_stack.pop().unwrap_or(0);
        let b = f.lp_stack.pop().unwrap_or(0);
        f.lp_stack.push(b - a);
    } else if (ctile == '/') {
        let a = f.lp_stack.pop().unwrap_or(0);
        let b = f.lp_stack.pop().unwrap_or(0);
        if a == 0 {
            f.lp_stack.push(b);
        } else {
            f.lp_stack.push(b / a);
        }
    } else if (ctile == '%') {
        let a = f.lp_stack.pop().unwrap_or(0);
        let b = f.lp_stack.pop().unwrap_or(0);
        if a == 0 {
            f.lp_stack.push(b);
        } else {
            f.lp_stack.push(b % a);
        }
    } else if (ctile == 'p') {
        print!("{}", f.lp_stack.pop().unwrap_or(0));
    } else if (ctile == 'a') {
        print!("{}", f.lp_stack.pop().unwrap_or(0) as u8 as char);
    } else if (ctile == '#') {
        return false;
    }
    true
}

pub fn to_outpt(f: &Frog, p: &Pond) -> () {
    for i in 0..p.row {
        for j in 0..p.col {
            if (f.x == j as isize && f.y == i as isize) {
                print!("o")
            } else {
                print!("{}", p.p_grid[i * p.col + j]);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let row: usize = 4;
        let col: usize = 4;
        let inp: String = ".+....++.+.....+".to_string();
        let mut np: Pond = Pond {
            p_grid: parse_str_vec(inp.as_str()),
            row: row,
            col: col,
        };
        let mut nf: Frog = Frog {
            x: 0,
            y: 0,
            dx: 1,
            dy: 1,
            lp_stack: vec![],
        };
        let mut steps: usize = 0;
        while step(&mut nf, &np) && steps < 100 {
            println!("x: {} y: {} dx: {} dy: {}", nf.x, nf.y, nf.dx, nf.dy);
            to_outpt(&nf, &np);
            steps += 1;
        }

        assert_eq!(parse_str_vec(inp.as_str()).len(), row * col);
    }
}
