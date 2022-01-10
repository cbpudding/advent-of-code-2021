use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        // ...
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
