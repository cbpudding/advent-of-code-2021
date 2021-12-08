#![feature(int_abs_diff)]

use std::{
    cmp, env,
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_cost(crabs: &Vec<usize>, x: usize) -> usize {
    crabs
        .iter()
        .map(|c| (1usize..=c.abs_diff(x)).sum::<usize>())
        .sum()
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
        let max = *crabs.iter().max().unwrap();
        let mut best = usize::MAX;
        for x in 0..max {
            best = cmp::min(best, calculate_cost(&crabs, x));
        }
        println!("Cheapest possible outcome: {}", best);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
