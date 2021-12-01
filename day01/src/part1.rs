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
        let mut depth = None;
        let mut increases = 0;
        for line in reader.lines() {
            let measurement = line.unwrap().parse::<usize>().unwrap();
            if let Some(prev) = depth {
                if measurement > prev {
                    increases += 1;
                }
            }
            depth = Some(measurement);
        }
        println!("Increases: {}", increases);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
