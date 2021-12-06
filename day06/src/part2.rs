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
        let data: Vec<usize> = reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        let mut fishes = vec![0; 9];
        for f in data {
            fishes[f] += 1;
        }
        for _ in 0..256 {
            let mut newfish = 0;
            for stage in 0..8 {
                if stage == 0 {
                    fishes[7] += fishes[0];
                    newfish += fishes[0];
                }
                fishes[stage] = fishes[stage + 1];
            }
            fishes[8] = newfish;
        }
        let count: usize = fishes.iter().sum();
        println!("Lanternfish: {}", count);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
