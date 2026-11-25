struct Pond {
    p_grid: Vec<char>,
    row: usize,
    col: usize,
}
struct Frog {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

pub fn parse_str_vec(inp: &str) -> Vec<char> {
    let mut out: Vec<char> = vec![];
    for i in inp.chars() {
        out.push(i);
    }
    out
}
pub fn to_outpt(inp: Vec<char>, row: usize, col: usize) -> () {
    for i in 0..row {
        for j in 0..col {
            print!("{}", inp[i * col + j]);
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
        let inp: String = "......++.+......".to_string();
        println!("Hello world");
        to_outpt(parse_str_vec(inp.as_str()), row, col);
        assert_eq!(parse_str_vec(inp.as_str()).len(), row * col);
    }
}
