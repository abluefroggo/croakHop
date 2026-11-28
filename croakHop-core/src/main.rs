use croakhop_core::*;
use std::env;
use std::fs;

fn main() -> () {
    let mut verbose = false;
    let mut args: Vec<String> = env::args().collect();

    for arg in &args {
        if arg.as_str() == "--verbose" {
            verbose = true;
        }
    }

    args.retain(|a| a != "--verbose");

    let path = args.get(1).expect(".frog file not specified");
    let mut steps_lim: usize = 100;
    if args.len() > 2 {
        steps_lim = args[2]
            .parse()
            .expect("steps limit must be a positive integer");
    }
    println!("{}", path);
    let content = fs::read_to_string(path).expect("File read error");
    println!("{content}");
    let pond = parse_input(&content);
    let mut interpreter = Interpreter::new(pond);
    interpreter.run(steps_lim, verbose);
}
