pub struct Pond {
    p_grid: Vec<char>,
    row: usize,
    col: usize,
}
pub struct Frog {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

pub fn hop(f: &mut Frog, p: &Pond) -> () {
    f.x += f.dx;
    if (f.x >= p.col as isize) {
        f.x = p.col as isize - 1;
    } else if (f.x < 0) {
        f.x = 0;
    }
    f.y += f.dy;
    if (f.y >= p.row as isize) {
        f.y = p.row as isize - 1;
    } else if (f.y < 0) {
        f.y = 0;
    }
}

pub fn to_idx(f: &Frog, p: &Pond) -> usize {
    f.y as usize * p.col + f.x as usize
}

pub fn step(f: &mut Frog, p: &Pond) -> bool {
    let ctile: char = p.p_grid[to_idx(f, p)];
    if (ctile == '.') {
    } else if (ctile == '+') {
        let old_dx = f.dx;
        f.dx = -f.dy;
        f.dy = old_dx;
    } else if (ctile == '@') {
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
        let inp: String = ".+....++.+...@.+".to_string();
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
        };
        while step(&mut nf, &np) {
            println!("x: {} y: {} dx: {} dy: {}", nf.x, nf.y, nf.dx, nf.dy);
            to_outpt(&nf, &np);
        }

        assert_eq!(parse_str_vec(inp.as_str()).len(), row * col);
    }
}
