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
        let data: Vec<Vec<u8>> = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .split(" ")
                    .filter(|&c| c != "|")
                    .map(|c| c.chars().map(|d| 1 << (d as u8 - 97)).sum())
                    .collect()
            })
            .collect();
        let count: usize = data
            .iter()
            .map(|l| {
                l[10..]
                    .iter()
                    .map(|c| c.count_ones())
                    .filter(|c| [2, 3, 4, 7].contains(c))
                    .count()
            })
            .sum();
        println!("Unique digit count: {}", count);
    } else {
        eprintln!("Which file am I trying to open?");
    }
}
