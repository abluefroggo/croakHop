use croakhop_core::*;
use std::env;
use std::fs;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let steps_lim: usize = args[2]
        .parse()
        .expect("steps limit must be a positive integer");
    println!("{}", path);
    let content = fs::read_to_string(path).expect("File read error");
    println!("{content}");
    let pond = parse_input(&content);
    let mut interpreter = Interpreter::new(pond);
    interpreter.run(steps_lim)
}
