use croakhop_core::*;
use std::env;
use std::fs;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("{}", path);
    let content = fs::read_to_string(path).expect("File read error");
    println!("{content}");
    let mut nf: Frog = Frog {
        x: 0,
        y: 0,
        dx: 4,
        dy: 1,
        lp_stack: vec![],
    };
    let np = parse_input(content.as_str());
    let mut steps: usize = 0;
    while step(&mut nf, &np) && steps < 100 {
        println!("x: {} y: {} dx: {} dy: {}", nf.x, nf.y, nf.dx, nf.dy);
        to_outpt(&nf, &np);
        steps += 1;
    }
}
