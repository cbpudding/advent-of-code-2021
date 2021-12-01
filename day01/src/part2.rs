use std::{
    collections::VecDeque,
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
        let mut window = VecDeque::new();
        for line in reader.lines() {
            let measurement = line.unwrap().parse::<usize>().unwrap();
            window.push_front(measurement);
            if window.len() == 3 {
                let sum = window.iter().fold(0, |a, x| a + x);
                if let Some(prev) = depth {
                    if sum > prev {
                        increases += 1;
                    }
                }
                depth = Some(sum);
                window.pop_back();
            }
        }
        println!("Increases: {}", increases);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
