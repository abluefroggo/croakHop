pub mod frog;
pub mod interpreter;

pub enum ExecutionState {
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
    sx: usize,
    sy: usize,
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
    let mut sx = 0;
    let mut sy = 0;
    let mut ln = 0;
    let mut cn = 0;
    let mut out: Vec<char> = vec![];
    for l in lines {
        cn = 0;
        for ch in l.chars() {
            out.push(ch);
            if (ch == 'o') {
                sx = cn % c;
                sy = ln % r;
            }
            cn += 1;
            assert!(cn <= c, "Row lengths dont match up.");
        }
        assert_eq!(cn, c, "Row lengths dont match up");
        ln += 1;
    }
    Pond {
        p_grid: out,
        row: r,
        col: c,
        sx: sx,
        sy: sy,
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
