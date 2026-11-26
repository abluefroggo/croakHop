use std::env;
use std::fs;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("{}", path);
    let content = fs::read_to_string(path).expect("File read error");
    println!("{content}");
}
