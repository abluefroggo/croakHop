pub fn parseStrVec(inp: &str) -> Vec<char> {
    let mut out: Vec<char> = vec![];
    for i in inp.chars() {
        out.push(i);
    }
    out
}
pub fn toOutpt(inp: Vec<char>, row: usize, col: usize) -> () {
    for i in 0..row {
        for j in 0..col {
            print!("{}", inp[i * row + j]);
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
        let inp: String = "......+..+......".to_string();
        println!("Hello world");
        toOutpt(parseStrVec(inp.as_str()), row, col);
        assert_eq!(parseStrVec(inp.as_str()).len(), row * col);
    }
}
