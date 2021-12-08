#![feature(int_abs_diff)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_cost(crabs: &Vec<usize>, x: usize) -> usize {
    crabs.iter().map(|c| c.abs_diff(x)).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = File::open(&args[1]).unwrap();
        let reader = BufReader::new(input);
        let crabs: Vec<usize> = reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect();
        let mut best = usize::MAX;
        let mut last = usize::MAX - 1;
        let mut x = 0;
        // Rational people think marginally(Right?)
        while last < best {
            best = last;
            last = calculate_cost(&crabs, x);
            x += 1;
        }
        println!("Cheapest possible outcome: {}", best);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
