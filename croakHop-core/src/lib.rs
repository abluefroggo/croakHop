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

pub fn to_idx(f: &Frog, p: &Pond) -> usize {
    f.y as usize * p.col + f.x as usize
}

pub fn step(f: &mut Frog, p: &Pond) -> bool {
    let ctile: char = p.p_grid[to_idx(f, p)];
    if (ctile == '.') {
    } else if (ctile == '@') {
        let old_dx = f.dx;
        f.dx = -f.dy;
        f.dy = old_dx;
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
    } else if (ctile == 'p') {
        println!("{}", f.lp_stack.pop().unwrap_or(0));
    } else if (ctile == '#') {
        return false;
    }
    hop(f, p);
    true
}

pub fn parse_str_vec(inp: &str) -> Vec<char> {
    let mut out: Vec<char> = vec![];
    for i in inp.chars() {
        out.push(i);
    }
    out
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
