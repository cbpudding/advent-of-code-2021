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
        let mut total = 0;
        let mut count = Vec::new();
        for raw in reader.lines() {
            let line = raw.unwrap();
            let data = usize::from_str_radix(&line, 2).unwrap();
            if count.len() == 0 {
                count = vec![0; line.len()];
            }
            for bit in 0..count.len() {
                let mask = 1 << bit;
                if data & mask != 0 {
                    count[bit] += 1;
                }
            }
            total += 1;
        }
        let mut gamma = 0;
        for bit in 0..count.len() {
            if count[bit] > total - count[bit] {
                gamma |= 1 << bit;
            }
        }
        let epsilon = !gamma & (1 << count.len()) - 1;
        println!("Gamma: {}", gamma);
        println!("Epsilon: {}", epsilon);
        println!("Power consumption: {}", gamma * epsilon);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
